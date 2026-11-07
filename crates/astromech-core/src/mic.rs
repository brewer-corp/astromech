//! The microphone, behind a boundary ([`crate::app::Boundaries`]): the push-to-talk loop's open
//! device, calibration's and the level check's recordings, and the mic picker's device names.
//! The installed app's is cpal through [`astromech_voice::mic::Recorder`] ([`DeviceMics`]); a
//! world's gives its recorded clips.

use crate::calibration::Clip;
use anyhow::Result;
use std::time::{Duration, Instant};

/// Stream errors since the last look (for the log): `astromech_voice::mic::StreamErrors`'s.
#[derive(Debug, Default, Clone)]
pub struct StreamErrors {
    /// Errors since the last look.
    pub new: u64,
    /// Buffer under- and overruns so far.
    pub xruns: u64,
    pub last: Option<String>,
}

/// An open recording device: a running count of the samples it has delivered, from which the
/// push-to-talk loop cuts each clip.
pub trait Mic: Send {
    fn device_name(&self) -> String;
    fn sample_rate(&self) -> u32;
    /// The system's default recording device when this one was opened.
    fn default_name(&self) -> Option<String>;
    /// Samples delivered so far.
    fn position(&self) -> u64;
    /// Samples `from..to` (as far as they are still held).
    fn take(&self, from: u64, to: u64) -> Vec<f32>;
    /// Wait (at most `max`) for audio delivered after `t`.
    fn wait_after(&self, t: Instant, max: Duration);
    /// Stream errors since the last call.
    fn take_errors(&self) -> Option<StreamErrors>;
    /// Restart a lost or stalled stream; true if it restarted.
    fn recover(&mut self) -> Result<bool>;
}

/// The recording devices.
pub trait Mics: Send + Sync {
    /// Open the device `want` picks (a name or part of one; None: the default).
    fn open(&self, want: Option<&str>) -> Result<Box<dyn Mic>>;
    /// Record `ms` of audio from the device the `[voice] mic` setting `want` picks
    /// (calibration and the level check).
    fn record(&self, want: &str, ms: u64) -> Result<Clip>;
    /// The recording devices' names and the default's, for the mic picker.
    fn names(&self) -> (Vec<String>, Option<String>);
}

/// The PC's microphones (Windows; elsewhere there is none to open).
pub struct DeviceMics;

impl Mics for DeviceMics {
    fn open(&self, want: Option<&str>) -> Result<Box<dyn Mic>> {
        #[cfg(windows)]
        {
            Ok(Box::new(astromech_voice::mic::Recorder::start_named(want)?))
        }
        #[cfg(not(windows))]
        {
            let _ = want;
            anyhow::bail!("recording needs Windows")
        }
    }

    fn record(&self, want: &str, ms: u64) -> Result<Clip> {
        crate::calibration::record_mic(want, ms)
    }

    fn names(&self) -> (Vec<String>, Option<String>) {
        #[cfg(windows)]
        let (names, default) =
            (astromech_voice::mic::Recorder::list_names(), astromech_voice::mic::Recorder::default_device_name());
        #[cfg(not(windows))]
        let (names, default): (Vec<String>, Option<String>) = (vec![], None);
        (names, default)
    }
}

#[cfg(windows)]
impl Mic for astromech_voice::mic::Recorder {
    fn device_name(&self) -> String {
        astromech_voice::mic::Recorder::device_name(self)
    }
    fn sample_rate(&self) -> u32 {
        astromech_voice::mic::Recorder::sample_rate(self)
    }
    fn default_name(&self) -> Option<String> {
        self.default_name.clone()
    }
    fn position(&self) -> u64 {
        astromech_voice::mic::Recorder::position(self)
    }
    fn take(&self, from: u64, to: u64) -> Vec<f32> {
        astromech_voice::mic::Recorder::take(self, from, to)
    }
    fn wait_after(&self, t: Instant, max: Duration) {
        astromech_voice::mic::Recorder::wait_after(self, t, max)
    }
    fn take_errors(&self) -> Option<StreamErrors> {
        astromech_voice::mic::Recorder::take_errors(self).map(|e| StreamErrors {
            new: e.new,
            xruns: e.xruns,
            last: e.last,
        })
    }
    fn recover(&mut self) -> Result<bool> {
        astromech_voice::mic::Recorder::recover(self)
    }
}
