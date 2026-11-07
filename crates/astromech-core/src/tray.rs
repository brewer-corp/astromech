//! What the tray icon's native menu shows (docs/V1-DESIGN.md §7.1, §2.2): a remote for the
//! core while you play. Status line, Open astromech, Pause voice, Kill switch (while active),
//! Check for updates, Quit, plus one line each when something needs attention (a crash,
//! unfinished setup, an update ready to install). Everything else lives in the window.
//!
//! This is the menu as data: [`read`] takes it from the core, and [`items`] turns it into the
//! list of items in order. The Tauri host (app/src-tauri) shows that list with Tauri's tray and
//! runs the item a click picks; left-clicking the icon opens the window.

use crate::app::Core;
use crate::session::VOICE_PAUSED;
use astromech_api::Recover;
use std::sync::atomic::Ordering;

/// Which optional lines the menu has; the host rebuilds the menu when this changes, and
/// otherwise only updates the items' texts and states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shape {
    pub crash: bool,
    pub setup: bool,
    pub active: bool,
    pub update: bool,
}

/// Everything the menu shows, read from the core at one moment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Menu {
    pub shape: Shape,
    /// The status line, also the tooltip's second line.
    pub status: String,
    pub setup: String,
    pub install: String,
    pub check: (&'static str, bool),
    /// The kill switch is on (the Kill item says Re-arm).
    pub killed: bool,
    pub paused: bool,
}

/// A menu item's identity, which is also its Tauri menu id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Id {
    Status,
    Crash,
    Setup,
    Open,
    Pause,
    Kill,
    Install,
    Check,
    Quit,
}

impl Id {
    pub const ALL: [Id; 9] =
        [Id::Status, Id::Crash, Id::Setup, Id::Open, Id::Pause, Id::Kill, Id::Install, Id::Check, Id::Quit];

    pub fn as_str(self) -> &'static str {
        match self {
            Id::Status => "status",
            Id::Crash => "crash",
            Id::Setup => "setup",
            Id::Open => "open",
            Id::Pause => "pause",
            Id::Kill => "kill",
            Id::Install => "install",
            Id::Check => "check",
            Id::Quit => "quit",
        }
    }

    pub fn parse(s: &str) -> Option<Id> {
        Id::ALL.into_iter().find(|i| i.as_str() == s)
    }
}

/// One line of the menu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Separator,
    Text { id: Id, text: String, enabled: bool },
    Check { id: Id, text: String, checked: bool },
}

pub const CRASH: &str = "astromech crashed last time: save diagnostics…";
pub const OPEN: &str = "Open astromech";
pub const PAUSE: &str = "Pause voice";
pub const KILL: &str = "Kill switch (stop all actions)";
pub const REARM: &str = "Re-arm (kill switch is on)";
pub const QUIT: &str = "Quit astromech";

/// The tray icon's tooltip.
pub fn tooltip(status: &str) -> String {
    format!("astromech\n{status}")
}

fn setup_text(core: &Core) -> String {
    match core.setup_needed().first() {
        Some(what) => format!("Finish setup: {what}…"),
        None => "Finish setup…".into(),
    }
}

fn update_text(core: &Core) -> String {
    match &core.update.lock().recover().snap.latest {
        Some(v) => format!("Restart to update to {v}"),
        None => "Restart to update".into(),
    }
}

/// The menu as the core is now. Each of the core's locks is taken and released in turn; none is
/// held when this returns, so the host can make its menu calls after it.
pub fn read(core: &Core) -> Menu {
    let shape = Shape {
        crash: core.crash.lock().recover().is_some(),
        setup: !core.setup_needed().is_empty(),
        active: core.active(),
        update: core.update.lock().recover().snap.ready,
    };
    let killed = core.session.lock().recover().as_ref().map(|s| s.runner.policy.is_killed()).unwrap_or(false);
    let check = core.update.lock().recover().snap.status.tray_check_item();
    Menu {
        shape,
        status: core.status_line(),
        setup: setup_text(core),
        install: update_text(core),
        check,
        killed,
        paused: VOICE_PAUSED.load(Ordering::SeqCst),
    }
}

