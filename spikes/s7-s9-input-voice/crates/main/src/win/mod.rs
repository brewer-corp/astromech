//! Windows-only pieces: Raw Input listener, XInput poller, microphone, playback, system info.

pub mod audio;
pub mod platform;
pub mod rawinput;
pub mod sound;
pub mod sys;
pub mod xinput;

pub fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}
