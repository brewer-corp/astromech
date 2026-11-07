//! The core's one way to watch the game's and the Bindings folders: [`watcher`], which drops the
//! notifications that can't be a change before they reach a clock channel. Linux reports a file
//! merely opened, or closed unwritten (notify asks inotify for opens), so without the filter a
//! watcher's own reads would wake it for ever and a world's clock would never settle. Clippy
//! rejects `notify::recommended_watcher` everywhere else (`clippy.toml`), so a new watcher
//! can't skip the filter.

use astromech_proto::ClockSender;

/// A file notification.
pub type Event = notify::Result<notify::Event>;

/// Whether a notification may be a change: not a file merely opened, or closed unwritten.
/// Windows never reports those, so there every notification passes.
pub fn may_change(e: &Event) -> bool {
    use notify::event::{AccessKind, AccessMode, EventKind};
    !matches!(
        e,
        Ok(notify::Event { kind: EventKind::Access(AccessKind::Open(_) | AccessKind::Close(AccessMode::Read)), .. })
    )
}

/// A watcher (not yet watching anything) that sends `item(event)` into `tx` for each
/// notification that [may be a change](may_change), unless `item` gives `None`. Sending into a
/// clock channel marks its receiver busy before a manual clock's `advance` can see it parked.
#[allow(clippy::disallowed_methods, reason = "the one watcher constructor, which applies the filter")]
pub fn watcher<T: Send + 'static>(
    tx: ClockSender<T>,
    item: impl Fn(Event) -> Option<T> + Send + 'static,
) -> notify::Result<notify::RecommendedWatcher> {
    notify::recommended_watcher(move |e: Event| {
        if may_change(&e) {
            if let Some(t) = item(e) {
                let _ = tx.send(t);
            }
        }
    })
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)
mod tests {
    use super::*;

    /// Opening a file, or closing it unwritten, isn't a change (Linux reports both; the
    /// watchers' own reads would wake them for ever). A close after writing is.
    #[test]
    fn a_read_is_not_a_change() {
        use notify::event::{AccessKind, AccessMode, EventKind, ModifyKind};
        let e = |k| Ok(notify::Event::new(k));
        assert!(!may_change(&e(EventKind::Access(AccessKind::Open(AccessMode::Any)))));
        assert!(!may_change(&e(EventKind::Access(AccessKind::Close(AccessMode::Read)))));
        assert!(may_change(&e(EventKind::Access(AccessKind::Close(AccessMode::Write)))));
        assert!(may_change(&e(EventKind::Modify(ModifyKind::Any))));
        assert!(may_change(&e(EventKind::Any)));
        assert!(may_change(&Err(notify::Error::generic("lost"))));
    }

    /// The real watcher on Linux, where inotify reports reads: a file read in the watched folder
    /// sends nothing, a file written there does. inotify delivers in order, so if the read's
    /// open or close got through, it would be the first item, ahead of the write's.
    #[cfg(target_os = "linux")]
    #[test]
    fn only_a_real_write_reaches_the_clock_channel() {
        use notify::{RecursiveMode, Watcher};
        use std::time::Duration;
        let d = tempfile::tempdir().unwrap();
        let (read, written) = (d.path().join("Status.json"), d.path().join("Market.json"));
        std::fs::write(&read, "{}").unwrap();
        let clock: std::sync::Arc<dyn astromech_proto::Clock> = std::sync::Arc::new(astromech_proto::SystemClock);
        let (tx, rx) = clock.channel::<Event>();
        let mut w = watcher(tx, Some).unwrap();
        w.watch(d.path(), RecursiveMode::NonRecursive).unwrap();

        assert_eq!(std::fs::read_to_string(&read).unwrap(), "{}");
        std::fs::write(&written, "{}").unwrap();

        // The 30 s bound only fails a watcher that never sends; it decides nothing.
        let first = rx.recv_timeout(Duration::from_secs(30)).expect("the write was never reported").unwrap();
        assert_eq!(first.paths, vec![written], "the first notification is the write's, not the read's: {first:?}");
    }
}
