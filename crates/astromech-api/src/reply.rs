//! A core command's reply that says itself whether it worked: `{"ok": true, …}` with the
//! command's fields, or `{"ok": false, "error": "…"}`. This is the second of two error paths,
//! and it is kept on purpose (astromech-se44): a call that fails outright is the control API's
//! envelope `{"ok": false, "error"}`, while these commands report their own failure inside the
//! reply's value, as the window's `.ok` checks read it today.

use serde::Serialize;
use ts_rs::TS;

/// `{"ok": true, …T}` or `{"ok": false, "error": "…"}`.
#[derive(Debug, Clone, PartialEq, Serialize, TS)]
#[serde(untagged)]
pub enum Answer<T> {
    Done(Done<T>),
    Failed(Failed),
}

/// `{"ok": true}` and the fields of `T`.
#[derive(Debug, Clone, PartialEq, Serialize, TS)]
pub struct Done<T> {
    #[ts(type = "true")]
    ok: bool,
    #[serde(flatten)]
    pub value: T,
}

/// `{"ok": false, "error": "…"}`.
#[derive(Debug, Clone, PartialEq, Serialize, TS)]
pub struct Failed {
    #[ts(type = "false")]
    ok: bool,
    pub error: String,
}

/// No fields: `{}`, or in an [`Answer`] nothing besides `ok`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, serde::Deserialize, TS)]
pub struct Nothing {}

impl<T> Answer<T> {
    pub fn ok(value: T) -> Self {
        Answer::Done(Done::new(value))
    }

    pub fn failed(error: impl std::fmt::Display) -> Self {
        Answer::Failed(failed(error))
    }
}

impl<T> Done<T> {
    /// `{"ok": true}` and `value`'s fields, for a reply that can't fail.
    pub fn new(value: T) -> Self {
        Done { ok: true, value }
    }
}

impl Done<Nothing> {
    /// `{"ok": true}`.
    pub fn empty() -> Self {
        Done::new(Nothing {})
    }
}

/// `{"ok": false, "error": "…"}` on its own, for a reply whose only failure is this.
pub fn failed(error: impl std::fmt::Display) -> Failed {
    Failed { ok: false, error: error.to_string() }
}

impl<T, E: std::fmt::Display> From<Result<T, E>> for Answer<T> {
    fn from(r: Result<T, E>) -> Self {
        match r {
            Ok(v) => Answer::ok(v),
            Err(e) => Answer::failed(e),
        }
    }
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)
mod tests {
    use super::*;
    use serde_json::json;

    #[derive(Serialize, TS)]
    struct Path {
        path: String,
    }

    #[test]
    fn the_json_is_ok_and_the_fields_or_ok_false_and_the_error() {
        let v = |a: &Answer<Path>| serde_json::to_value(a).unwrap();
        assert_eq!(v(&Answer::ok(Path { path: "x".into() })), json!({"ok": true, "path": "x"}));
        assert_eq!(v(&Answer::failed("no")), json!({"ok": false, "error": "no"}));
        assert_eq!(serde_json::to_value(Done::empty()).unwrap(), json!({"ok": true}));
        assert_eq!(serde_json::to_value(Answer::ok(Nothing {})).unwrap(), json!({"ok": true}));
        let cfg = crate::bindings::config();
        assert_eq!(Done::<Path>::decl(&cfg), "type Done<T> = { ok: true, } & T;");
    }
}
