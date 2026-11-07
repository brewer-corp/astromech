//! A network recording (`net.json`) and its replay (decision astromech-e1a5, "Network"): the
//! answers a world's HTTP requests get, so none reaches the internet. [`Replay`] answers in
//! process, as a [`Transport`] for the remote-data client and through [`Replay::answer`] for the
//! other transports (EDDN's upload, EDSM's API). [`Server`] answers over 127.0.0.1 what takes a
//! URL rather than a transport (the downloads). [`Tape`] and [`Recorder`] write a recording from
//! real answers (record mode), with keys and commander names redacted.
//!
//! # `net.json`
//!
//! `{"exchanges": [ … ]}`, each exchange one JSON object:
//!
//! - `url`: a substring of the request's URL, query included (`…/get-logs?commanderName=…`).
//!   Query keys and values are written percent-encoded as path segments are
//!   ([`crate::encode_segment`]), except a redacted value, which is `<redacted>`;
//! - `method`: optional, `"GET"` or `"POST"` (any case);
//! - `sent`: optional, a substring of the body sent (the JSON text, or a form's `k=v&…`);
//! - the answer:
//!   - `status` (default 200) with a body: `json` (written as it appears), `text`, `from` (a
//!     file, relative to the recording's folder) or none (empty);
//!   - or `fault`: `"timeout"` (in process the transport's timeout at once; over the server a
//!     dropped connection, since a real timeout needs a real wait), `"offline"` (a connection
//!     failure), or `"garbage"` (HTTP 200 with a body that is no format at all);
//!   - or `in_turn`: a list of answers, each a status with a body or a fault, given one per
//!     request in order.
//!
//! The last exchange that matches answers, as in recorded.rs. An `in_turn` exchange that has given
//! all its answers no longer matches, so an earlier one answers the next request. A request no
//! exchange matches is refused (a connection failure) and reported as unrecorded.
//!
//! # Redaction
//!
//! The values of `apiKey`, `commanderName` and any query, form or JSON field named like a key
//! (its name, in any case, holds `key`, `token`, `secret` or `password`) are `<redacted>` in the
//! URL and body a request is matched by, observed as and recorded with, so a recording made with
//! one commander's key replays for any key. A recorded answer has those values replaced too,
//! wherever it echoes them, JSON-escaped (`\uXXXX`, `\/`) or not.

use crate::{encode_segment, Body, Method, Request, Response, Transport, TransportError};
use serde_json::value::RawValue;
use serde_json::{json, Value};
use std::collections::{BTreeMap, VecDeque};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// A world's network recording.
pub const FILE: &str = "net.json";

/// What a redacted value is replaced by.
pub const REDACTED: &str = "<redacted>";

/// The body of a `garbage` fault: no format at all.
pub const GARBAGE: &[u8] = b"\x00\x9f\x92\x96 garbage, not an answer \xff\xfe";

/// Whether a query, form or JSON field holds a secret: `apiKey`, `commanderName`, or a name
/// with `key`, `token`, `secret` or `password` in it.
pub fn secret(name: &str) -> bool {
    let n = name.to_ascii_lowercase();
    n == "commandername" || ["key", "token", "secret", "password"].iter().any(|s| n.contains(s))
}

/// A recorded fault.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fault {
    Timeout,
    Offline,
    Garbage,
}

impl Fault {
    fn name(self) -> &'static str {
        match self {
            Fault::Timeout => "timeout",
            Fault::Offline => "offline",
            Fault::Garbage => "garbage",
        }
    }
}

/// One recorded answer.
#[derive(Debug, Clone, PartialEq)]
pub enum Answer {
    Reply { status: u16, body: Vec<u8> },
    Fault(Fault),
}

#[derive(Debug, Clone, PartialEq)]
enum Answers {
    Always(Answer),
    InTurn(VecDeque<Answer>),
}

/// One exchange of a recording: what it matches and how it answers.
#[derive(Debug, Clone, PartialEq)]
pub struct Exchange {
    url: String,
    method: Option<String>,
    sent: Option<String>,
    answers: Answers,
}

type Fields = BTreeMap<String, Box<RawValue>>;

const ANSWER_FIELDS: [&str; 5] = ["status", "json", "text", "from", "fault"];

fn field<T: serde::de::DeserializeOwned>(f: &Fields, k: &str) -> Result<Option<T>, String> {
    f.get(k).map(|v| serde_json::from_str(v.get()).map_err(|e| format!("`{k}`: {e}"))).transpose()
}

