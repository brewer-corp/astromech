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

/// Kokoro v1.0 (sherpa-onnx `kokoro-multi-lang-v1_0`, checked 2026-10-16): 54 voices, 8 of them
/// British. Extracted it takes about 401 MB (model.onnx 326 MB, voices.bin 28 MB, espeak-ng-data,
/// English and Chinese lexicons).
pub const TTS_KOKORO: Asset = Asset {
    id: "tts-kokoro-multi-lang-v1_0",
    kind: Kind::Tts,
    tag: "tts-models",
    file: "kokoro-multi-lang-v1_0.tar.bz2",
    size: 349_906_910,
    sha256: "c5f7e2d2caf082bc1d20fb70334a61d99d20b484500aad32e7cf84c128ea3298",
    dest: "models",
    label: "Kokoro v1.0 (fp32)",
    ..BLANK
};

/// The int8 build of [`TTS_KOKORO`]: same voices (identical voices.bin), about 190 MB extracted.
pub const TTS_KOKORO_INT8: Asset = Asset {
    id: "tts-kokoro-int8-multi-lang-v1_0",
    kind: Kind::Tts,
    tag: "tts-models",
    file: "kokoro-int8-multi-lang-v1_0.tar.bz2",
    size: 132_303_094,
    sha256: "4c3052abaa60943a341f193888cf6abd68787dae6ab8ae5c925a706caa247e4e",
    dest: "models",
    label: "Kokoro v1.0 (int8, smaller)",
    ..BLANK
};

/// Kokoro v0.19, replaced by v1.0. Listed only so an update can remove its folders.
pub const RETIRED_TTS: [Asset; 2] = [
    Asset {
        id: "tts-kokoro-en-v0_19",
        kind: Kind::Tts,
        tag: "tts-models",
        file: "kokoro-en-v0_19.tar.bz2",
        size: 319_625_534,
        sha256: "912804855a04745fa77a30be545b3f9a5d15c4d66db00b88cbcd4921df605ac7",
        dest: "models",
        label: "Kokoro English v0.19 (fp32)",
        ..BLANK
    },
    Asset {
        id: "tts-kokoro-int8-en-v0_19",
        kind: Kind::Tts,
        tag: "tts-models",
        file: "kokoro-int8-en-v0_19.tar.bz2",
        size: 103_248_205,
        sha256: "c9f0dd393615805b0bab050c340834d5e684e732aec91c0e860cd30e982c08bd",
        dest: "models",
        label: "Kokoro English v0.19 (int8, smaller)",
        ..BLANK
    },
];

/// Kokoro v1.0 voices; the index is the speaker ID (`sid`). Read from the `speaker_names`
/// metadata of both v1.0 model files. The prefix is language and sex: `a` American English,
/// `b` British English, `e` Spanish, `f` French, `h` Hindi, `i` Italian, `j` Japanese,
/// `p` Brazilian Portuguese, `z` Mandarin; then `f` female, `m` male.
pub const KOKORO_VOICES: [&str; 54] = [
    "af_alloy",
    "af_aoede",
    "af_bella",
    "af_heart",
    "af_jessica",
    "af_kore",
    "af_nicole",
    "af_nova", // 0-7
    "af_river",
    "af_sarah",
    "af_sky",
    "am_adam",
    "am_echo",
    "am_eric",
    "am_fenrir",
    "am_liam", // 8-15
    "am_michael",
    "am_onyx",
    "am_puck",
    "am_santa",
    "bf_alice",
    "bf_emma",
    "bf_isabella",
    "bf_lily", // 16-23
    "bm_daniel",
    "bm_fable",
    "bm_george",
    "bm_lewis",
    "ef_dora",
    "em_alex",
    "ff_siwis",
    "hf_alpha", // 24-31
    "hf_beta",
    "hm_omega",
    "hm_psi",
    "if_sara",
    "im_nicola",
    "jf_alpha",
    "jf_gongitsune",
    "jf_nezumi", // 32-39
    "jf_tebukuro",
    "jm_kumo",
    "pf_dora",
    "pm_alex",
    "pm_santa",
    "zf_xiaobei",
    "zf_xiaoni",
    "zf_xiaoxiao", // 40-47
    "zf_xiaoyi",
    "zm_yunjian",
    "zm_yunxi",
    "zm_yunxia",
    "zm_yunyang",
    "em_santa", // 48-53
];

