//! Bodies known to EDSM for the arrival briefing ("12 of 30 bodies known.",
//! `astromech_announce::known_bodies`): EDSM's bodies of the system against its body count,
//! looked up in the background ([`crate::background`]) so the pipeline thread never waits on the
//! network. Through the process's one remote-data client, so EDSM's answer is cached (an hour,
//! `ttl::SCANNED`, as long as [`crate::background::KEEP_FOR`] keeps it) and rate limited with everything else. A system EDSM doesn't know is "no
//! bodies known"; a failed lookup (offline, EDSM down or slow) is logged and stays unknown, so
//! nothing is said.

use crate::background::Background;
use astromech_announce::known_bodies::{Known, KnownBodies};
use astromech_data::{Client, Error};
use std::sync::Arc;

/// Where failures go (the core's log).
pub type Log = Arc<dyn Fn(&str) + Send + Sync>;

/// What EDSM knows about `system`'s bodies.
pub fn look_up(client: &Client, system: &str) -> astromech_data::Result<Known> {
    match client.edsm_bodies(system) {
        Ok(f) => Ok(Known { bodies: f.value.bodies.len() as u32, total: f.value.body_count }),
        Err(Error::NotFound { .. }) => Ok(Known { bodies: 0, total: None }),
        Err(e) => Err(e),
    }
}

/// EDSM lookups for [`KnownBodies`], in the background.
pub struct EdsmBodies {
    client: Arc<Client>,
    systems: Background<Known>,
    log: Log,
}

impl EdsmBodies {
    pub fn new(client: Arc<Client>, log: Log) -> Self {
        Self { client, systems: Background::new("known-bodies"), log }
    }
}

impl KnownBodies for EdsmBodies {
    fn known(&self, system: &str) -> Option<Known> {
        let (client, log, name) = (self.client.clone(), self.log.clone(), system.to_string());
        self.systems.get(system, move || {
            look_up(&client, &name).inspect_err(|e| log(&format!("bodies known to EDSM, {name}: {e}")))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use astromech_data::{Cache, Request, Response, Transport, TransportError};

    struct Answer(u16, &'static str);
    impl Transport for Answer {
        fn send(&self, url: &str, _req: &Request) -> Result<Response, TransportError> {
            assert!(url.ends_with("/api-system-v1/bodies"), "{url}");
            Ok(Response { status: self.0, body: self.1.into() })
        }
    }

    fn client(status: u16, body: &'static str) -> Client {
        Client::with_transport(Cache::in_memory().unwrap(), Arc::new(Answer(status, body)))
    }

    /// EDSM's bodies of a system: `n` on the `n`th request.
    struct Growing(std::sync::atomic::AtomicU32);
    impl Transport for Growing {
        fn send(&self, _url: &str, _req: &Request) -> Result<Response, TransportError> {
            let n = self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
            let bodies: Vec<String> = (0..n)
                .map(|i| format!(r#"{{"bodyId":{i},"name":"B {i}","type":"Planet","distanceToArrival":10}}"#))
                .collect();
            Ok(Response {
                status: 200,
                body: format!(r#"{{"id64":5,"name":"Sys","bodyCount":9,"bodies":[{}]}}"#, bodies.join(",")),
            })
        }
    }

    struct At(Arc<std::sync::atomic::AtomicI64>);
    impl astromech_data::Clock for At {
        fn now_s(&self) -> i64 {
            self.0.load(std::sync::atomic::Ordering::SeqCst)
        }
    }

    /// astromech-yfje: an answer kept for `background::KEEP_FOR` is looked up again, and that
    /// lookup reaches EDSM: others may have scanned the system since, during a long session.
    #[test]
    fn a_count_refreshed_after_an_hour_is_edsms_new_one() {
        use std::time::Instant;
        let wall = Arc::new(std::sync::atomic::AtomicI64::new(1_800_000_000));
        let client = Client::with_transport(Cache::in_memory().unwrap(), Arc::new(Growing(0.into())))
            .clock(Arc::new(At(wall.clone())));
        let mut k = EdsmBodies::new(Arc::new(client), Arc::new(|_: &str| {}));
        let start = Instant::now();
        let now = Arc::new(std::sync::Mutex::new(start));
        let n2 = now.clone();
        k.systems.set_clock(Arc::new(move || *n2.lock().unwrap()));
        assert_eq!(k.known("Sys"), None);
        k.systems.settle("Sys");
        assert_eq!(k.known("Sys"), Some(Known { bodies: 1, total: Some(9) }));

        // Others scan the system; an hour on, the answer is looked up again.
        let keep = crate::background::KEEP_FOR;
        *now.lock().unwrap() = start + keep;
        wall.fetch_add(keep.as_secs() as i64, std::sync::atomic::Ordering::SeqCst);
        assert_eq!(k.known("Sys"), None);
        k.systems.settle("Sys");
        assert_eq!(k.known("Sys"), Some(Known { bodies: 2, total: Some(9) }), "EDSM's count now");
    }

    #[test]
    fn a_recorded_answer_is_bodies_against_the_count() {
        let recorded = include_str!("../../astromech-data/tests/fixtures/edsm-bodies.json");
        let k = look_up(&client(200, recorded), "Alpha Centauri").unwrap();
        assert_eq!(k, Known { bodies: 2, total: Some(9) });
        // EDSM's "no such system" ({} with HTTP 200): nothing known there.
        assert_eq!(look_up(&client(200, "{}"), "Nowhere").unwrap(), Known { bodies: 0, total: None });
        assert!(look_up(&client(503, ""), "Sol").is_err());
    }
}