/// A status with a body, or a fault, from `f` (an exchange or an `in_turn` entry).
fn answer(f: &Fields, dir: &Path) -> Result<Answer, String> {
    if let Some(fault) = field::<String>(f, "fault")? {
        if let Some(k) = ["status", "json", "text", "from"].iter().find(|k| f.contains_key(**k)) {
            return Err(format!("a `fault` has no `{k}`"));
        }
        return Ok(Answer::Fault(match fault.as_str() {
            "timeout" => Fault::Timeout,
            "offline" => Fault::Offline,
            "garbage" => Fault::Garbage,
            _ => return Err(format!("unknown fault {fault:?}: \"timeout\", \"offline\" or \"garbage\"")),
        }));
    }
    let status = field::<u16>(f, "status")?.unwrap_or(200);
    let body = match (f.get("json"), field::<String>(f, "text")?, field::<String>(f, "from")?) {
        (None, None, None) => vec![],
        (Some(j), None, None) => j.get().as_bytes().to_vec(),
        (None, Some(t), None) => t.into_bytes(),
        (None, None, Some(p)) => std::fs::read(dir.join(&p)).map_err(|e| format!("reading the body {p}: {e}"))?,
        _ => return Err("an answer has one of `json`, `text` or `from`".into()),
    };
    Ok(Answer::Reply { status, body })
}

impl Exchange {
    /// One exchange in `net.json`'s form (a `net` step holds one too). `from` names a file in
    /// `dir`.
    pub fn parse(text: &str, dir: &Path) -> Result<Self, String> {
        let f: Fields = serde_json::from_str(text).map_err(|e| format!("not a JSON object: {e}"))?;
        let known = ["url", "method", "sent", "in_turn"];
        if let Some(k) = f.keys().find(|k| !known.contains(&k.as_str()) && !ANSWER_FIELDS.contains(&k.as_str())) {
            return Err(format!("unknown field `{k}` in an exchange"));
        }
        let url: String = field(&f, "url")?.ok_or("an exchange needs `url`, a part of the request's URL")?;
        let method = field::<String>(&f, "method")?.map(|m| m.to_ascii_uppercase());
        if method.as_deref().is_some_and(|m| m != "GET" && m != "POST") {
            return Err("`method` is \"GET\" or \"POST\"".into());
        }
        let answers = match f.get("in_turn") {
            Some(list) => {
                if let Some(k) = ANSWER_FIELDS.iter().find(|k| f.contains_key(**k)) {
                    return Err(format!("an `in_turn` exchange has its answers in the list, not `{k}`"));
                }
                let list: Vec<Fields> =
                    serde_json::from_str(list.get()).map_err(|e| format!("`in_turn` is a list of answers: {e}"))?;
                let mut q = VecDeque::new();
                for (i, a) in list.iter().enumerate() {
                    if let Some(k) = a.keys().find(|k| !ANSWER_FIELDS.contains(&k.as_str())) {
                        return Err(format!("in_turn[{i}]: unknown field `{k}` in an answer"));
                    }
                    q.push_back(answer(a, dir).map_err(|e| format!("in_turn[{i}]: {e}"))?);
                }
                Answers::InTurn(q)
            }
            None => Answers::Always(answer(&f, dir)?),
        };
        Ok(Exchange { url, method, sent: field(&f, "sent")?, answers })
    }

    fn matches(&self, a: &Asked) -> bool {
        a.url.contains(&self.url)
            && self.method.as_deref().is_none_or(|m| m == a.method)
            && self.sent.as_deref().is_none_or(|s| a.sent.as_deref().is_some_and(|b| b.contains(s)))
    }
}

/// A loaded `net.json`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Recording {
    exchanges: Vec<Exchange>,
}

impl Recording {
    /// `net.json`'s text; `from` bodies are read from `dir`. Errors name the exchange.
    pub fn parse(text: &str, dir: &Path) -> Result<Self, String> {
        #[derive(serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        struct File {
            exchanges: Vec<Box<RawValue>>,
        }
        let f: File =
            serde_json::from_str(text).map_err(|e| format!("{FILE}: expected {{\"exchanges\": [...]}}: {e}"))?;
        let exchanges = f
            .exchanges
            .iter()
            .enumerate()
            .map(|(i, e)| Exchange::parse(e.get(), dir).map_err(|e| format!("{FILE}: exchange {i}: {e}")))
            .collect::<Result<_, _>>()?;
        Ok(Recording { exchanges })
    }

    /// The recording at `path`, its bodies beside it.
    pub fn load(path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("reading {}: {e}", path.display()))?;
        Self::parse(&text, path.parent().unwrap_or(Path::new(".")))
    }
}

