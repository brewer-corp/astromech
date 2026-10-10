//! Finding the speech models on disk (same layout as the S9 tool: `<base>/models/<archive stem>/`).

use crate::catalog::{self, Asset};
use crate::download::find_file;
use crate::proto::Request;
use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

pub const STT: Asset = catalog::STT_PARAKEET_V2_INT8;
pub const TTS: Asset = catalog::TTS_KOKORO_EN;
pub const TTS_SMALL: Asset = catalog::TTS_KOKORO_EN_INT8;

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

/// Find the TTS model: fp32 Kokoro first, then the int8 one.
pub fn find_tts(models_dir: &Path) -> Result<TtsPaths> {
    tts_paths(&model_root(models_dir, &TTS)).or_else(|_| tts_paths(&model_root(models_dir, &TTS_SMALL)))
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
        assert!(stt_paths(&st).unwrap().encoder.ends_with("encoder.int8.onnx"));
        assert!(find_tts(d.path()).unwrap().model.ends_with("model.int8.onnx"));
        assert!(matches!(load_stt_req(&stt_paths(&st).unwrap(), 4), Request::LoadStt { threads: 4, .. }));
    }
}
