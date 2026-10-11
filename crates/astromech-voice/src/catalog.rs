//! What gets downloaded on first run. Sizes and SHA-256 digests come from the GitHub
//! release-asset metadata of k2-fsa/sherpa-onnx (checked 2026-10-09).

pub const SHERPA_VERSION: &str = "1.13.8";
const BASE: &str = "https://github.com/k2-fsa/sherpa-onnx/releases/download";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// sherpa-onnx + onnxruntime shared libraries (Linux reference runs only; the Windows
    /// worker links them statically).
    Runtime,
    /// A single DLL taken out of an official NuGet package (the optional DirectML test).
    GpuRuntime,
    Stt,
    Tts,
}

#[derive(Debug, Clone, Copy)]
pub struct Asset {
    pub id: &'static str,
    pub kind: Kind,
    /// Release tag the file lives under (sherpa-onnx GitHub releases).
    pub tag: &'static str,
    /// Archive name, or for a zip member the name the extracted file is saved as.
    pub file: &'static str,
    /// Bytes downloaded (for a zip member: its compressed size).
    pub size: u64,
    /// SHA-256 of the archive, or for a zip member of the extracted file.
    pub sha256: &'static str,
    /// Folder (relative to the exe folder) the archive is extracted into.
    pub dest: &'static str,
    pub label: &'static str,
    /// Full URL when the file is not a sherpa-onnx GitHub release asset.
    pub url: Option<&'static str>,
    /// Take only this member out of the zip at `url` (fetched with HTTP range requests,
    /// so only that file is downloaded, not the whole package).
    pub zip_member: Option<ZipMember>,
}

#[derive(Debug, Clone, Copy)]
pub struct ZipMember {
    pub path: &'static str,
    pub uncompressed_size: u64,
}

/// Field defaults for the asset constants below.
pub const BLANK: Asset = Asset {
    id: "",
    kind: Kind::Stt,
    tag: "",
    file: "",
    size: 0,
    sha256: "",
    dest: "",
    label: "",
    url: None,
    zip_member: None,
};

impl Asset {
    pub fn url(&self) -> String {
        match self.url {
            Some(u) => u.to_string(),
            None => format!("{BASE}/{}/{}", self.tag, self.file),
        }
    }
    /// Archive name without `.tar.bz2`; also the top-level folder inside the archive.
    pub fn stem(&self) -> &'static str {
        self.file.strip_suffix(".tar.bz2").unwrap_or(self.file)
    }
    /// The path that must exist for the asset to count as installed (besides its marker).
    pub fn installed_path(&self) -> &'static str {
        if self.zip_member.is_some() {
            self.file
        } else {
            self.stem()
        }
    }
    /// Archive members we skip when extracting (demo programs and headers in runtime archives).
    pub fn skip_member(&self, path: &str) -> bool {
        self.kind == Kind::Runtime && (path.contains("/bin/") || path.contains("/include/"))
    }
}

pub const RUNTIME_CPU_LINUX: Asset = Asset {
    id: "runtime-cpu-linux-x64",
    kind: Kind::Runtime,
    tag: "v1.13.8",
    file: "sherpa-onnx-v1.13.8-linux-x64-shared-lib.tar.bz2",
    size: 9_816_899,
    sha256: "3892d184be41027e18165e67f549cd4e4cdd8dcd73ac5579e97afd55e14e30b6",
    dest: "runtimes/cpu",
    label: "sherpa-onnx 1.13.8 + onnxruntime, CPU (Linux x64, reference runs only)",
    ..BLANK
};

/// Microsoft's official ONNX Runtime build with the DirectML execution provider
/// (NuGet Microsoft.ML.OnnxRuntime.DirectML 1.24.4, MIT licence). Optional GPU test only.
pub const ORT_DIRECTML_WIN: Asset = Asset {
    id: "ort-directml-1.24.4-win-x64",
    kind: Kind::GpuRuntime,
    file: "onnxruntime.dll",
    size: 6_039_111,
    sha256: "e7eedec6a6f26dc39dc948276a75ef6d2bee3fff944d874ceed0bbd3b97bff40",
    dest: "runtimes/directml",
    label: "ONNX Runtime 1.24.4 with DirectML (Microsoft, NuGet)",
    url: Some("https://api.nuget.org/v3-flatcontainer/microsoft.ml.onnxruntime.directml/1.24.4/microsoft.ml.onnxruntime.directml.1.24.4.nupkg"),
    zip_member: Some(ZipMember { path: "runtimes/win-x64/native/onnxruntime.dll", uncompressed_size: 17_328_152 }),
    ..BLANK
};