/// A request as a recording sees it: its method, its URL with the query (redacted), and the
/// body it sent (redacted), if it was text.
#[derive(Debug, Clone, PartialEq)]
pub struct Asked {
    pub method: &'static str,
    pub url: String,
    pub sent: Option<String>,
    /// The values redacted out of it, which a recorded answer must not hold either.
    secrets: Vec<String>,
}

impl Asked {
    /// `GET url?query`.
    pub fn get<K: AsRef<str>, V: AsRef<str>>(url: &str, query: &[(K, V)]) -> Self {
        let mut secrets = vec![];
        let mut full = url.to_string();
        for (i, (k, v)) in query.iter().enumerate() {
            let (k, v) = (k.as_ref(), v.as_ref());
            full.push(if i == 0 && !url.contains('?') { '?' } else { '&' });
            full.push_str(&encode_segment(k));
            full.push('=');
            if secret(k) {
                secrets.push(v.to_string());
                full.push_str(REDACTED);
            } else {
                full.push_str(&encode_segment(v));
            }
        }
        Asked { method: "GET", url: full, sent: None, secrets }
    }

    /// `POST` of a JSON body.
    pub fn post_json(url: &str, body: &[u8]) -> Self {
        let mut secrets = vec![];
        let sent = match serde_json::from_slice::<Value>(body) {
            Ok(mut v) => {
                redact_json(&mut v, &mut secrets);
                v.to_string()
            }
            Err(_) => String::from_utf8_lossy(body).into_owned(),
        };
        Asked { method: "POST", url: url.to_string(), sent: Some(sent), secrets }
    }

    /// `POST` of a form.
    pub fn post_form<K: AsRef<str>, V: AsRef<str>>(url: &str, form: &[(K, V)]) -> Self {
        let mut secrets = vec![];
        let pairs: Vec<String> = form
            .iter()
            .map(|(k, v)| {
                let (k, v) = (k.as_ref(), v.as_ref());
                if secret(k) {
                    secrets.push(v.to_string());
                    format!("{k}={REDACTED}")
                } else {
                    format!("{k}={v}")
                }
            })
            .collect();
        Asked { method: "POST", url: url.to_string(), sent: Some(pairs.join("&")), secrets }
    }

    /// `POST` of a body that isn't text (EDDN's gzipped message): matched by its URL alone.
    pub fn post_bytes(url: &str) -> Self {
        Asked { method: "POST", url: url.to_string(), sent: None, secrets: vec![] }
    }

    /// The remote-data client's request `req` to `url`.
    pub fn of(url: &str, req: &Request) -> Self {
        let get = Asked::get(url, &req.query);
        let mut a = match &req.body {
            Body::Empty => Asked { method: method_name(req.method), ..get.clone() },
            Body::Json(v) => Asked::post_json(&get.url, v.to_string().as_bytes()),
            Body::Form(f) => Asked::post_form(&get.url, f),
        };
        a.method = method_name(req.method);
        a.secrets.extend(get.secrets);
        a
    }

    /// A request the [`Server`] got: `target` its request line's path (the real URL's host and
    /// path, as [`Server::url`] made it).
    fn served(method: &str, target: &str, body: &[u8]) -> Self {
        let real = match target.strip_prefix('/') {
            Some(rest) if rest.starts_with("http:") || rest.starts_with("https:") => rest.to_string(),
            Some(rest) => format!("https://{rest}"),
            None => target.to_string(),
        };
        let (url, query) = match real.split_once('?') {
            Some((u, q)) => (u.to_string(), q.to_string()),
            None => (real, String::new()),
        };
        let mut secrets = vec![];
        let mut full = url;
        for (i, pair) in query.split('&').filter(|p| !p.is_empty()).enumerate() {
            full.push(if i == 0 { '?' } else { '&' });
            match pair.split_once('=') {
                Some((k, v)) if secret(k) => {
                    secrets.push(v.to_string());
                    full.push_str(&format!("{k}={REDACTED}"));
                }
                _ => full.push_str(pair),
            }
        }
        let method = if method.eq_ignore_ascii_case("POST") { "POST" } else { "GET" };
        let sent = (!body.is_empty()).then(|| String::from_utf8_lossy(body).into_owned());
        Asked { method, url: full, sent, secrets }
    }
}

fn method_name(m: Method) -> &'static str {
    match m {
        Method::Get => "GET",
        Method::Post => "POST",
    }
}

/// Replace the values of secret fields anywhere in `v`, keeping them in `secrets`.
fn redact_json(v: &mut Value, secrets: &mut Vec<String>) {
    match v {
        Value::Object(o) => {
            for (k, x) in o.iter_mut() {
                if secret(k) && !x.is_null() {
                    secrets.push(x.as_str().map(String::from).unwrap_or_else(|| x.to_string()));
                    *x = json!(REDACTED);
                } else {
                    redact_json(x, secrets);
                }
            }
        }
        Value::Array(a) => a.iter_mut().for_each(|x| redact_json(x, secrets)),
        _ => {}
    }
}

