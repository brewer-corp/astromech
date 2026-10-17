//! EDSM's HTTP endpoints, behind a trait so tests never reach the real site. Written from
//! EDSM's API docs: Journal API v1 (<https://www.edsm.net/en/api-journal-v1>) and Logs API v1
//! (<https://www.edsm.net/en/api-logs-v1>, `get-logs`).

use super::Credentials;
use crate::transport::Response;
use serde::Serialize;
use serde_json::Value;
use std::io::Read;
use std::time::Duration;

/// `POST`: journal events (one, or a list) with the commander's name and API key.
pub const JOURNAL_URL: &str = "https://www.edsm.net/api-journal-v1";
/// `GET`: the events EDSM won't process (answer 304 if sent anyway).
pub const DISCARD_URL: &str = "https://www.edsm.net/api-journal-v1/discard";
/// `GET`: the commander's flight log (one week at most; 360 requests an hour per user).
pub const LOGS_URL: &str = "https://www.edsm.net/api-logs-v1/get-logs";

/// How long a request may take overall. The flight log is read while the app window waits on
/// the core, which it does for 15 s at most (`call_with` in the shell), so it must give up
/// sooner for the page to show our own message.
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
pub const FLIGHT_LOG_TIMEOUT: Duration = Duration::from_secs(10);

/// Answers longer than this are cut off (the discard list is a few KB; a week of flight log
/// well under this).
const MAX_ANSWER: u64 = 4 * 1024 * 1024;

pub trait Api: Send {
    /// `GET url?query`. `Err` is a failure with no HTTP status (refused, timed out, TLS).
    fn get(&self, url: &str, query: &[(&str, &str)]) -> Result<Response, String>;
    /// `POST` a JSON body (EDSM takes the parameters "as a raw body JSON").
    fn post_json(&self, url: &str, body: &[u8]) -> Result<Response, String>;
}

pub struct HttpApi {
    agent: ureq::Agent,
    user_agent: String,
    timeout: Duration,
}

impl HttpApi {
    pub fn new(user_agent: &str) -> Self {
        Self::with_timeout(user_agent, REQUEST_TIMEOUT)
    }

    /// Every request gives up after `timeout` overall.
    pub fn with_timeout(user_agent: &str, timeout: Duration) -> Self {
        Self { agent: ureq::AgentBuilder::new().timeout(timeout).build(), user_agent: user_agent.into(), timeout }
    }

    fn answer(&self, r: Result<ureq::Response, ureq::Error>) -> Result<Response, String> {
        let resp = match r {
            Ok(r) => r,
            Err(ureq::Error::Status(_, r)) => r,
            Err(ureq::Error::Transport(t)) => return Err(transport_error(&t, self.timeout)),
        };
        let status = resp.status();
        let mut body = String::new();
        resp.into_reader().take(MAX_ANSWER).read_to_string(&mut body).map_err(|e| e.to_string())?;
        Ok(Response { status, body })
    }
}

/// What went wrong, without the URL: ureq's own text starts with the URL, and a `GET` to EDSM
/// carries the API key in its query.
fn transport_error(t: &ureq::Transport, timeout: Duration) -> String {
    let mut e = t.kind().to_string();
    let mut cause = std::error::Error::source(t);
    let mut timed_out = false;
    while let Some(c) = cause {
        timed_out |= c
            .downcast_ref::<std::io::Error>()
            .is_some_and(|io| matches!(io.kind(), std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock));
        e = format!("{e}: {c}");
        cause = c.source();
    }
    if timed_out {
        return format!("EDSM didn't answer within {} s", timeout.as_secs());
    }
    e
}

impl Api for HttpApi {
    fn get(&self, url: &str, query: &[(&str, &str)]) -> Result<Response, String> {
        let mut req = self.agent.get(url).set("User-Agent", &self.user_agent);
        for (k, v) in query {
            req = req.query(k, v);
        }
        self.answer(req.call())
    }

    fn post_json(&self, url: &str, body: &[u8]) -> Result<Response, String> {
        let req = self.agent.post(url).set("User-Agent", &self.user_agent).set("Content-Type", "application/json");
        self.answer(req.send_bytes(body))
    }
}

/// EDSM's answer as JSON, when it is JSON.
pub(crate) fn json_answer(r: &Response) -> Result<Value, String> {
    if r.status != 200 {
        return Err(format!("HTTP {}: {}", r.status, crate::transport::clip(r.body.trim(), 200)));
    }
    serde_json::from_str(&r.body)
        .map_err(|_| format!("unreadable answer: {}", crate::transport::clip(r.body.trim(), 200)))
}

