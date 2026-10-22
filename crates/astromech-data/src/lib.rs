//! # astromech-data
//!
//! Remote game data from Spansh, Ardent Insight and EDSM through one [`Client`]:
//!
//! - every request carries the shared [`USER_AGENT`];
//! - answers are cached in SQLite ([`Cache`]) and served without a request until their TTL ends
//!   (see [`ttl`]); a cache that can't be read or written is a miss, logged, never a failed
//!   lookup;
//! - every answer is checked against a typed schema before it is used or cached, and a mismatch
//!   is an [`Error::Schema`] naming the service;
//! - requests to each service are spaced out ([`Limiter`]);
//! - failures are [`Error`]s whose `Display` is a plain-words sentence fit to show or speak.
//!
//! A process makes one [`Client`] ([`Client::shared`]) and hands the `Arc` to everything that
//! looks something up, so the rate limit and the cache hold for the whole process.
//!
//! Endpoints live in [`spansh`], [`ardent`] and [`edsm`]. Tests swap the [`Transport`] for
//! recorded bodies.

pub mod ardent;
mod cache;
pub mod edsm;
mod limit;
pub mod spansh;
mod transport;

pub use cache::Cache;
pub use limit::Limiter;
pub use transport::{encode_segment, Body, HttpTransport, Method, Request, Response, Transport, TransportError};

use serde::de::DeserializeOwned;
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

pub const USER_AGENT: &str =
    concat!("astromech/", env!("CARGO_PKG_VERSION"), " (+https://github.com/brewer-corp/astromech)");

/// How long each kind of answer is reused, in seconds (ARCHITECTURE §5.8).
pub mod ttl {
    /// Positions, bodies and other slow-changing system data.
    pub const SYSTEM: i64 = 7 * 24 * 3600;
    /// Searches (stations, systems, bodies, nearest service).
    pub const SEARCH: i64 = 3600;
    /// Market prices and stock.
    pub const MARKET: i64 = 15 * 60;
    /// Traffic counts.
    pub const TRAFFIC: i64 = 3600;
}

/// A remote data service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Service {
    Spansh,
    Ardent,
    Edsm,
}

impl Service {
    pub const ALL: [Service; 3] = [Service::Spansh, Service::Ardent, Service::Edsm];

    /// The name to show the commander.
    pub fn name(self) -> &'static str {
        match self {
            Service::Spansh => "Spansh",
            Service::Ardent => "Ardent Insight",
            Service::Edsm => "EDSM",
        }
    }

    pub fn base_url(self) -> &'static str {
        match self {
            Service::Spansh => "https://spansh.co.uk",
            Service::Ardent => "https://api.ardent-insight.com",
            Service::Edsm => "https://www.edsm.net",
        }
    }

    /// Minimum gap between requests. None of the three publishes a limit for these endpoints;
    /// one a second matches ARCHITECTURE §5.8 for EDSM and is polite for the others.
    pub fn min_interval(self) -> Duration {
        Duration::from_secs(1)
    }

    fn key(self) -> &'static str {
        match self {
            Service::Spansh => "spansh",
            Service::Ardent => "ardent",
            Service::Edsm => "edsm",
        }
    }
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// Why a lookup failed. `Display` is a sentence for the commander.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Cannot reach {service}. Check your internet connection. If it is working, {service} may be down.")]
    Offline { service: Service, detail: String },
    #[error("{service} took too long to answer. Try again in a minute.")]
    Timeout { service: Service },
    #[error("{service} asked astromech to slow down. Try again in a minute.")]
    RateLimited { service: Service },
    #[error("{service} found nothing: {message}.")]
    NotFound { service: Service, message: String },
    #[error("{service} turned the request down: {message}.")]
    Rejected { service: Service, message: String },
    #[error("{service} is having trouble (HTTP {status}). Try again later.")]
    Unavailable { service: Service, status: u16 },
    /// `detail` (what serde said) goes to the log, not into the sentence.
    #[error("{service} sent an answer astromech does not understand. Its format may have changed.")]
    Schema { service: Service, detail: String },
}

