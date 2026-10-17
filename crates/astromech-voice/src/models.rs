//! Finding the speech models on disk (same layout as the S9 tool: `<base>/models/<archive stem>/`).

use crate::catalog::{self, Asset};
use crate::download::{self, find_file, Progress};
use crate::proto::Request;
use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

pub const STT: Asset = catalog::STT_PARAKEET_V2_INT8;
pub const TTS: Asset = catalog::TTS_KOKORO;
pub const TTS_SMALL: Asset = catalog::TTS_KOKORO_INT8;

pub struct SttPaths {
    pub encoder: PathBuf,
    pub decoder: PathBuf,
    pub joiner: PathBuf,
    pub tokens: PathBuf,
}

pub struct TtsPaths {
    pub model: PathBuf,
    pub voices: PathBuf,
    pub tokens: PathBuf,
    pub data_dir: PathBuf,
}

/// `<models_dir>/<stem>` where `models_dir` is the folder that holds the extracted archives.
pub fn model_root(models_dir: &Path, a: &Asset) -> PathBuf {
    models_dir.join(a.stem())
}

fn pick(root: &Path, prefix: &str) -> Result<PathBuf> {
    find_file(root, &|n| n.starts_with(prefix) && n.ends_with(".int8.onnx"))
        .or_else(|| find_file(root, &|n| n.starts_with(prefix) && n.ends_with(".onnx")))
        .ok_or_else(|| anyhow!("no {prefix}*.onnx under {}", root.display()))
}

pub fn stt_paths(root: &Path) -> Result<SttPaths> {
    Ok(SttPaths {
        encoder: pick(root, "encoder")?,
        decoder: pick(root, "decoder")?,
        joiner: pick(root, "joiner")?,
        tokens: find_file(root, &|n| n == "tokens.txt")
            .ok_or_else(|| anyhow!("no tokens.txt under {}", root.display()))?,
    })
}

pub fn tts_paths(root: &Path) -> Result<TtsPaths> {
    let model = find_file(root, &|n| n.starts_with("model") && n.ends_with(".onnx"))
        .ok_or_else(|| anyhow!("no model*.onnx under {}", root.display()))?;
    let dir = model.parent().unwrap_or(root).to_path_buf();
    let need = |p: PathBuf| -> Result<PathBuf> {
        if p.exists() {
            Ok(p)
        } else {
            Err(anyhow!("missing {}", p.display()))
        }
    };
    Ok(TtsPaths {
        voices: need(dir.join("voices.bin"))?,
        tokens: need(dir.join("tokens.txt"))?,
        data_dir: need(dir.join("espeak-ng-data"))?,
        model,
    })
}

/// Find the speech-to-text model. Like [`find_tts`], only a completed download counts: an
/// interrupted extraction can leave all four files with a truncated encoder.
pub fn find_stt(models_dir: &Path) -> Result<SttPaths> {
    if !download::is_installed_in(models_dir, &STT) {
        return Err(anyhow!("no completely downloaded Parakeet model in {}", models_dir.display()));
    }
    stt_paths(&model_root(models_dir, &STT))
}

/// Find the TTS model: fp32 Kokoro first, then the int8 one. Only a completed download
/// counts (marker present): extraction writes straight into the model folder, so an
/// interrupted one leaves a folder with a truncated model in it.
pub fn find_tts(models_dir: &Path) -> Result<TtsPaths> {
    [TTS, TTS_SMALL]
        .iter()
        .filter(|a| download::is_installed_in(models_dir, a))
        .find_map(|a| tts_paths(&model_root(models_dir, a)).ok())
        .ok_or_else(|| anyhow!("no completely downloaded Kokoro v1.0 model in {}", models_dir.display()))
}

