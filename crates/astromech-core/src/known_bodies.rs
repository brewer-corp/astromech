//! Bodies known to EDSM for the arrival briefing ("12 of 30 bodies known.",
//! `astromech_announce::known_bodies`): EDSM's bodies of the system against its body count,
//! looked up in the background ([`crate::background`]) so the pipeline thread never waits on the
//! network. Through the process's one remote-data client, so EDSM's answer is cached (a week,
//! `ttl::SYSTEM`) and rate limited with everything else. A system EDSM doesn't know is "no
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