impl Error {
    /// The service the error came from.
    pub fn service(&self) -> Service {
        match self {
            Error::Offline { service, .. }
            | Error::Timeout { service }
            | Error::RateLimited { service }
            | Error::NotFound { service, .. }
            | Error::Rejected { service, .. }
            | Error::Unavailable { service, .. }
            | Error::Schema { service, .. } => *service,
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// An answer and where it came from.
#[derive(Debug, Clone, PartialEq)]
pub struct Fetched<T> {
    pub value: T,
    /// When the answer was fetched from the service (unix seconds).
    pub fetched_at: i64,
    /// Served from the cache without a request.
    pub cached: bool,
}

/// The client's clock lives in `astromech-proto` (ARCHITECTURE §5.1), re-exported for its callers.
pub use astromech_proto::{Clock, SystemClock};

/// The file in the data folder (`astromech_store::Paths::data_dir`) holding the remote-data cache
/// both processes share.
pub const CACHE_FILE: &str = "remote_cache.sqlite";

/// Where the client's log lines go (the core's log file; stderr by default).
pub type Log = Arc<dyn Fn(&str) + Send + Sync>;

/// The one way to the remote services.
pub struct Client {
    transport: Arc<dyn Transport>,
    cache: Mutex<Cache>,
    clock: Arc<dyn Clock>,
    limiter: Mutex<Limiter>,
    bases: Vec<(Service, String)>,
    log: Log,
}

impl Client {
    /// Real HTTP, the system clock and the default rate limits.
    pub fn new(cache: Cache) -> Self {
        Self::with_transport(cache, Arc::new(HttpTransport::new()))
    }

    /// The process's one client, made by its composition root (the core's session, the app
    /// window's setup) and shared by every lookup: real HTTP, the default rate limits, and its
    /// cache in [`CACHE_FILE`] under `dir`, the file the core and the app window both use, so
    /// an answer fetched by one is served to the other. If the file can't be opened the cache
    /// lives in memory for this run, and `log` says why.
    pub fn shared(dir: &std::path::Path, log: Log) -> Arc<Self> {
        Self::shared_with_transport(dir, Arc::new(HttpTransport::new()), log)
    }

    /// [`Client::shared`] over `transport` (tests pass recorded answers).
    pub fn shared_with_transport(dir: &std::path::Path, transport: Arc<dyn Transport>, log: Log) -> Arc<Self> {
        let cache = Cache::open(&dir.join(CACHE_FILE)).unwrap_or_else(|e| {
            log(&format!("remote data cache not opened, answers won't be shared: {e}"));
            Cache::in_memory().expect("an in-memory SQLite cache always opens")
        });
        Arc::new(Self::with_transport(cache, transport).log(log))
    }

    pub fn with_transport(cache: Cache, transport: Arc<dyn Transport>) -> Self {
        Client {
            transport,
            cache: Mutex::new(cache),
            clock: Arc::new(SystemClock),
            limiter: Mutex::new(Limiter::default()),
            bases: Service::ALL.iter().map(|s| (*s, s.base_url().to_string())).collect(),
            log: Arc::new(|s: &str| eprintln!("{s}")),
        }
    }

    pub fn log(mut self, log: Log) -> Self {
        self.log = log;
        self
    }

    pub fn clock(mut self, clock: Arc<dyn Clock>) -> Self {
        self.clock = clock;
        self
    }

    pub fn limiter(mut self, limiter: Limiter) -> Self {
        self.limiter = Mutex::new(limiter);
        self
    }

    /// Send `service`'s requests to `base` instead (a mirror, or a test server).
    pub fn base_url(mut self, service: Service, base: impl Into<String>) -> Self {
        let base = base.into();
        for (s, b) in &mut self.bases {
            if *s == service {
                *b = base.clone();
            }
        }
        self
    }

    fn base(&self, service: Service) -> &str {
        self.bases.iter().find(|(s, _)| *s == service).map(|(_, b)| b.as_str()).unwrap_or(service.base_url())
    }

    fn cache(&self) -> MutexGuard<'_, Cache> {
        self.cache.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// One rate-limited request, without the cache. 2xx bodies are returned as-is; anything
    /// else becomes an [`Error`].
    pub fn send(&self, service: Service, req: &Request) -> Result<String> {
        let resp = self.exchange(service, req)?;
        status_result(service, resp)
    }

    /// One rate-limited request: any HTTP answer, or the connection failure as an [`Error`].
    pub(crate) fn exchange(&self, service: Service, req: &Request) -> Result<Response> {
        let wait = self.limiter.lock().unwrap_or_else(|e| e.into_inner()).reserve(service, Instant::now());
        if !wait.is_zero() {
            std::thread::sleep(wait);
        }
        let url = format!("{}{}", self.base(service), req.path);
        self.transport.send(&url, req).map_err(|e| match e {
            TransportError::Offline(detail) => Error::Offline { service, detail },
            TransportError::Timeout => Error::Timeout { service },
        })
    }

    /// Log a failed schema check of `service`'s answer to `req`.
    pub(crate) fn log_schema(&self, service: Service, req: &Request, e: &Error) {
        if let Error::Schema { detail, .. } = e {
            (self.log)(&format!("{service} answer to {} failed the schema check: {detail}", req.path));
        }
    }

    /// Cached when `ttl` is set: a stored answer younger than `ttl` seconds is served without a
    /// request. A fresh answer is parsed (the schema check) before it is cached. A cache that
    /// can't be read or written is logged and treated as a miss.
    pub fn fetch_with<T>(
        &self,
        service: Service,
        req: &Request,
        ttl: Option<i64>,
        parse: impl Fn(&str) -> Result<T>,
    ) -> Result<Fetched<T>> {
        let key = req.cache_key(service.key());
        let now = self.clock.now_s();
        if ttl.is_some() {
            // A cache that can't be read is a miss: the network may still answer.
            let stored = self.cache().get(&key, now).unwrap_or_else(|e| {
                (self.log)(&format!("{service} answer not read from the remote data cache: {e}"));
                None
            });
            if let Some((at, body)) = stored {
                // A stored body that no longer parses (the types changed) is refetched.
                if let Ok(value) = parse(&body) {
                    return Ok(Fetched { value, fetched_at: at, cached: true });
                }
            }
        }
        let body = self.send(service, req)?;
        let value = parse(&body).inspect_err(|e| self.log_schema(service, req, e))?;
        if let Some(ttl) = ttl {
            if let Err(e) = self.cache().put(&key, now, ttl, &body) {
                (self.log)(&format!("{service} answer not saved to the remote data cache: {e}"));
            }
        }
        Ok(Fetched { value, fetched_at: now, cached: false })
    }

    /// [`Client::fetch_with`] using [`parse_json`].
    pub fn fetch<T: DeserializeOwned>(&self, service: Service, req: &Request, ttl: Option<i64>) -> Result<Fetched<T>> {
        self.fetch_with(service, req, ttl, |b| parse_json(service, b))
    }
}

/// A 2xx body as-is; any other status as an [`Error`].
pub(crate) fn status_result(service: Service, resp: Response) -> Result<String> {
    match resp.status {
        200..=299 => Ok(resp.body),
        429 => Err(Error::RateLimited { service }),
        404 => Err(Error::NotFound {
            service,
            message: error_message(&resp.body).unwrap_or_else(|| "no such page".into()),
        }),
        400..=499 => Err(Error::Rejected {
            service,
            message: error_message(&resp.body).unwrap_or_else(|| format!("HTTP {}", resp.status)),
        }),
        status => Err(Error::Unavailable { service, status }),
    }
}

/// The schema check: the body must deserialize into `T`. A body that is an error object
/// (`{"error": ...}`, as Spansh and Ardent send) is [`Error::Rejected`].
pub fn parse_json<T: DeserializeOwned>(service: Service, body: &str) -> Result<T> {
    let v: serde_json::Value =
        serde_json::from_str(body).map_err(|e| Error::Schema { service, detail: format!("not JSON: {e}") })?;
    if let Some(message) = error_object(&v) {
        return Err(Error::Rejected { service, message });
    }
    serde_json::from_value(v).map_err(|e| Error::Schema { service, detail: e.to_string() })
}

fn error_object(v: &serde_json::Value) -> Option<String> {
    let o = v.as_object()?;
    let e = o.get("error")?.as_str()?;
    Some(o.get("message").and_then(|m| m.as_str()).unwrap_or(e).to_string())
}

pub(crate) fn error_message(body: &str) -> Option<String> {
    error_object(&serde_json::from_str(body).ok()?)
}