/// Kokoro v0.19 voices by speaker ID (its `speaker_names` metadata), for migrating old configs.
/// `af` is a blend of `af_bella` and `af_sarah` and has no v1.0 equivalent.
pub const KOKORO_V0_19_VOICES: [&str; 11] = [
    "af",
    "af_bella",
    "af_nicole",
    "af_sarah",
    "af_sky",
    "am_adam",
    "am_michael",
    "bf_emma",
    "bf_isabella",
    "bm_george",
    "bm_lewis",
];

/// The voice used when none is set: `bf_emma`, the best-graded British voice in hexgrad's
/// Kokoro-82M VOICES.md (overall B-). The voice is a Settings choice; this is only the default.
pub const DEFAULT_VOICE: &str = "bf_emma";

/// Speaker ID of a v1.0 voice.
pub fn voice_sid(name: &str) -> Option<i32> {
    KOKORO_VOICES.iter().position(|v| *v == name).map(|i| i as i32)
}

/// The espeak-ng voice the text is phonemised with: British voices get British English (`en`,
/// espeak-ng's "English (Great Britain)"; it has no voice named `en-gb`), every other voice
/// American English (astromech only speaks English).
pub fn voice_lang(name: &str) -> &'static str {
    if name.starts_with('b') {
        "en"
    } else {
        "en-us"
    }
}

/// The v1.0 voice for a v0.19 speaker ID, if v1.0 has the same voice.
pub fn voice_from_v0_19(sid: i32) -> Option<&'static str> {
    let name = *KOKORO_V0_19_VOICES.get(usize::try_from(sid).ok()?)?;
    KOKORO_VOICES.iter().copied().find(|v| *v == name)
}

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
        for a in [
            RUNTIME_CPU_LINUX,
            ORT_DIRECTML_WIN,
            DIRECTML_WIN,
            STT_PARAKEET_V2_INT8,
            TTS_KOKORO,
            TTS_KOKORO_INT8,
            RETIRED_TTS[0],
            RETIRED_TTS[1],
        ] {
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
        assert!(!TTS_KOKORO.skip_member("kokoro-multi-lang-v1_0/espeak-ng-data/bin/x"));
    }

    #[test]
    fn kokoro_v1_assets_and_voices() {
        assert_eq!(
            TTS_KOKORO.url(),
            "https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/kokoro-multi-lang-v1_0.tar.bz2"
        );
        assert_eq!(TTS_KOKORO.stem(), "kokoro-multi-lang-v1_0");
        assert_eq!(TTS_KOKORO_INT8.stem(), "kokoro-int8-multi-lang-v1_0");
        assert_eq!(RETIRED_TTS.map(|a| a.stem()), ["kokoro-en-v0_19", "kokoro-int8-en-v0_19"]);
        // voices.bin holds 54 style tables of 510 x 256 f32 (28,200,960 bytes).
        assert_eq!(KOKORO_VOICES.len() * 510 * 256 * 4, 28_200_960);
        let british: Vec<&str> = KOKORO_VOICES.iter().copied().filter(|v| v.starts_with('b')).collect();
        assert_eq!(
            british,
            ["bf_alice", "bf_emma", "bf_isabella", "bf_lily", "bm_daniel", "bm_fable", "bm_george", "bm_lewis"]
        );
        assert_eq!(voice_sid("af_alloy"), Some(0));
        assert_eq!(voice_sid("bf_emma"), Some(21));
        assert_eq!(voice_sid("em_santa"), Some(53));
        assert_eq!(voice_sid("af"), None);
        assert!(voice_sid(DEFAULT_VOICE).is_some());
        assert_eq!((voice_lang("bm_george"), voice_lang("af_heart")), ("en", "en-us"));
        assert_eq!(voice_from_v0_19(0), None, "af is a blend v1.0 doesn't have");
        assert_eq!(voice_from_v0_19(7), Some("bf_emma"));
        assert_eq!(voice_from_v0_19(10), Some("bm_lewis"));
        assert_eq!(voice_from_v0_19(11), None);
        assert_eq!(voice_from_v0_19(-1), None);
        for sid in 1..11 {
            assert!(voice_from_v0_19(sid).is_some(), "v0.19 speaker {sid} is in v1.0");
        }
    }
}