/// Replace each of `secrets` inside every string in `v` (parsed, so JSON escapes are undone);
/// true if any was there.
fn scrub_strings(v: &mut Value, secrets: &[String]) -> bool {
    match v {
        Value::String(s) => {
            let mut hit = false;
            for x in secrets.iter().filter(|x| !x.is_empty() && x.as_str() != REDACTED) {
                if s.contains(x.as_str()) {
                    *s = s.replace(x.as_str(), REDACTED);
                    hit = true;
                }
            }
            hit
        }
        Value::Object(o) => o.values_mut().fold(false, |hit, x| scrub_strings(x, secrets) | hit),
        Value::Array(a) => a.iter_mut().fold(false, |hit, x| scrub_strings(x, secrets) | hit),
        _ => false,
    }
}

/// How a request was answered.
#[derive(Debug, Clone, PartialEq)]
pub enum Answered {
    Reply {
        status: u16,
        body: Vec<u8>,
    },
    Timeout,
    Offline,
    /// No exchange matched: refused.
    Unrecorded,
}

/// A request a replay or a recorder saw, for the world's observations.
#[derive(Debug, Clone, PartialEq)]
pub struct Seen {
    pub method: &'static str,
    /// The URL with its query, redacted.
    pub url: String,
    /// The recording held no answer for it, so it was refused.
    pub unrecorded: bool,
    /// Which exchange answered it: its place in the recording, then the ones added after
    /// ([`Replay::add`]) in turn. None when refused, and in record mode.
    pub exchange: Option<usize>,
}

type Observe = Box<dyn Fn(&Seen) + Send + Sync>;

/// A recording being answered from. A `net` step adds an exchange to it as the world runs.
pub struct Replay {
    exchanges: Mutex<Vec<Exchange>>,
    observe: Observe,
}

impl Replay {
    pub fn new(r: Recording) -> Self {
        Replay { exchanges: Mutex::new(r.exchanges), observe: Box::new(|_| {}) }
    }

    /// Tell `f` of every request.
    pub fn observe(mut self, f: impl Fn(&Seen) + Send + Sync + 'static) -> Self {
        self.observe = Box::new(f);
        self
    }

    /// Add `e` after the recording's exchanges, so it answers what it matches from now on.
    pub fn add(&self, e: Exchange) {
        self.exchanges.lock().unwrap_or_else(|e| e.into_inner()).push(e);
    }

    /// The recorded answer to `a`.
    pub fn answer(&self, a: &Asked) -> Answered {
        let (exchange, found) = {
            let mut exchanges = self.exchanges.lock().unwrap_or_else(|e| e.into_inner());
            let answer = exchanges.iter_mut().enumerate().rev().filter(|(_, e)| e.matches(a)).find_map(|(i, e)| {
                match &mut e.answers {
                    Answers::Always(x) => Some(x.clone()),
                    Answers::InTurn(q) => q.pop_front(),
                }
                .map(|x| (i, x))
            });
            answer.map_or((None, None), |(i, x)| (Some(i), Some(x)))
        };
        (self.observe)(&Seen { method: a.method, url: a.url.clone(), unrecorded: found.is_none(), exchange });
        match found {
            None => Answered::Unrecorded,
            Some(Answer::Reply { status, body }) => Answered::Reply { status, body },
            Some(Answer::Fault(Fault::Timeout)) => Answered::Timeout,
            Some(Answer::Fault(Fault::Offline)) => Answered::Offline,
            Some(Answer::Fault(Fault::Garbage)) => Answered::Reply { status: 200, body: GARBAGE.to_vec() },
        }
    }
}

/// Why a refused request failed, for the log.
pub fn unrecorded(url: &str) -> String {
    format!("this world's recording has no answer for {url}")
}

impl Transport for Replay {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        let a = Asked::of(url, req);
        match self.answer(&a) {
            Answered::Reply { status, body } => {
                Ok(Response { status, body: String::from_utf8_lossy(&body).into_owned() })
            }
            Answered::Timeout => Err(TransportError::Timeout),
            Answered::Offline => Err(TransportError::Offline("this world's recording answers offline".into())),
            Answered::Unrecorded => Err(TransportError::Offline(unrecorded(&a.url))),
        }
    }
}