/// DirectML 1.15.4 redistributable (NuGet Microsoft.AI.DirectML; Microsoft licence that
/// allows use with apps on Windows). Optional GPU test only.
pub const DIRECTML_WIN: Asset = Asset {
    id: "directml-1.15.4-win-x64",
    kind: Kind::GpuRuntime,
    file: "DirectML.dll",
    size: 9_332_741,
    sha256: "9c9e6d822561c6c41b90e6994b3e8857cf1d66dbfb1e0c4c799c7c89b4e92da1",
    dest: "runtimes/directml",
    label: "DirectML 1.15.4 (Microsoft, NuGet)",
    url: Some("https://api.nuget.org/v3-flatcontainer/microsoft.ai.directml/1.15.4/microsoft.ai.directml.1.15.4.nupkg"),
    zip_member: Some(ZipMember { path: "bin/x64-win/DirectML.dll", uncompressed_size: 18_527_776 }),
    ..BLANK
};

pub const STT_PARAKEET_V2_INT8: Asset = Asset {
    id: "stt-parakeet-tdt-0.6b-v2-int8",
    kind: Kind::Stt,
    tag: "asr-models",
    file: "sherpa-onnx-nemo-parakeet-tdt-0.6b-v2-int8.tar.bz2",
    size: 482_468_385,
    sha256: "157c157bc51155e03e37d2466522a3a737dd9c72bb25f36eb18912964161e1ad",
    dest: "models",
    label: "Parakeet TDT 0.6B v2 (English), int8",
    ..BLANK
};

pub const TTS_KOKORO_EN: Asset = Asset {
    id: "tts-kokoro-en-v0_19",
    kind: Kind::Tts,
    tag: "tts-models",
    file: "kokoro-en-v0_19.tar.bz2",
    size: 319_625_534,
    sha256: "912804855a04745fa77a30be545b3f9a5d15c4d66db00b88cbcd4921df605ac7",
    dest: "models",
    label: "Kokoro English v0.19 (fp32)",
    ..BLANK
};

pub const TTS_KOKORO_EN_INT8: Asset = Asset {
    id: "tts-kokoro-int8-en-v0_19",
    kind: Kind::Tts,
    tag: "tts-models",
    file: "kokoro-int8-en-v0_19.tar.bz2",
    size: 103_248_205,
    sha256: "c9f0dd393615805b0bab050c340834d5e684e732aec91c0e860cd30e982c08bd",
    dest: "models",
    label: "Kokoro English v0.19 (int8, smaller)",
    ..BLANK
};

/// Shared runtime the worker needs at run time: none on Windows (sherpa-onnx and
/// onnxruntime are linked into s9-voice-worker.exe), the shared libraries on Linux.
pub fn cpu_runtime() -> Option<Asset> {
    if cfg!(windows) {
        None
    } else {
        Some(RUNTIME_CPU_LINUX)
    }
}

pub fn gpu_runtime() -> [Asset; 2] {
    [ORT_DIRECTML_WIN, DIRECTML_WIN]
}

pub fn mb(bytes: u64) -> f64 {
    bytes as f64 / 1_000_000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_and_digests() {
        for a in
            [RUNTIME_CPU_LINUX, ORT_DIRECTML_WIN, DIRECTML_WIN, STT_PARAKEET_V2_INT8, TTS_KOKORO_EN, TTS_KOKORO_EN_INT8]
        {
            assert!(a.url().starts_with("https://"), "{}", a.id);
            assert_eq!(a.sha256.len(), 64, "{}", a.id);
            assert!(a.sha256.chars().all(|c| c.is_ascii_hexdigit()));
            assert!(!a.label.is_empty() && !a.dest.is_empty() && a.size > 0);
        }
        assert!(STT_PARAKEET_V2_INT8
            .url()
            .starts_with("https://github.com/k2-fsa/sherpa-onnx/releases/download/asr-models/"));
        assert_eq!(STT_PARAKEET_V2_INT8.stem(), "sherpa-onnx-nemo-parakeet-tdt-0.6b-v2-int8");
        assert_eq!(DIRECTML_WIN.installed_path(), "DirectML.dll");
        assert!(ORT_DIRECTML_WIN.url().starts_with("https://api.nuget.org/"));
        assert!(RUNTIME_CPU_LINUX.skip_member("x/bin/sherpa-onnx"));
        assert!(!RUNTIME_CPU_LINUX.skip_member("x/lib/libonnxruntime.so"));
        assert!(!TTS_KOKORO_EN.skip_member("kokoro-en-v0_19/espeak-ng-data/bin/x"));
    }
}