/// The speech models still to download into `models_dir`: Parakeet unless a completed
/// download is there, Kokoro v1.0 unless a completed download is. Someone who had only the small v0.19
/// model gets the small v1.0 one.
pub fn missing(models_dir: &Path) -> Vec<Asset> {
    let mut v = vec![];
    if find_stt(models_dir).is_err() {
        v.push(STT);
    }
    if find_tts(models_dir).is_err() {
        let [big, small] = catalog::RETIRED_TTS.map(|a| model_root(models_dir, &a).exists());
        v.push(if small && !big { TTS_SMALL } else { TTS });
    }
    v
}

/// Download, verify and extract `a` into `models_dir` (the archive goes to a `downloads`
/// folder next to it, where a manually downloaded archive is also picked up).
pub fn download(models_dir: &Path, a: &Asset, progress: &mut dyn FnMut(Progress)) -> Result<()> {
    let dl = models_dir.parent().unwrap_or(models_dir).join("downloads");
    download::ensure_in(models_dir, &dl, a, progress)
}

/// Is a replaced model (Kokoro v0.19) still in `models_dir`?
pub fn has_retired(models_dir: &Path) -> bool {
    catalog::RETIRED_TTS.iter().any(|a| model_root(models_dir, a).exists())
}

/// Once Kokoro v1.0 is completely downloaded, delete the v0.19 folders (and their download
/// markers) it replaced. Does nothing while v1.0 is missing. Returns the folders removed.
/// Only for astromech's own models folder: another tool may still use a folder the user
/// pointed `models_dir` at.
pub fn remove_retired(models_dir: &Path) -> Vec<PathBuf> {
    if find_tts(models_dir).is_err() {
        return vec![];
    }
    let mut gone = vec![];
    for a in &catalog::RETIRED_TTS {
        let _ = std::fs::remove_file(download::marker_in(models_dir, a));
        let root = model_root(models_dir, a);
        if root.exists() && std::fs::remove_dir_all(&root).is_ok() {
            gone.push(root);
        }
    }
    gone
}

fn s(p: &Path) -> String {
    p.display().to_string()
}

pub fn load_stt_req(p: &SttPaths, threads: i32) -> Request {
    Request::LoadStt {
        encoder: s(&p.encoder),
        decoder: s(&p.decoder),
        joiner: s(&p.joiner),
        tokens: s(&p.tokens),
        provider: "cpu".into(),
        threads,
    }
}

