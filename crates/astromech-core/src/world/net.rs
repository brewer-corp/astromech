//! The world's network: every HTTP request the core makes, answered from the world's `net.json`
//! ([`astromech_data::replay`], which has the format), so none reaches the internet.
//!
//! - In process: the remote-data client's transport, EDDN's upload transport and EDSM's API are
//!   the replay.
//! - Over 127.0.0.1: the downloads (the update feed and zip, the speech models and the neutron
//!   star file) fetch a URL themselves, so [`Http::downloads`] sends each to the replay server,
//!   which answers it as its real URL.
//!
//! Every request is observed as `http {method, url, unrecorded}`, and one the recording doesn't
//! hold is refused as a connection failure.
//!
//! Record mode (`--record-net <file>`): the remote-data client's requests and EDSM's reads
//! (its `GET`s: the flight log, the discard list) go to the real services, and their answers are
//! written to `<file>` in `net.json`'s form, redacted. EDSM's journal upload (a `POST` of the
//! world's journal as the commander) and EDDN's upload are still answered from the recording,
//! so record mode never publishes a world's journal. The downloads aren't recorded: worlds give
//! them statuses and faults. The recording client runs on the system clock with the default
//! request spacing and Spansh polling, so the real services are asked at their usual pace.

use super::Observed;
use crate::app::Http;
use anyhow::{Context, Result};
use astromech_data::replay::{self, Answered, Asked, Fault, Outcome, Recording, Replay, Seen, Server, Tape};
use astromech_eddn::edsm::api::{Api, HttpApi};
use astromech_proto::ManualClock;
use serde_json::json;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

/// A world's recording, its server, and in record mode the tape being written.
pub struct Net {
    replay: Arc<Replay>,
    server: Arc<Server>,
    tape: Option<Arc<Tape>>,
}

/// Each request, as the world observes it.
fn observe(o: &Arc<Observed>) -> impl Fn(&Seen) + Send + Sync + 'static {
    let o = o.clone();
    move |s: &Seen| o.record("http", json!({ "method": s.method, "url": s.url, "unrecorded": s.unrecorded }))
}

impl Net {
    /// The recording in the world at `src` (none: every request is refused), and its server.
    /// `record`: record mode's file.
    pub fn load(src: &Path, record: Option<&Path>, observed: &Arc<Observed>) -> Result<Self> {
        let path = src.join(replay::FILE);
        let recording =
            if path.is_file() { Recording::load(&path).map_err(anyhow::Error::msg)? } else { Recording::default() };
        let replay = Arc::new(Replay::new(recording).observe(observe(observed)));
        let server = Arc::new(Server::start(replay.clone()).context("starting the network replay server")?);
        let tape = match record {
            Some(p) => Some(Arc::new(
                Tape::create(p).with_context(|| format!("creating {}", p.display()))?.observe(observe(observed)),
            )),
            None => None,
        };
        Ok(Self { replay, server, tape })
    }

    /// The recording a `net` step adds to.
    pub fn replay(&self) -> &Arc<Replay> {
        &self.replay
    }

    /// Record mode is on.
    pub fn recording(&self) -> bool {
        self.tape.is_some()
    }

    /// The remote-data client: on the replay, with no request spacing and on the world's clock,
    /// so a lookup never waits for the clock to move; in record mode, real HTTP recorded to the
    /// tape, on the system clock with the default spacing.
    pub fn remote(&self, clock: Arc<ManualClock>) -> Arc<astromech_data::Client> {
        let cache = astromech_data::Cache::in_memory().expect("an in-memory cache");
        let log = Arc::new(|s: &str| crate::logging::log(s));
        Arc::new(match &self.tape {
            None => astromech_data::Client::with_transport(cache, self.replay.clone())
                .limiter(astromech_data::Limiter::none())
                .clock(clock)
                .log(log),
            Some(tape) => {
                let real = Arc::new(astromech_data::HttpTransport::new());
                astromech_data::Client::with_transport(cache, Arc::new(replay::Recorder::new(real, tape.clone())))
                    .log(log)
            }
        })
    }