/// `msgnum` and `msg` of an answer.
pub(crate) fn code(v: &Value) -> (i64, String) {
    (v["msgnum"].as_i64().unwrap_or(0), v["msg"].as_str().unwrap_or_default().to_owned())
}

/// The event names EDSM discards.
pub fn discard_list(api: &dyn Api) -> Result<Vec<String>, String> {
    let v = json_answer(&api.get(DISCARD_URL, &[])?)?;
    serde_json::from_value(v).map_err(|_| "the discard list isn't a list of event names".to_owned())
}

/// One visit in the flight log.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Visit {
    pub system: String,
    /// UTC, `YYYY-MM-DD HH:MM:SS`.
    pub date: String,
    /// The commander was the first to report the system to EDSM.
    pub first_discover: bool,
    pub ship_id: Option<i64>,
}

/// The commander's flight log on EDSM: the last week, as EDSM gives it without dates.
pub fn flight_log(api: &dyn Api, c: &Credentials) -> Result<Vec<Visit>, String> {
    // Whatever goes wrong is shown in Settings: never with the key in it.
    let hide = |e: String| if c.api_key.is_empty() { e } else { e.replace(&c.api_key, "<API key>") };
    let r = api.get(LOGS_URL, &[("commanderName", &c.commander), ("apiKey", &c.api_key)]).map_err(hide)?;
    let v = json_answer(&r).map_err(hide)?;
    match code(&v) {
        (100, _) => {}
        (n, msg) => return Err(hide(format!("EDSM {n}: {msg}"))),
    }
    let logs = v["logs"].as_array().map(Vec::as_slice).unwrap_or_default();
    Ok(logs
        .iter()
        .map(|l| Visit {
            system: l["system"].as_str().unwrap_or_default().to_owned(),
            date: l["date"].as_str().unwrap_or_default().to_owned(),
            first_discover: l["firstDiscover"].as_bool().unwrap_or(false),
            ship_id: l["shipId"].as_i64(),
        })
        .collect())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    /// One recorded request.
    #[derive(Debug, Clone, PartialEq)]
    pub enum Call {
        Get(String, Vec<(String, String)>),
        Post(String, Value),
    }

    /// Scripted answers by URL; records every request.
    #[derive(Clone, Default)]
    pub struct FakeApi {
        pub answers: Arc<Mutex<Vec<(String, Result<Response, String>)>>>,
        pub calls: Arc<Mutex<Vec<Call>>>,
    }

    impl FakeApi {
        pub fn answer(&self, url: &str, r: Result<Response, String>) -> &Self {
            self.answers.lock().unwrap().push((url.to_owned(), r));
            self
        }
        pub fn ok(&self, url: &str, body: Value) -> &Self {
            self.answer(url, Ok(Response { status: 200, body: body.to_string() }))
        }
        pub fn posts(&self) -> Vec<Value> {
            let calls = self.calls.lock().unwrap();
            calls.iter().filter_map(|c| if let Call::Post(_, b) = c { Some(b.clone()) } else { None }).collect()
        }
        fn next(&self, url: &str) -> Result<Response, String> {
            let mut a = self.answers.lock().unwrap();
            let i = a.iter().position(|(u, _)| u == url).unwrap_or_else(|| panic!("unexpected request to {url}"));
            a.remove(i).1
        }
    }

    impl Api for FakeApi {
        fn get(&self, url: &str, query: &[(&str, &str)]) -> Result<Response, String> {
            let q = query.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
            self.calls.lock().unwrap().push(Call::Get(url.into(), q));
            self.next(url)
        }
        fn post_json(&self, url: &str, body: &[u8]) -> Result<Response, String> {
            self.calls.lock().unwrap().push(Call::Post(url.into(), serde_json::from_slice(body).unwrap()));
            self.next(url)
        }
    }

    fn creds() -> Credentials {
        Credentials { commander: "Jameson".into(), api_key: "k3y".into() }
    }

    #[test]
    fn the_flight_log_is_read_with_the_commanders_key() {
        let api = FakeApi::default();
        api.ok(
            LOGS_URL,
            serde_json::json!({"msgnum":100,"msg":"OK","startDateTime":"2026-10-10 00:00:00","endDateTime":"2026-10-17 00:00:00",
                "logs":[{"shipId":7,"system":"Sol","firstDiscover":false,"date":"2026-10-16 20:00:00"},
                        {"shipId":7,"system":"Achenar","firstDiscover":true,"date":"2026-10-16 19:00:00"}]}),
        );
        let log = flight_log(&api, &creds()).unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(
            log[1],
            Visit {
                system: "Achenar".into(),
                date: "2026-10-16 19:00:00".into(),
                first_discover: true,
                ship_id: Some(7)
            }
        );
        let q = vec![("commanderName".into(), "Jameson".into()), ("apiKey".into(), "k3y".into())];
        assert_eq!(api.calls.lock().unwrap()[0], Call::Get(LOGS_URL.into(), q));
    }

    #[test]
    fn a_flight_log_error_says_what_edsm_said() {
        let api = FakeApi::default();
        api.ok(LOGS_URL, serde_json::json!({"msgnum":203,"msg":"Commander name/API Key not found"}));
        assert_eq!(flight_log(&api, &creds()), Err("EDSM 203: Commander name/API Key not found".into()));
        api.answer(LOGS_URL, Ok(Response { status: 429, body: "slow down".into() }));
        assert_eq!(flight_log(&api, &creds()), Err("HTTP 429: slow down".into()));
        // Anything that quotes the key back has it hidden.
        api.answer(LOGS_URL, Err("https://www.edsm.net/api-logs-v1/get-logs?apiKey=k3y: timed out".into()));
        let e = flight_log(&api, &creds()).unwrap_err();
        assert!(!e.contains("k3y") && e.ends_with("apiKey=<API key>: timed out"), "{e}");
    }

    #[test]
    fn a_failed_flight_log_request_never_shows_the_key() {
        // A closed local port: the connection is refused.
        let l = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/api-logs-v1/get-logs", l.local_addr().unwrap());
        drop(l);
        let api = HttpApi::new("astromech/test");
        let e = api.get(&url, &[("commanderName", "Jameson"), ("apiKey", "SECRETKEY")]).unwrap_err();
        assert!(!e.contains("SECRETKEY") && !e.contains("apiKey"), "{e}");
        assert!(e.to_ascii_lowercase().contains("connection"), "still says what went wrong: {e}");
        let e = api.post_json(&url, br#"{"apiKey":"SECRETKEY"}"#).unwrap_err();
        assert!(!e.contains("SECRETKEY"), "{e}");
    }

    #[test]
    fn a_slow_flight_log_says_edsm_did_not_answer_in_time() {
        // The app window gives up on the core after 15 s: ours must come first.
        assert!(FLIGHT_LOG_TIMEOUT < Duration::from_secs(15));
        // A server that accepts the connection and never answers.
        let l = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/api-logs-v1/get-logs", l.local_addr().unwrap());
        let api = HttpApi::with_timeout("astromech/test", Duration::from_secs(1));
        let e = api.get(&url, &[("commanderName", "Jameson"), ("apiKey", "SECRETKEY")]).unwrap_err();
        assert_eq!(e, "EDSM didn't answer within 1 s");
        drop(l);
    }

    #[test]
    fn http_api_posts_json_and_reads_the_whole_answer() {
        use std::io::{BufRead, BufReader, Write};
        let l = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let base = format!("http://{}", l.local_addr().unwrap());
        let long = "x".repeat(10_000);
        let answer = long.clone();
        let h = std::thread::spawn(move || {
            let (s, _) = l.accept().unwrap();
            let mut r = BufReader::new(s.try_clone().unwrap());
            let mut head = String::new();
            loop {
                let mut line = String::new();
                r.read_line(&mut line).unwrap();
                if line == "\r\n" {
                    break;
                }
                head += &line;
            }
            let len: usize = head
                .lines()
                .find_map(|l| l.to_ascii_lowercase().strip_prefix("content-length:").map(|v| v.trim().parse().unwrap()))
                .unwrap();
            let mut got = vec![0; len];
            r.read_exact(&mut got).unwrap();
            let mut s = s;
            write!(s, "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{answer}", answer.len())
                .unwrap();
            (head, got)
        });
        let r = HttpApi::new("astromech/test").post_json(&format!("{base}/api-journal-v1"), b"{\"a\":1}").unwrap();
        assert_eq!((r.status, r.body.len()), (200, long.len()), "not clipped like EDDN's send log text");
        let (head, body) = h.join().unwrap();
        let head = head.to_ascii_lowercase();
        assert!(head.starts_with("post /api-journal-v1 http/1.1"), "{head}");
        assert!(
            head.contains("content-type: application/json") && head.contains("user-agent: astromech/test"),
            "{head}"
        );
        assert_eq!(body, b"{\"a\":1}");
    }
}