pub fn load_tts_req(p: &TtsPaths, threads: i32) -> Request {
    Request::LoadTts {
        model: s(&p.model),
        voices: s(&p.voices),
        tokens: s(&p.tokens),
        data_dir: s(&p.data_dir),
        // Kokoro v1.0 is multilingual and needs a language to load; each request sets its own.
        lang: "en-us".into(),
        provider: "cpu".into(),
        threads,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finds_s9_layout() {
        let d = tempfile::tempdir().unwrap();
        let st = model_root(d.path(), &STT);
        std::fs::create_dir_all(&st).unwrap();
        for f in ["encoder.int8.onnx", "decoder.int8.onnx", "joiner.int8.onnx", "tokens.txt"] {
            std::fs::write(st.join(f), "x").unwrap();
        }
        let tt = model_root(d.path(), &TTS_SMALL);
        std::fs::create_dir_all(tt.join("espeak-ng-data")).unwrap();
        for f in ["model.int8.onnx", "voices.bin", "tokens.txt"] {
            std::fs::write(tt.join(f), "x").unwrap();
        }
        std::fs::write(download::marker_in(d.path(), &TTS_SMALL), TTS_SMALL.sha256).unwrap();
        assert!(stt_paths(&st).unwrap().encoder.ends_with("encoder.int8.onnx"));
        assert!(find_tts(d.path()).unwrap().model.ends_with("model.int8.onnx"));
        assert!(matches!(load_stt_req(&stt_paths(&st).unwrap(), 4), Request::LoadStt { threads: 4, .. }));
    }

    /// A model folder with its files; `done` also writes the marker a finished download leaves.
    fn fake_tts(models: &Path, a: &Asset, done: bool) {
        let root = model_root(models, a);
        std::fs::create_dir_all(root.join("espeak-ng-data")).unwrap();
        for f in ["model.onnx", "voices.bin", "tokens.txt"] {
            std::fs::write(root.join(f), "x").unwrap();
        }
        if done {
            std::fs::write(download::marker_in(models, a), a.sha256).unwrap();
        }
    }

    #[test]
    fn half_extracted_v1_is_not_installed() {
        let d = tempfile::tempdir().unwrap();
        let m = d.path();
        fake_tts(m, &catalog::RETIRED_TTS[0], true);
        // Interrupted first download: every v1.0 file is there, but no marker.
        fake_tts(m, &TTS, false);
        assert!(find_tts(m).is_err());
        assert_eq!(missing(m).iter().map(|a| a.id).collect::<Vec<_>>(), [STT.id, TTS.id], "fetched again");
        assert!(remove_retired(m).is_empty(), "v0.19 stays until v1.0 is complete");
        assert!(has_retired(m));
        // The download finishes.
        fake_tts(m, &TTS, true);
        assert!(find_tts(m).is_ok());
        assert_eq!(missing(m).iter().map(|a| a.id).collect::<Vec<_>>(), [STT.id]);
    }

    #[test]
    fn parakeet_files_without_the_marker_are_missing() {
        let d = tempfile::tempdir().unwrap();
        let m = d.path();
        let root = model_root(m, &STT);
        std::fs::create_dir_all(&root).unwrap();
        // Interrupted first extraction: all four files, the encoder possibly truncated, no marker.
        for f in ["encoder.int8.onnx", "decoder.int8.onnx", "joiner.int8.onnx", "tokens.txt"] {
            std::fs::write(root.join(f), "x").unwrap();
        }
        assert!(find_stt(m).is_err());
        assert!(missing(m).iter().any(|a| a.id == STT.id), "the download list offers it again");
        // The download finishes.
        std::fs::write(download::marker_in(m, &STT), STT.sha256).unwrap();
        assert!(find_stt(m).is_ok());
        assert!(!missing(m).iter().any(|a| a.id == STT.id));
    }

    #[test]
    fn small_v0_19_users_get_small_v1() {
        let d = tempfile::tempdir().unwrap();
        fake_tts(d.path(), &catalog::RETIRED_TTS[1], true);
        assert!(missing(d.path()).iter().any(|a| a.id == TTS_SMALL.id));
        fake_tts(d.path(), &catalog::RETIRED_TTS[0], true);
        assert!(missing(d.path()).iter().any(|a| a.id == TTS.id), "had the full model: full v1.0");
        let e = tempfile::tempdir().unwrap();
        assert!(missing(e.path()).iter().any(|a| a.id == TTS.id), "fresh install: full v1.0");
    }

    #[test]
    fn retired_kokoro_is_removed_only_after_v1_is_installed() {
        let d = tempfile::tempdir().unwrap();
        let m = d.path();
        // A v0.19 install: v1.0 is missing, so the old model is not usable and stays put.
        fake_tts(m, &catalog::RETIRED_TTS[0], true);
        assert!(find_tts(m).is_err(), "v0.19 no longer counts as an installed TTS model");
        assert!(has_retired(m));
        assert!(remove_retired(m).is_empty());
        assert!(model_root(m, &catalog::RETIRED_TTS[0]).is_dir());
        // v1.0 downloaded: the v0.19 folder and its marker go, v1.0 stays.
        fake_tts(m, &TTS, true);
        assert_eq!(remove_retired(m), vec![model_root(m, &catalog::RETIRED_TTS[0])]);
        assert!(!has_retired(m));
        assert!(!m.join(".tts-kokoro-en-v0_19.ok").exists());
        assert!(find_tts(m).unwrap().model.starts_with(model_root(m, &TTS)));
        assert!(m.join(".tts-kokoro-multi-lang-v1_0.ok").exists());
    }
}