    /// EDDN's upload and EDSM's API on the replay (EDSM's reads recorded in record mode), and
    /// every download sent to the replay server.
    pub fn http(&self) -> Http {
        let (eddn, edsm, tape, server) =
            (self.replay.clone(), self.replay.clone(), self.tape.clone(), self.server.clone());
        Http {
            eddn: Arc::new(move || Box::new(EddnReplay(eddn.clone()))),
            edsm: Arc::new(move |timeout| match &tape {
                None => Box::new(EdsmReplay { replay: edsm.clone(), timeout }),
                Some(tape) => Box::new(EdsmRecorder {
                    real: HttpApi::with_timeout(&format!("astromech/{}", env!("CARGO_PKG_VERSION")), timeout),
                    tape: tape.clone(),
                    replay: EdsmReplay { replay: edsm.clone(), timeout },
                }),
            }),
            downloads: Arc::new(move |url: &str| server.url(url)),
        }
    }
}

/// EDDN's gateway, answered from the recording by its URL.
struct EddnReplay(Arc<Replay>);

impl astromech_eddn::Transport for EddnReplay {
    fn post(&self, _gzipped_json: &[u8]) -> Result<astromech_eddn::Response, String> {
        let a = Asked::post_bytes(astromech_eddn::UPLOAD_URL);
        match self.0.answer(&a) {
            Answered::Reply { status, body } => Ok(astromech_eddn::Response {
                status,
                body: astromech_eddn::transport::clip(
                    &String::from_utf8_lossy(&body),
                    astromech_eddn::transport::MAX_BODY_TEXT,
                )
                .to_owned(),
                headers: vec![],
            }),
            Answered::Timeout => Err("timed out (this world's recording)".into()),
            Answered::Offline => Err("connection failed (this world's recording)".into()),
            Answered::Unrecorded => Err(replay::unrecorded(&a.url)),
        }
    }
}

/// EDSM's API, answered from the recording. A timeout is the error EDSM's own API gives.
struct EdsmReplay {
    replay: Arc<Replay>,
    timeout: Duration,
}

impl EdsmReplay {
    fn answer(&self, a: &Asked) -> Result<astromech_eddn::Response, String> {
        match self.replay.answer(a) {
            Answered::Reply { status, body } => Ok(astromech_eddn::Response {
                status,
                body: String::from_utf8_lossy(&body).into_owned(),
                headers: vec![],
            }),
            Answered::Timeout => Err(format!("EDSM didn't answer within {} s", self.timeout.as_secs())),
            Answered::Offline => Err("connection failed (this world's recording)".into()),
            Answered::Unrecorded => Err(replay::unrecorded(&a.url)),
        }
    }
}

impl Api for EdsmReplay {
    fn get(&self, url: &str, query: &[(&str, &str)]) -> Result<astromech_eddn::Response, String> {
        self.answer(&Asked::get(url, query))
    }

    fn post_json(&self, url: &str, body: &[u8]) -> Result<astromech_eddn::Response, String> {
        self.answer(&Asked::post_json(url, body))
    }
}

/// Record mode's EDSM API: reads go to EDSM and are recorded; the journal upload is answered
/// from the recording.
struct EdsmRecorder {
    real: HttpApi,
    tape: Arc<Tape>,
    replay: EdsmReplay,
}

impl Api for EdsmRecorder {
    fn get(&self, url: &str, query: &[(&str, &str)]) -> Result<astromech_eddn::Response, String> {
        let r = self.real.get(url, query);
        let outcome = match &r {
            Ok(resp) => Outcome::Reply { status: resp.status, body: &resp.body },
            Err(e) if e.contains("didn't answer within") => Outcome::Fault(Fault::Timeout),
            Err(_) => Outcome::Fault(Fault::Offline),
        };
        self.tape.record(&Asked::get(url, query), outcome).map_err(|e| format!("couldn't write the recording: {e}"))?;
        r
    }

