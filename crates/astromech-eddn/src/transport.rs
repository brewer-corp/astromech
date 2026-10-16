//! The HTTP upload, behind a trait so tests never touch the real gateway.

use std::time::Duration;

/// The live EDDN gateway (HTTPS, port 4430, trailing slash required).
pub const UPLOAD_URL: &str = "https://eddn.edcd.io:4430/upload/";

/// An HTTP answer from the gateway (any status, including errors).
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

pub trait Transport {
    /// POST one gzipped message. `Err` is a failure with no HTTP status (refused, timed out, TLS).
    fn post(&self, gzipped_json: &[u8]) -> Result<Response, String>;
}

/// HTTP/1.1 POST with `Content-Encoding: gzip`, as `docs/Developers.md` asks.
pub struct HttpTransport {
    agent: ureq::Agent,
    url: String,
    user_agent: String,
}

impl HttpTransport {
    pub fn new(url: &str, user_agent: &str) -> Self {
        let agent = ureq::AgentBuilder::new().timeout(Duration::from_secs(30)).build();
        Self { agent, url: url.into(), user_agent: user_agent.into() }
    }
}

impl Transport for HttpTransport {
    fn post(&self, gzipped_json: &[u8]) -> Result<Response, String> {
        let r = self
            .agent
            .post(&self.url)
            .set("User-Agent", &self.user_agent)
            .set("Content-Type", "application/json")
            .set("Content-Encoding", "gzip")
            .send_bytes(gzipped_json);
        let resp = match r {
            Ok(r) => r,
            Err(ureq::Error::Status(_, r)) => r,
            Err(ureq::Error::Transport(t)) => return Err(t.to_string()),
        };
        let status = resp.status();
        let mut body = resp.into_string().unwrap_or_default();
        body.truncate(500);
        Ok(Response { status, body })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader, Read, Write};
    use std::net::TcpListener;

    /// One-shot local HTTP server: returns the request head and body it got.
    fn serve_once(status: &str, body: &'static str) -> (String, std::thread::JoinHandle<(String, Vec<u8>)>) {
        let l = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/upload/", l.local_addr().unwrap());
        let status = status.to_owned();
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
            write!(s, "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len())
                .unwrap();
            (head, got)
        });
        (url, h)
    }

    #[test]
    fn posts_gzip_with_the_required_headers_and_returns_error_statuses() {
        let (url, h) = serve_once("426 Upgrade Required", "FAIL: Oudated Schema");
        let t = HttpTransport::new(&url, "astromech/0.1.5");
        let r = t.post(b"\x1f\x8bgz").unwrap();
        assert_eq!(r, Response { status: 426, body: "FAIL: Oudated Schema".into() });
        let (head, body) = h.join().unwrap();
        let head = head.to_ascii_lowercase();
        assert!(head.starts_with("post /upload/ http/1.1"), "{head}");
        assert!(head.contains("content-encoding: gzip"), "{head}");
        assert!(head.contains("content-type: application/json"), "{head}");
        assert!(head.contains("user-agent: astromech/0.1.5"), "{head}");
        assert_eq!(body, b"\x1f\x8bgz");
    }

    #[test]
    fn connection_failure_is_an_error_without_status() {
        let l = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/upload/", l.local_addr().unwrap());
        drop(l);
        assert!(HttpTransport::new(&url, "astromech/test").post(b"x").is_err());
    }
}
