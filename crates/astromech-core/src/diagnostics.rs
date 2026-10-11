//! "Save diagnostics" (docs/V1-DESIGN.md §1.5 #2, §7.7): one zip with what's needed to debug a
//! report: the logs, the last crash, the binds health report, the config and a short system
//! summary. Everything text-like is redacted (user name, profile paths) on the way in.

use anyhow::{Context, Result};
use astromech_keybinds::report::Redactor;
use std::io::Write;
use std::path::{Path, PathBuf};

/// What goes in, besides the log folder.
pub struct Inputs<'a> {
    pub app_dir: &'a Path,
    pub data_dir: &'a Path,
    pub log_dir: &'a Path,
    pub config: Option<&'a Path>,
    /// Extra lines for `system.txt` (core status).
    pub status: String,
}

/// Write `<app_dir>/diagnostics/astromech-diagnostics-<stamp>.zip` and return its path.
pub fn save(inp: &Inputs) -> Result<PathBuf> {
    let red = Redactor::from_env();
    let out_dir = inp.app_dir.join("diagnostics");
    std::fs::create_dir_all(&out_dir)?;
    let out = out_dir.join(format!("astromech-diagnostics-{}.zip", chrono::Local::now().format("%Y%m%d-%H%M%S")));
    let f = std::fs::File::create(&out).with_context(|| format!("creating {}", out.display()))?;
    let mut z = zip::ZipWriter::new(f);
    let opt = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let mut add = |name: &str, text: &str| -> Result<()> {
        z.start_file(name, opt)?;
        z.write_all(red.apply(text).as_bytes())?;
        Ok(())
    };
    add(
        "system.txt",
        &format!(
            "astromech-core {}\nos: {} {}\ntime: {}\n\n{}\n",
            env!("CARGO_PKG_VERSION"),
            std::env::consts::OS,
            std::env::consts::ARCH,
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S %:z"),
            inp.status
        ),
    )?;
    if let Ok(rd) = std::fs::read_dir(inp.log_dir) {
        let mut logs: Vec<PathBuf> =
            rd.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|e| e == "log")).collect();
        logs.sort();
        for p in logs {
            if let Ok(t) = std::fs::read_to_string(&p) {
                add(&format!("logs/{}", p.file_name().unwrap_or_default().to_string_lossy()), &t)?;
            }
        }
    }
    let mut singles: Vec<(String, PathBuf)> = vec![
        ("crash.marker".into(), inp.app_dir.join(crate::crash::MARKER)),
        ("crash-last.txt".into(), inp.app_dir.join(crate::crash::LAST)),
        ("health.json".into(), inp.data_dir.join("health.json")),
        ("worker-stt.log".into(), inp.data_dir.join("worker-stt.log")),
        ("worker-tts.log".into(), inp.data_dir.join("worker-tts.log")),
    ];
    if let Some(c) = inp.config {
        singles.push(("astromech.toml".into(), c.to_path_buf()));
    }
    for (name, p) in singles {
        if let Ok(t) = std::fs::read_to_string(&p) {
            add(&name, &t)?;
        }
    }
    z.finish()?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundles_logs_and_reports() {
        let d = tempfile::tempdir().unwrap();
        let logs = d.path().join("logs");
        std::fs::create_dir_all(&logs).unwrap();
        std::fs::write(logs.join("astromech-core.log"), "hello\n").unwrap();
        std::fs::write(d.path().join("health.json"), "{}").unwrap();
        std::fs::write(d.path().join(crate::crash::MARKER), "boom").unwrap();
        let p = save(&Inputs {
            app_dir: d.path(),
            data_dir: d.path(),
            log_dir: &logs,
            config: None,
            status: "idle".into(),
        })
        .unwrap();
        let mut z = zip::ZipArchive::new(std::fs::File::open(p).unwrap()).unwrap();
        let mut names: Vec<String> = (0..z.len()).map(|i| z.by_index(i).unwrap().name().unwrap().to_string()).collect();
        names.sort();
        assert_eq!(names, ["crash.marker", "health.json", "logs/astromech-core.log", "system.txt"]);
    }
}