    fn post_json(&self, url: &str, body: &[u8]) -> Result<astromech_eddn::Response, String> {
        self.replay.post_json(url, body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader, Write};

    /// Answers each request on 127.0.0.1 with `body`, and keeps the request lines it got.
    fn edsm_stand_in(body: &'static str) -> (String, Arc<std::sync::Mutex<Vec<String>>>) {
        let l = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let base = format!("http://{}", l.local_addr().unwrap());
        let got = Arc::new(std::sync::Mutex::new(vec![]));
        let g = got.clone();
        std::thread::spawn(move || {
            for s in l.incoming() {
                let Ok(mut s) = s else { continue };
                let mut r = BufReader::new(s.try_clone().unwrap());
                let mut line = String::new();
                r.read_line(&mut line).unwrap();
                g.lock().unwrap().push(line.clone());
                while r.read_line(&mut line).is_ok_and(|n| n > 2) {
                    line.clear();
                }
                let _ =
                    write!(s, "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len());
            }
        });
        (base, got)
    }

    /// Record mode records a flight-log read with the commander's name and key redacted, and the
    /// recording replays it to the same flight log for any commander.
    #[test]
    fn a_recorded_flight_log_holds_no_key_or_commander_and_replays_the_same() {
        use astromech_eddn::edsm::{api::flight_log, Credentials};
        const ANSWER: &str = r#"{"msgnum":100,"msg":"OK","commanderName":"Jameson","logs":[{"shipId":7,"system":"Sol","firstDiscover":false,"date":"2026-10-16 20:00:00"}]}"#;
        let (base, got) = edsm_stand_in(ANSWER);
        let d = tempfile::tempdir().unwrap();
        let file = d.path().join("recorded.json");
        let observed = Arc::new(Observed::default());
        let rebased = |net: &Net| RebasedApi { api: (net.http().edsm)(Duration::from_secs(10)), to: base.clone() };
        let jameson = Credentials { commander: "Jameson".into(), api_key: "k3y".into() };

        let net = Net::load(d.path(), Some(&file), &observed).unwrap();
        let live = flight_log(&rebased(&net), &jameson).unwrap();
        assert_eq!(live[0].system, "Sol");
        let sent = got.lock().unwrap().clone();
        assert!(
            sent[0].contains("commanderName=Jameson") && sent[0].contains("apiKey=k3y"),
            "it really asked: {sent:?}"
        );
        let text = std::fs::read_to_string(&file).unwrap();
        assert!(!text.contains("k3y") && !text.contains("Jameson"), "{text}");
        assert!(text.contains("/api-logs-v1/get-logs?commanderName=<redacted>&apiKey=<redacted>"), "{text}");

        // Replayed from a world whose net.json is that recording, with another commander's key.
        std::fs::copy(&file, d.path().join(replay::FILE)).unwrap();
        let replayed = Net::load(d.path(), None, &observed).unwrap();
        let other = Credentials { commander: "Other".into(), api_key: "else".into() };
        assert_eq!(flight_log(&rebased(&replayed), &other).unwrap(), live);
        assert_eq!(got.lock().unwrap().len(), 1, "the replay asked nothing");
        let (seen, _) = observed.since(0);
        assert_eq!(seen.len(), 2);
        assert!(seen.iter().all(|o| o["unrecorded"] == false), "{seen:?}");
        assert!(seen.iter().all(|o| !o.to_string().contains("k3y") && !o.to_string().contains("Jameson")), "{seen:?}");
    }

    /// The flight log asks EDSM's own URL; this sends it to the stand-in's instead.
    struct RebasedApi {
        api: Box<dyn Api>,
        to: String,
    }

    impl Api for RebasedApi {
        fn get(&self, url: &str, query: &[(&str, &str)]) -> Result<astromech_eddn::Response, String> {
            self.api.get(&url.replace("https://www.edsm.net", &self.to), query)
        }
        fn post_json(&self, url: &str, body: &[u8]) -> Result<astromech_eddn::Response, String> {
            self.api.post_json(&url.replace("https://www.edsm.net", &self.to), body)
        }
    }
}
