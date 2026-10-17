//! The core's log (`astromech-log`): `<log dir>/astromech-core.log`, the same lines as the
//! console, with paths under the user's profile and the Windows user name redacted.

use astromech_keybinds::report::Redactor;
use std::path::Path;

pub use astromech_log::{dir, log};

/// Start logging into `dir` (created if needed). `console`: also print each line.
pub fn init(dir: &Path, console: bool) {
    let red = Redactor::from_env();
    astromech_log::init(dir, astromech_log::CORE, console, Box::new(move |l: &str| red.apply(l)));
}
