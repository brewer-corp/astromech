//! A recording transport: every answer is kept on disk forever (the spike's cache), live
//! requests are spaced 1 s apart per service (the real client's limit), counted, logged to a
//! ledger and capped. The real `Client` runs on top with `Limiter::none()`, so cached replays
//! are instant and live requests still go out at 1 request/s per service.

use astromech_data::{Body, HttpTransport, Method, Request, Response, Transport, TransportError};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[derive(Debug, Default, Clone, Copy)]
pub struct Tally {
    pub live: u32,
    pub cached: u32,
    pub bytes: u64,
    /// Wall time of live requests including the rate-limit waits.
    pub live_ms: u128,
}

pub struct DiskTransport {
    dir: PathBuf,
    http: HttpTransport,
    offline: bool,
    cap: usize,
    state: Mutex<State>,
}

#[derive(Default)]
struct State {
    next: HashMap<String, Instant>,
    tally: HashMap<String, Tally>,
}

fn service_of(url: &str) -> String {
    let host = url.split("://").nth(1).unwrap_or(url).split('/').next().unwrap_or("");
    match host {
        h if h.contains("spansh") => "spansh".into(),
        h if h.contains("ardent") => "ardent".into(),
        h if h.contains("edsm") => "edsm".into(),
        h => h.into(),
    }
}

/// FNV-1a 64: a stable file name for a request.
fn fnv(s: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

pub fn request_key(url: &str, req: &Request) -> String {
    let mut q = req.query.clone();
    q.sort();
    let q: Vec<String> = q.iter().map(|(k, v)| format!("{k}={v}")).collect();
    let body = match &req.body {
        Body::Empty => String::new(),
        Body::Json(v) => v.to_string(),
        Body::Form(f) => f.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join("&"),
    };
    let m = if req.method == Method::Get { "GET" } else { "POST" };
    format!("{m} {url}?{} {body}", q.join("&"))
}

impl DiskTransport {
    pub fn new(dir: PathBuf, offline: bool, cap: usize) -> Self {
        std::fs::create_dir_all(dir.join("answers")).expect("cache dir");
        DiskTransport { dir, http: HttpTransport::new(), offline, cap, state: Mutex::new(State::default()) }
    }

    fn ledger(&self) -> PathBuf {
        self.dir.join("ledger.tsv")
    }

    /// Live requests ever made by the spike (every run), from the ledger.
    pub fn ledger_count(&self) -> usize {
        std::fs::read_to_string(self.ledger()).map(|s| s.lines().count()).unwrap_or(0)
    }

    pub fn tally(&self) -> Vec<(String, Tally)> {
        let st = self.state.lock().unwrap();
        let mut v: Vec<_> = st.tally.iter().map(|(k, t)| (k.clone(), *t)).collect();
        v.sort_by(|a, b| a.0.cmp(&b.0));
        v
    }

    pub fn reset_tally(&self) {
        self.state.lock().unwrap().tally.clear();
    }
}

impl Transport for DiskTransport {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        let key = request_key(url, req);
        let path = self.dir.join("answers").join(format!("{:016x}.json", fnv(&key)));
        let svc = service_of(url);
        if let Ok(text) = std::fs::read_to_string(&path) {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
                if v["key"].as_str() == Some(key.as_str()) {
                    let body = v["body"].as_str().unwrap_or("").to_string();
                    let mut st = self.state.lock().unwrap();
                    let t = st.tally.entry(svc).or_default();
                    t.cached += 1;
                    t.bytes += body.len() as u64;
                    return Ok(Response { status: v["status"].as_u64().unwrap_or(200) as u16, body });
                }
            }
        }
        if self.offline {
            return Err(TransportError::Offline(format!("not in the spike cache (offline run): {key}")));
        }
        if self.ledger_count() >= self.cap {
            return Err(TransportError::Offline(format!("request cap of {} reached; stopping", self.cap)));
        }
        let start = Instant::now();
        // Rate limit: 1 request/s per service, like astromech_data::Limiter.
        let wait = {
            let mut st = self.state.lock().unwrap();
            let now = Instant::now();
            let at = st.next.get(&svc).copied().filter(|t| *t > now).unwrap_or(now);
            st.next.insert(svc.clone(), at + Duration::from_secs(1));
            at - now
        };
        std::thread::sleep(wait);
        let sent = Instant::now();
        let resp = self.http.send(url, req)?;
        let ms = sent.elapsed().as_millis();
        let unix = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        if resp.status < 500 && resp.status != 429 {
            let rec = serde_json::json!({"key": key, "status": resp.status, "fetched_at": unix, "body": resp.body});
            let _ = std::fs::write(&path, rec.to_string());
        }
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(self.ledger()) {
            let short: String = key.chars().take(300).collect();
            let _ = writeln!(f, "{unix}\t{svc}\t{}\t{}\t{ms}\t{short}", resp.status, resp.body.len());
        }
        eprintln!("  live {svc} {} {} B {ms} ms", resp.status, resp.body.len());
        let mut st = self.state.lock().unwrap();
        let t = st.tally.entry(svc).or_default();
        t.live += 1;
        t.bytes += resp.body.len() as u64;
        t.live_ms += start.elapsed().as_millis();
        Ok(resp)
    }
}
