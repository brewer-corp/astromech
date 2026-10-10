//! JSON-lines protocol between the main exe and `s9-voice-worker`.
//!
//! The worker reads one [`Request`] per line on stdin and answers with exactly one
//! [`Response`] per request on stdout. Response lines start with [`RESPONSE_PREFIX`] so
//! that anything a native library prints to stdout can be told apart and ignored.

use serde::{Deserialize, Serialize};

pub const RESPONSE_PREFIX: &str = "@@S9@@ ";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum Request {
    Info,
    LoadStt {
        encoder: String,
        decoder: String,
        joiner: String,
        tokens: String,
        provider: String,
        threads: i32,
    },
    LoadTts {
        model: String,
        voices: String,
        tokens: String,
        data_dir: String,
        provider: String,
        threads: i32,
    },
    /// Transcribe a 16-bit or float WAV file (any sample rate; sherpa-onnx resamples).
    Stt { wav: String },
    /// Synthesise `text`; write the audio to `out_wav` (if not empty).
    Tts { text: String, sid: i32, speed: f32, out_wav: String },
    Quit,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Response {
    pub ok: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Info
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_sha1: Option<String>,
    /// Load*
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_ms: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_speakers: Option<i32>,
    /// Stt
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_ms: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decode_ms: Option<f64>,
    /// Stt (input length) and Tts (output length)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_ms: Option<f64>,
    /// Tts
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_audio_ms: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_ms: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rtf: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<u32>,
}

impl Response {
    pub fn err(msg: impl Into<String>) -> Self {
        Response { ok: false, error: Some(msg.into()), ..Default::default() }
    }
    pub fn to_line(&self) -> String {
        format!("{RESPONSE_PREFIX}{}", serde_json::to_string(self).expect("serialise response"))
    }
    /// Parse a worker stdout line; `None` if it is not a protocol line.
    pub fn from_line(line: &str) -> Option<Result<Self, serde_json::Error>> {
        line.trim_end().strip_prefix(RESPONSE_PREFIX).map(serde_json::from_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_roundtrip() {
        let r = Request::Tts { text: "Lights on.".into(), sid: 0, speed: 1.0, out_wav: "a.wav".into() };
        let s = serde_json::to_string(&r).unwrap();
        assert!(s.contains("\"cmd\":\"tts\""));
        assert_eq!(serde_json::from_str::<Request>(&s).unwrap(), r);
        let q: Request = serde_json::from_str(r#"{"cmd":"quit"}"#).unwrap();
        assert_eq!(q, Request::Quit);
    }

    #[test]
    fn response_lines() {
        let r = Response { ok: true, text: Some("lights on".into()), decode_ms: Some(12.5), ..Default::default() };
        let line = r.to_line();
        assert_eq!(Response::from_line(&line).unwrap().unwrap(), r);
        assert!(Response::from_line("some native log line").is_none());
        assert!(!line.contains("error"), "None fields are omitted: {line}");
    }
}