/// The menu's lines in order: the status line, then the crash and setup lines when they apply,
/// Open, Pause voice, the kill switch while a session runs, Restart to update when one is
/// ready, Check for updates and Quit.
pub fn items(m: &Menu) -> Vec<Item> {
    let text = |id, text: &str, enabled| Item::Text { id, text: text.to_string(), enabled };
    let mut v = vec![text(Id::Status, &m.status, false)];
    if m.shape.crash {
        v.push(text(Id::Crash, CRASH, true));
    }
    if m.shape.setup {
        v.push(text(Id::Setup, &m.setup, true));
    }
    v.extend([Item::Separator, text(Id::Open, OPEN, true), Item::Separator]);
    v.push(Item::Check { id: Id::Pause, text: PAUSE.into(), checked: m.paused });
    if m.shape.active {
        v.push(text(Id::Kill, if m.killed { REARM } else { KILL }, true));
    }
    v.push(Item::Separator);
    if m.shape.update {
        v.push(text(Id::Install, &m.install, true));
    }
    v.push(text(Id::Check, m.check.0, m.check.1));
    v.push(text(Id::Quit, QUIT, true));
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn menu(crash: bool, setup: bool, active: bool, update: bool) -> Menu {
        Menu {
            shape: Shape { crash, setup, active, update },
            status: "Elite not running".into(),
            setup: "Finish setup: push-to-talk not set…".into(),
            install: "Restart to update to 1.2.3".into(),
            check: ("Check for updates", true),
            killed: false,
            paused: false,
        }
    }

    /// Each line as `id text` (`-` for a separator, `[x]`/`[ ]` for the check item, `(off)` for
    /// a disabled one), so a test reads like the menu.
    fn lines(m: &Menu) -> Vec<String> {
        items(m)
            .into_iter()
            .map(|i| match i {
                Item::Separator => "-".into(),
                Item::Text { id, text, enabled } => {
                    format!("{} {text}{}", id.as_str(), if enabled { "" } else { " (off)" })
                }
                Item::Check { id, text, checked } => {
                    format!("{} [{}] {text}", id.as_str(), if checked { "x" } else { " " })
                }
            })
            .collect()
    }

    #[test]
    fn idle_with_nothing_to_do_has_the_fixed_lines_only() {
        assert_eq!(
            lines(&menu(false, false, false, false)),
            [
                "status Elite not running (off)",
                "-",
                "open Open astromech",
                "-",
                "pause [ ] Pause voice",
                "-",
                "check Check for updates",
                "quit Quit astromech",
            ]
        );
    }

    #[test]
    fn every_optional_line_in_its_place() {
        assert_eq!(
            lines(&menu(true, true, true, true)),
            [
                "status Elite not running (off)",
                "crash astromech crashed last time: save diagnostics…",
                "setup Finish setup: push-to-talk not set…",
                "-",
                "open Open astromech",
                "-",
                "pause [ ] Pause voice",
                "kill Kill switch (stop all actions)",
                "-",
                "install Restart to update to 1.2.3",
                "check Check for updates",
                "quit Quit astromech",
            ]
        );
    }

    #[test]
    fn each_optional_line_alone() {
        let only = |crash, setup, active, update| {
            let all = lines(&menu(false, false, false, false));
            lines(&menu(crash, setup, active, update)).into_iter().filter(|l| !all.contains(l)).collect::<Vec<_>>()
        };
        assert_eq!(only(true, false, false, false), ["crash astromech crashed last time: save diagnostics…"]);
        assert_eq!(only(false, true, false, false), ["setup Finish setup: push-to-talk not set…"]);
        assert_eq!(only(false, false, true, false), ["kill Kill switch (stop all actions)"]);
        assert_eq!(only(false, false, false, true), ["install Restart to update to 1.2.3"]);
    }

    #[test]
    fn states_show_in_the_items() {
        let mut m = menu(false, false, true, false);
        m.killed = true;
        m.paused = true;
        m.check = ("Checking for updates…", false);
        let l = lines(&m);
        assert!(l.contains(&"kill Re-arm (kill switch is on)".to_string()), "{l:?}");
        assert!(l.contains(&"pause [x] Pause voice".to_string()), "{l:?}");
        assert!(l.contains(&"check Checking for updates… (off)".to_string()), "{l:?}");
        assert_eq!(tooltip(&m.status), "astromech\nElite not running");
    }

    #[test]
    fn ids_round_trip() {
        for id in Id::ALL {
            assert_eq!(Id::parse(id.as_str()), Some(id));
        }
        assert_eq!(Id::parse("nope"), None);
    }
}
