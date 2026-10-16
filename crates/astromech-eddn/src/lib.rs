//! # astromech-eddn
//!
//! Sends game data to EDDN (the Elite Dangerous Data Network) the way EDCD's
//! `docs/Developers.md` (live branch) asks. Written from that spec, not from EDMC.
//!
//! - [`context`]: the header values, flags, commander and last system, folded from the journal;
//! - [`message`]: one message from journal JSON: the live-only gate, `*_Localised` and commander
//!   data stripped, per-schema elisions ([`SchemaSpec`]), the augmentation cross-check, gzip and
//!   the 1 MiB limit;
//! - [`outbox`]: the SQLite outbox with retry and backoff, and the send log Diagnostics shows;
//! - [`transport`]: the HTTP upload, behind a trait;
//! - [`companion`]: commodity/3, outfitting/2 and shipyard/2 from `Market.json`, `Outfitting.json`
//!   and `Shipyard.json`.
//!
//! Each schema slice (commodity/3, journal/1, …) defines its [`SchemaSpec`] and maps its journal
//! event to the schema's shape before calling [`Sender::submit`].

pub mod companion;
pub mod context;
pub mod message;
pub mod outbox;
pub mod transport;

#[cfg(test)]
pub(crate) mod testutil;

pub use context::{GameContext, SystemFix};
pub use message::{Augment, Dropped, Elision, SchemaSpec, Software, SystemField};
pub use outbox::{Delivery, LogEntry, Outbox, Outcome};
pub use transport::{HttpTransport, Response, Transport, UPLOAD_URL};

use serde_json::Value;

/// The outbox file in the data folder.
pub const DB_FILE: &str = "eddn.sqlite";

pub struct Sender {
    pub ctx: GameContext,
    outbox: Outbox,
    software: Software,
    /// Send to the `/test` schemas (development, and checking new schema code).
    test: bool,
}

impl Sender {
    pub fn new(outbox: Outbox, software: Software, test: bool) -> Self {
        Self { ctx: GameContext::default(), outbox, software, test }
    }

    /// Feed every journal event, in order, before submitting messages built from it.
    pub fn observe(&mut self, ev: &Value) {
        self.ctx.observe(ev);
    }

    /// Build, check and queue one message. Drops are written to the send log and returned.
    pub fn submit(&self, spec: &SchemaSpec, message: Value, now: i64) -> rusqlite::Result<Result<(), Dropped>> {
        let schema_ref = message::schema_ref(spec, self.test);
        match message::build(spec, &self.ctx, &self.software, self.test, message).and_then(|up| message::encode(&up)) {
            Ok(body) => {
                self.outbox.enqueue(&schema_ref, &body, now)?;
                Ok(Ok(()))
            }
            Err(d) => {
                self.outbox.log(now, &schema_ref, Outcome::Dropped, &d.to_string())?;
                Ok(Err(d))
            }
        }
    }

    /// One delivery pass over everything due at `now`.
    pub fn deliver(&self, transport: &dyn Transport, now: i64) -> rusqlite::Result<Delivery> {
        self.outbox.deliver_due(transport, now)
    }

    pub fn outbox(&self) -> &Outbox {
        &self.outbox
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::outbox::tests::{status, FakeTransport};
    use serde_json::json;

    const SPEC: SchemaSpec = SchemaSpec { name: "journal", version: 1, elide: &[], augment: &[], flags: true };

    fn sender(test: bool) -> Sender {
        let mut s = Sender::new(
            Outbox::in_memory().unwrap(),
            Software { name: "astromech".into(), version: "0.1.5".into() },
            test,
        );
        s.observe(&json!({"event":"Fileheader","gameversion":"4.0.0.1904","build":"r1/r0 "}));
        s.observe(&json!({"event":"LoadGame","Commander":"Jameson","Horizons":true,"Odyssey":true}));
        s
    }

    #[test]
    fn submitted_message_is_posted_gzipped_to_the_test_schema() {
        let s = sender(true);
        let ev = json!({"timestamp":"2026-10-16T12:00:00Z","event":"Docked","StationName_Localised":"x","FID":"F1"});
        assert_eq!(s.submit(&SPEC, ev, 100).unwrap(), Ok(()));
        let t = FakeTransport::new([status(200)]);
        assert_eq!(s.deliver(&t, 100).unwrap().sent, 1);
        let up = message::gunzip(&t.posted.borrow()[0]);
        assert_eq!(up["$schemaRef"], "https://eddn.edcd.io/schemas/journal/1/test");
        assert_eq!(
            up["message"],
            json!({"timestamp":"2026-10-16T12:00:00Z","event":"Docked","horizons":true,"odyssey":true})
        );
        let log = s.outbox().recent(10).unwrap();
        assert_eq!((log[0].outcome.as_str(), log[0].schema_ref.as_str()), ("sent", up["$schemaRef"].as_str().unwrap()));
    }

    #[test]
    fn beta_messages_are_never_queued_or_sent() {
        let mut s = sender(false);
        s.observe(&json!({"event":"Fileheader","gameversion":"4.1.0.100 BETA","build":"r2/r0"}));
        assert!(matches!(s.submit(&SPEC, json!({"event":"Docked"}), 100).unwrap(), Err(Dropped::Beta(_))));
        assert_eq!(s.outbox().pending().unwrap(), 0);
        let t = FakeTransport::default();
        assert_eq!(s.deliver(&t, 100).unwrap(), Delivery::default());
        assert_eq!(t.calls(), 0);
        let log = s.outbox().recent(1).unwrap();
        assert_eq!(log[0].outcome, "dropped");
        assert!(log[0].detail.contains("beta"), "{}", log[0].detail);
    }

    #[test]
    fn message_over_one_mib_gzipped_is_dropped_and_logged() {
        let s = sender(false);
        let mut x: u64 = 0x2545_F491_4F6C_DD1D;
        let noise: Vec<String> = (0..(1024 * 1024 / 8) * 2)
            .map(|_| {
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                format!("{:08x}", x as u32)
            })
            .collect();
        let r = s.submit(&SPEC, json!({"event":"Docked","Items":noise}), 100).unwrap();
        assert!(matches!(r, Err(Dropped::TooLarge(n)) if n > message::MAX_BODY), "{r:?}");
        assert_eq!(s.outbox().pending().unwrap(), 0);
        let log = s.outbox().recent(1).unwrap();
        assert_eq!(log[0].outcome, "dropped");
        assert!(log[0].detail.contains("over the 1048576-byte limit"), "{}", log[0].detail);
    }
}