/// The replay over HTTP on 127.0.0.1, for what fetches a URL itself (the update feed and zip,
/// the speech models, the neutron star file). [`Server::url`] gives the address that stands
/// for a real URL; the server answers each request from the replay, as the real URL. Every
/// answer closes its connection; a fault or a refused request closes it unanswered. It stops
/// when dropped.
pub struct Server {
    port: u16,
    stop: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Server {
    pub fn start(replay: Arc<Replay>) -> std::io::Result<Self> {
        let l = TcpListener::bind("127.0.0.1:0")?;
        let port = l.local_addr()?.port();
        let stop = Arc::new(AtomicBool::new(false));
        let s = stop.clone();
        let thread = std::thread::Builder::new().name("net-replay".into()).spawn(move || {
            for conn in l.incoming() {
                if s.load(Ordering::SeqCst) {
                    break;
                }
                let Ok(conn) = conn else { continue };
                let r = replay.clone();
                let _ = std::thread::Builder::new().name("net-replay-conn".into()).spawn(move || serve(conn, &r));
            }
        })?;
        Ok(Server { port, stop, thread: Some(thread) })
    }

    /// Where a request for `real` (an absolute URL, or a base URL that paths are added to) goes:
    /// `https://host/path` is `http://127.0.0.1:<port>/host/path`.
    pub fn url(&self, real: &str) -> String {
        let rest = real.strip_prefix("https://").map(String::from).unwrap_or_else(|| real.to_string());
        format!("http://127.0.0.1:{}/{rest}", self.port)
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        // Wake the accept, which then sees the stop.
        let _ = TcpStream::connect(("127.0.0.1", self.port));
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

/// Read one request from `conn`, answer it from `replay`, and close.
fn serve(conn: TcpStream, replay: &Replay) {
    let mut r = BufReader::new(match conn.try_clone() {
        Ok(c) => c,
        Err(_) => return,
    });
    let mut line = String::new();
    if r.read_line(&mut line).unwrap_or(0) == 0 {
        return;
    }
    let mut parts = line.split_whitespace();
    let (Some(method), Some(target)) = (parts.next(), parts.next()) else { return };
    let mut len = 0usize;
    loop {
        let mut h = String::new();
        if r.read_line(&mut h).unwrap_or(0) == 0 || h == "\r\n" || h == "\n" {
            break;
        }
        if let Some((k, v)) = h.split_once(':') {
            if k.trim().eq_ignore_ascii_case("content-length") {
                len = v.trim().parse().unwrap_or(0);
            }
        }
    }
    let mut body = vec![0; len];
    if r.read_exact(&mut body).is_err() {
        return;
    }
    let mut conn = conn;
    if let Answered::Reply { status, body } = replay.answer(&Asked::served(method, target, &body)) {
        let head = format!("HTTP/1.1 {status} Recorded\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len());
        let _ = conn.write_all(head.as_bytes()).and_then(|()| conn.write_all(&body));
    }
    // A fault or a refused request: closed with no answer.
    let _ = conn.shutdown(std::net::Shutdown::Both);
}

/// How a recorded request went, for [`Tape::record`].
pub enum Outcome<'a> {
    Reply { status: u16, body: &'a str },
    Fault(Fault),
}

/// One recorded answer, as `net.json` holds it.
#[derive(serde::Serialize)]
struct Written {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<u16>,
    /// A JSON answer as it came (unless a secret field in it was blanked).
    #[serde(skip_serializing_if = "Option::is_none")]
    json: Option<Box<RawValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fault: Option<&'static str>,
}

/// One recorded exchange: one answer, or the answers to a request asked again, in turn.
#[derive(serde::Serialize)]
struct Recorded {
    url: String,
    method: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    sent: Option<String>,
    #[serde(flatten)]
    answer: Option<Written>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    in_turn: Vec<Written>,
}

/// A recording being written from real answers (record mode). After each request the whole
/// file is rewritten, so it is complete whenever the process stops. A request asked again
/// (same method, URL and body) adds its answer to the first one's, answered in turn.
pub struct Tape {
    path: PathBuf,
    exchanges: Mutex<Vec<Recorded>>,
    observe: Observe,
}

impl Tape {
    /// Start an empty recording at `path`.
    pub fn create(path: &Path) -> std::io::Result<Self> {
        let t = Tape { path: path.to_path_buf(), exchanges: Mutex::new(vec![]), observe: Box::new(|_| {}) };
        t.write(&[])?;
        Ok(t)
    }

    /// Tell `f` of every request.
    pub fn observe(mut self, f: impl Fn(&Seen) + Send + Sync + 'static) -> Self {
        self.observe = Box::new(f);
        self
    }

    fn write(&self, exchanges: &[Recorded]) -> std::io::Result<()> {
        #[derive(serde::Serialize)]
        struct File<'a> {
            exchanges: &'a [Recorded],
        }
        let text = serde_json::to_string_pretty(&File { exchanges })?;
        let tmp = self.path.with_extension("json.part");
        std::fs::write(&tmp, text + "\n")?;
        std::fs::rename(&tmp, &self.path)
    }

    /// Add `a`'s answer to the recording, redacted.
    pub fn record(&self, a: &Asked, outcome: Outcome) -> std::io::Result<()> {
        (self.observe)(&Seen { method: a.method, url: a.url.clone(), unrecorded: false, exchange: None });
        let written = match outcome {
            Outcome::Fault(f) => Written { status: None, json: None, text: None, fault: Some(f.name()) },
            Outcome::Reply { status, body } => {
                let mut body = body.to_string();
                for s in a.secrets.iter().filter(|s| !s.is_empty()) {
                    body = body.replace(s.as_str(), REDACTED);
                }
                let (json, text) = match serde_json::from_str::<Value>(&body) {
                    Ok(mut v) => {
                        let mut found = vec![];
                        redact_json(&mut v, &mut found);
                        // The text replace above misses a secret the answer wrote JSON-escaped
                        // (`\u00eb`, `\/`); its parsed strings hold it unescaped.
                        let echoed = scrub_strings(&mut v, &a.secrets);
                        let raw = if found.is_empty() && !echoed { body } else { v.to_string() };
                        (Some(RawValue::from_string(raw).map_err(std::io::Error::other)?), None)
                    }
                    Err(_) if body.is_empty() => (None, None),
                    Err(_) => (None, Some(body)),
                };
                Written { status: Some(status), json, text, fault: None }
            }
        };
        let mut exchanges = self.exchanges.lock().unwrap_or_else(|e| e.into_inner());
        match exchanges.iter_mut().find(|e| e.url == a.url && e.method == a.method && e.sent == a.sent) {
            Some(e) => {
                if let Some(first) = e.answer.take() {
                    e.in_turn.push(first);
                }
                e.in_turn.push(written);
            }
            None => exchanges.push(Recorded {
                url: a.url.clone(),
                method: a.method,
                sent: a.sent.clone(),
                answer: Some(written),
                in_turn: vec![],
            }),
        }
        self.write(&exchanges)
    }
}

/// The remote-data client's transport in record mode: each request goes to `real`, and its
/// answer is recorded on the tape.
pub struct Recorder {
    real: Arc<dyn Transport>,
    tape: Arc<Tape>,
}

impl Recorder {
    pub fn new(real: Arc<dyn Transport>, tape: Arc<Tape>) -> Self {
        Recorder { real, tape }
    }
}

impl Transport for Recorder {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        let r = self.real.send(url, req);
        let outcome = match &r {
            Ok(resp) => Outcome::Reply { status: resp.status, body: &resp.body },
            Err(TransportError::Timeout) => Outcome::Fault(Fault::Timeout),
            Err(TransportError::Offline(_)) => Outcome::Fault(Fault::Offline),
        };
        if let Err(e) = self.tape.record(&Asked::of(url, req), outcome) {
            return Err(TransportError::Offline(format!("couldn't write the recording: {e}")));
        }
        r
    }
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)
mod tests {
    use super::*;

    fn rec(text: &str) -> Replay {
        Replay::new(Recording::parse(text, Path::new("/nowhere")).unwrap())
    }

    fn err(text: &str) -> String {
        Recording::parse(text, Path::new("/nowhere")).unwrap_err()
    }

    #[test]
    fn the_last_match_answers_and_an_exhausted_in_turn_falls_through() {
        let r = rec(r#"{"exchanges": [
            {"url": "/api/", "text": "any"},
            {"url": "/api/results/", "in_turn": [{"status": 503}, {"json": {"done": true}}]},
            {"url": "/api/results/", "method": "post", "fault": "offline"}
        ]}"#);
        let get = Asked::get("https://spansh.co.uk/api/results/1", &[] as &[(&str, &str)]);
        assert_eq!(r.answer(&get), Answered::Reply { status: 503, body: vec![] });
        assert_eq!(r.answer(&get), Answered::Reply { status: 200, body: br#"{"done": true}"#.to_vec() });
        assert_eq!(r.answer(&get), Answered::Reply { status: 200, body: b"any".to_vec() });
        assert_eq!(r.answer(&Asked::post_json("https://spansh.co.uk/api/results/1", b"{}")), Answered::Offline);
        assert_eq!(r.answer(&Asked::get("https://www.edsm.net/x", &[] as &[(&str, &str)])), Answered::Unrecorded);
    }

    #[test]
    fn a_sent_matcher_reads_the_body_and_secrets_never_reach_the_match() {
        let r = rec(r#"{"exchanges": [
            {"url": "/search", "sent": "\"Sol\"", "text": "sol"},
            {"url": "/logs?commanderName=<redacted>&apiKey=<redacted>", "text": "logs"}
        ]}"#);
        let sol = Asked::post_json("https://x/search", br#"{"reference":"Sol"}"#);
        assert_eq!(r.answer(&sol), Answered::Reply { status: 200, body: b"sol".to_vec() });
        assert_eq!(r.answer(&Asked::post_json("https://x/search", br#"{"reference":"Lave"}"#)), Answered::Unrecorded);
        let logs = Asked::get("https://x/logs", &[("commanderName", "Jameson"), ("apiKey", "k3y")]);
        assert_eq!(logs.url, "https://x/logs?commanderName=<redacted>&apiKey=<redacted>");
        assert_eq!(r.answer(&logs), Answered::Reply { status: 200, body: b"logs".to_vec() });
        let form = Asked::post_form("https://x/f", &[("api_token", "t0k"), ("system", "Sol")]);
        assert_eq!(form.sent.as_deref(), Some("api_token=<redacted>&system=Sol"));
        let posted = Asked::post_json("https://x/j", br#"{"header":{"apiKey":"k3y"},"commanderName":"Jameson","n":1}"#);
        assert_eq!(
            posted.sent.as_deref(),
            Some(r#"{"commanderName":"<redacted>","header":{"apiKey":"<redacted>"},"n":1}"#)
        );
        assert!(secret("APIKEY") && secret("commanderName") && secret("Password") && !secret("system"));
    }

    #[test]
    fn a_malformed_exchange_fails_the_load_naming_it() {
        assert!(err("[]").contains("expected {\"exchanges\""));
        assert!(err(r#"{"exchanges": [{"text": "x"}]}"#).contains("exchange 0: an exchange needs `url`"));
        assert!(err(r#"{"exchanges": [{"url": "x"}, {"url": "y", "colour": 1}]}"#)
            .contains("exchange 1: unknown field `colour`"));
        assert!(err(r#"{"exchanges": [{"url": "x", "fault": "slow"}]}"#).contains("unknown fault \"slow\""));
        assert!(err(r#"{"exchanges": [{"url": "x", "fault": "offline", "status": 200}]}"#)
            .contains("a `fault` has no `status`"));
        assert!(
            err(r#"{"exchanges": [{"url": "x", "text": "a", "json": 1}]}"#).contains("one of `json`, `text` or `from`")
        );
        assert!(err(r#"{"exchanges": [{"url": "x", "in_turn": [{"status": 200}], "status": 200}]}"#)
            .contains("in the list"));
        assert!(err(r#"{"exchanges": [{"url": "x", "in_turn": [{"url": "y"}]}]}"#)
            .contains("in_turn[0]: unknown field `url`"));
        assert!(err(r#"{"exchanges": [{"url": "x", "method": "PUT"}]}"#).contains("`method` is"));
        assert!(
            err(r#"{"exchanges": [{"url": "x", "from": "missing.json"}]}"#).contains("reading the body missing.json")
        );
    }

    #[test]
    fn a_body_comes_from_a_file_beside_the_recording() {
        let d = tempfile_dir();
        std::fs::write(d.join("body.json"), "{\"a\": 1}").unwrap();
        std::fs::write(d.join(FILE), r#"{"exchanges": [{"url": "/a", "from": "body.json"}]}"#).unwrap();
        let r = Replay::new(Recording::load(&d.join(FILE)).unwrap());
        assert_eq!(
            r.answer(&Asked::get("https://x/a", &[] as &[(&str, &str)])),
            Answered::Reply { status: 200, body: b"{\"a\": 1}".to_vec() }
        );
        let _ = std::fs::remove_dir_all(d);
    }

    fn tempfile_dir() -> PathBuf {
        static N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let d = std::env::temp_dir().join(format!(
            "astromech-replay-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::SeqCst)
        ));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    /// Each server answer, through the download functions that fetch from it.
    #[test]
    fn the_server_answers_downloads_from_the_replay_and_reports_each() {
        let seen = Arc::new(Mutex::new(vec![]));
        let s = seen.clone();
        let r = Arc::new(
            rec(r#"{"exchanges": [
                {"url": "github.com/a/releases/download/neutron/x.sha256", "text": "abc123  x\n"},
                {"url": "/down.sha256", "status": 503},
                {"url": "/gone.sha256", "fault": "offline"},
                {"url": "/slow.sha256", "fault": "timeout"},
                {"url": "/junk.bin", "fault": "garbage"}
            ]}"#)
            .observe(move |x| s.lock().unwrap().push(x.clone())),
        );
        let server = Server::start(r).unwrap();
        let base = server.url("https://github.com/a/releases/download/neutron");
        assert!(
            base.starts_with("http://127.0.0.1:") && base.ends_with("/github.com/a/releases/download/neutron"),
            "{base}"
        );
        assert_eq!(crate::download::text(&format!("{base}/x.sha256"), false).unwrap(), "abc123  x\n");
        use crate::download::{text, DownloadError};
        assert!(matches!(text(&server.url("https://h/down.sha256"), false), Err(DownloadError::Status(503))));
        assert!(matches!(text(&server.url("https://h/gone.sha256"), false), Err(DownloadError::Offline(_))));
        assert!(matches!(text(&server.url("https://h/slow.sha256"), false), Err(DownloadError::Offline(_))));
        assert!(matches!(text(&server.url("https://h/none.sha256"), false), Err(DownloadError::Offline(_))));
        let mut out = vec![];
        crate::download::stream(&server.url("https://h/junk.bin"), false, &mut out, |_, _| {}).unwrap();
        assert_eq!(out, GARBAGE);
        let seen = seen.lock().unwrap().clone();
        let urls: Vec<(&str, bool)> = seen.iter().map(|s| (s.url.as_str(), s.unrecorded)).collect();
        assert_eq!(
            urls,
            [
                ("https://github.com/a/releases/download/neutron/x.sha256", false),
                ("https://h/down.sha256", false),
                ("https://h/gone.sha256", false),
                ("https://h/slow.sha256", false),
                ("https://h/none.sha256", true),
                ("https://h/junk.bin", false),
            ]
        );
        assert!(seen.iter().all(|s| s.method == "GET"));
    }

    #[test]
    fn a_tape_records_redacted_answers_and_a_repeat_in_turn() {
        let d = tempfile_dir();
        let path = d.join("net.json");
        let tape = Tape::create(&path).unwrap();
        let logs = Asked::get("https://x/logs", &[("commanderName", "Jameson"), ("apiKey", "k3y")]);
        tape.record(&logs, Outcome::Reply { status: 200, body: r#"{"msg":"OK","commander":"Jameson","logs":[]}"# })
            .unwrap();
        let poll = Asked::get("https://x/results/1", &[] as &[(&str, &str)]);
        tape.record(&poll, Outcome::Reply { status: 202, body: "{\"status\": \"queued\"}" }).unwrap();
        tape.record(&poll, Outcome::Fault(Fault::Timeout)).unwrap();
        tape.record(&poll, Outcome::Reply { status: 200, body: "done" }).unwrap();
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(!text.contains("Jameson") && !text.contains("k3y"), "{text}");
        let r = Replay::new(Recording::load(&path).unwrap());
        let other = Asked::get("https://x/logs", &[("commanderName", "Other"), ("apiKey", "else")]);
        assert_eq!(
            r.answer(&other),
            Answered::Reply { status: 200, body: br#"{"msg":"OK","commander":"<redacted>","logs":[]}"#.to_vec() }
        );
        assert_eq!(r.answer(&poll), Answered::Reply { status: 202, body: b"{\"status\": \"queued\"}".to_vec() });
        assert_eq!(r.answer(&poll), Answered::Timeout);
        assert_eq!(r.answer(&poll), Answered::Reply { status: 200, body: b"done".to_vec() });
        assert_eq!(r.answer(&poll), Answered::Unrecorded, "each answer once, as recorded");
        let _ = std::fs::remove_dir_all(d);
    }

    /// An answer that echoes a secret JSON-escaped (`\uXXXX`, `\/`) in a field not named like
    /// one is redacted too: the raw text holds the secret only in its escaped form.
    #[test]
    fn a_tape_redacts_secrets_an_answer_echoes_json_escaped() {
        let d = tempfile_dir();
        let path = d.join("net.json");
        let tape = Tape::create(&path).unwrap();
        let logs = Asked::get("https://x/logs", &[("commanderName", "Zoë/K"), ("apiKey", "k/3y")]);
        let body = r#"{"msg":"Hello, CMDR Zo\u00eb\/K","note":"key k\/3y","logs":[]}"#;
        tape.record(&logs, Outcome::Reply { status: 200, body }).unwrap();
        let text = std::fs::read_to_string(&path).unwrap();
        let decoded = serde_json::from_str::<Value>(&text).unwrap().to_string();
        assert!(!decoded.contains("Zoë/K") && !decoded.contains("k/3y"), "{text}");
        assert!(decoded.contains("Hello, CMDR <redacted>") && decoded.contains("key <redacted>"), "{text}");
        let _ = std::fs::remove_dir_all(d);
    }
}
