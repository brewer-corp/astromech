//! # astromech-proto
//!
//! The typed API between the core and every client (app window, browser, VR overlay, voice
//! tools): queries, commands, subscription events and errors. See `docs/ARCHITECTURE.md` §18.
//!
//! **Rule:** this crate depends on nothing internal. Later it gains `specta`/`schemars` derives
//! for TypeScript generation (`app/src/lib/api`).

use serde::{Deserialize, Serialize};

/// Protocol version; bump on breaking changes to any message.
pub const PROTOCOL_VERSION: u32 = 1;

/// Where a request came from. Policy uses this to pick tiers and rate limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Origin {
    Voice,
    AppWindow,
    Dashboard,
    Overlay,
    Cloud,
}

/// Risk tier of a command (§5.14). Tier 0 is read-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Tier {
    /// Read-only query.
    T0,
    /// Reversible game action, e.g. landing gear.
    T1,
    /// Needs confirmation, e.g. jump, plot route.
    T2,
    /// Never automated.
    T3,
}

/// A request from a client.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Request {
    Query { id: u64, name: String, args: serde_json::Value },
    Command { id: u64, name: String, args: serde_json::Value, origin: Origin },
    Subscribe { id: u64, topic: String },
}

/// The core's answer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Response {
    Ok { id: u64, value: serde_json::Value },
    Err { id: u64, error: ApiError },
    Event { topic: String, value: serde_json::Value },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum ApiError {
    UnknownName { name: String },
    BadArgs { message: String },
    Denied { reason: String },
    NeedsConfirmation { prompt: String },
    Failed { message: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn request_roundtrip() {
        let r = Request::Command {
            id: 7,
            name: "ship.landing_gear".into(),
            args: serde_json::json!({"down": true}),
            origin: Origin::Voice,
        };
        let s = serde_json::to_string(&r).unwrap();
        assert!(s.contains(r#""kind":"command""#) && s.contains(r#""origin":"voice""#));
        assert_eq!(serde_json::from_str::<Request>(&s).unwrap(), r);
    }
    #[test]
    fn tiers_order() {
        assert!(Tier::T0 < Tier::T2);
    }
}
