//! HTTP behind a trait, so tests can answer with recorded bodies.

use crate::USER_AGENT;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Body {
    Empty,
    Json(serde_json::Value),
    Form(Vec<(String, String)>),
}

/// A request relative to a service's base URL. Path segments must already be encoded
/// ([`encode_segment`]); query and form values are encoded by the transport.
#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub query: Vec<(String, String)>,
    pub body: Body,
}

impl Request {
    pub fn get(path: impl Into<String>) -> Self {
        Request { method: Method::Get, path: path.into(), query: vec![], body: Body::Empty }
    }

    pub fn post_json(path: impl Into<String>, body: serde_json::Value) -> Self {
        Request { method: Method::Post, path: path.into(), query: vec![], body: Body::Json(body) }
    }

    pub fn post_form(path: impl Into<String>, form: Vec<(String, String)>) -> Self {
        Request { method: Method::Post, path: path.into(), query: vec![], body: Body::Form(form) }
    }

    pub fn query(mut self, k: &str, v: impl ToString) -> Self {
        self.query.push((k.to_string(), v.to_string()));
        self
    }

    /// Identifies the request in the cache. Query pairs are sorted so their order doesn't matter.
    pub(crate) fn cache_key(&self, service: &str) -> String {
        let mut q = self.query.clone();
        q.sort();
        let q: Vec<String> = q.iter().map(|(k, v)| format!("{k}={v}")).collect();
        let body = match &self.body {
            Body::Empty => String::new(),
            Body::Json(v) => v.to_string(),
            Body::Form(f) => f.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join("&"),
        };
        format!("{service} {:?} {}?{} {body}", self.method, self.path, q.join("&"))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

/// The connection failed before any HTTP answer: no network, DNS, refused, TLS, reset.
#[derive(Debug, Clone, PartialEq)]
pub enum TransportError {
    Offline(String),
    Timeout,
}

pub trait Transport: Send + Sync {
    /// Any HTTP status is an `Ok` response; only connection failures are errors.
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError>;
}

/// Real HTTP with the shared User-Agent.
pub struct HttpTransport {
    agent: ureq::Agent,
}

impl HttpTransport {
    pub fn new() -> Self {
        HttpTransport {
            agent: ureq::AgentBuilder::new()
                .timeout_connect(Duration::from_secs(10))
                .timeout(Duration::from_secs(30))
                .user_agent(USER_AGENT)
                .build(),
        }
    }
}

impl Default for HttpTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl Transport for HttpTransport {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        let method = match req.method {
            Method::Get => "GET",
            Method::Post => "POST",
        };
        let mut r = self.agent.request(method, url);
        for (k, v) in &req.query {
            r = r.query(k, v);
        }
        let result = match &req.body {
            Body::Empty => r.call(),
            Body::Json(v) => r.set("Content-Type", "application/json").send_string(&v.to_string()),
            Body::Form(f) => r.send_form(&f.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect::<Vec<_>>()),
        };
        let resp = match result {
            Ok(resp) | Err(ureq::Error::Status(_, resp)) => resp,
            Err(ureq::Error::Transport(t)) => return Err(transport_error(&t)),
        };
        let status = resp.status();
        let body = resp.into_string().map_err(|e| io_error(&e))?;
        Ok(Response { status, body })
    }
}

fn io_error(e: &std::io::Error) -> TransportError {
    match e.kind() {
        std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock => TransportError::Timeout,
        _ => TransportError::Offline(e.to_string()),
    }
}

fn transport_error(t: &ureq::Transport) -> TransportError {
    let io = std::error::Error::source(t).and_then(|s| s.downcast_ref::<std::io::Error>());
    match io {
        Some(e) => io_error(e),
        None => TransportError::Offline(t.to_string()),
    }
}

/// Percent-encode one URL path segment (a system or commodity name).
pub fn encode_segment(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segments_and_cache_keys() {
        assert_eq!(encode_segment("Barnard's Star"), "Barnard%27s%20Star");
        assert_eq!(encode_segment("HIP 1/2"), "HIP%201%2F2");
        let a = Request::get("/x").query("b", 2).query("a", 1);
        let b = Request::get("/x").query("a", 1).query("b", 2);
        assert_eq!(a.cache_key("s"), b.cache_key("s"));
        assert_ne!(a.cache_key("s"), Request::get("/x").query("a", 1).cache_key("s"));
        assert_ne!(
            Request::post_json("/x", serde_json::json!({"a": 1})).cache_key("s"),
            Request::post_json("/x", serde_json::json!({"a": 2})).cache_key("s")
        );
    }
}
