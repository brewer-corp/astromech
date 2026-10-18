//! Signed updates from GitHub Releases (V1-DESIGN.md §7.3).
//!
//! The core fetches `latest.json`, checks an ed25519 signature, downloads a zip of the
//! install tree and verifies SHA-256. Apply quits this process (drops keys and the
//! session), a helper in `%TEMP%` overlays the files, then optionally relaunches. That
//! works mid-session: the new process attaches to Elite the same way a cold start does.
//! Default policy is check + download; apply waits for a click (or `apply = "on_quit"`
//! when astromech itself quits).

use anyhow::{bail, Context, Result};
use ed25519_dalek::{Signature, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// 32-byte ed25519 public key (hex) for GitHub Release `latest.json`.
/// Private seed is repo secret `ASTROMECH_UPDATE_KEY`.
pub const PUBLIC_KEY_HEX: &str = "a3d477d762b6c7b2b87a2b092310e3150cc6eafc12ce02c52d8dcf53bd3827ac";

pub const DEFAULT_FEED: &str = "https://github.com/spadaval/astromech/releases/latest/download/latest.json";
pub const CHECK_EVERY: Duration = Duration::from_secs(24 * 60 * 60);
pub const STARTUP_DELAY: Duration = Duration::from_secs(20);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplyWhen {
    #[default]
    Ask,
    OnQuit,
    Never,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub check: bool,
    pub apply: ApplyWhen,
    /// Full URL of `latest.json`. Empty: GitHub latest.
    pub feed: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self { check: true, apply: ApplyWhen::Ask, feed: String::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Manifest {
    pub version: String,
    #[serde(default)]
    pub notes: String,
    pub zip: String,
    pub sha256: String,
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    #[default]
    Idle,
    Checking,
    Available,
    Downloading,
    Ready,
    Applying,
    Error,
}

impl Status {
    /// A check, download or install is running (a second check would do nothing).
    pub fn busy(self) -> bool {
        matches!(self, Status::Checking | Status::Downloading | Status::Applying)
    }

    /// The tray's "Check for updates" line and whether it can be clicked now.
    pub fn tray_check_item(self) -> (&'static str, bool) {
        match self {
            Status::Checking => ("Checking for updates…", false),
            Status::Downloading => ("Downloading the update…", false),
            Status::Applying => ("Installing the update…", false),
            _ => ("Check for updates", true),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Snapshot {
    pub status: Status,
    pub installed: String,
    pub latest: Option<String>,
    pub notes: String,
    pub error: Option<String>,
    pub last_check: Option<String>,
    pub ready: bool,
    /// Why the previous install failed (from the helper); kept until this process exits.
    pub apply_error: Option<String>,
}

#[derive(Debug)]
pub struct Handle {
    pub snap: Snapshot,
    pub payload: Option<PathBuf>,
}

impl Handle {
    pub fn new() -> Self {
        Self { snap: Snapshot { installed: installed_version().into(), ..Default::default() }, payload: None }
    }
}

impl Default for Handle {
    fn default() -> Self {
        Self::new()
    }
}

pub fn installed_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn feed_url(s: &Settings) -> String {
    let t = s.feed.trim();
    if t.is_empty() {
        DEFAULT_FEED.into()
    } else {
        t.to_string()
    }
}

pub fn newer(latest: &str, installed: &str) -> bool {
    match (parse_ver(latest), parse_ver(installed)) {
        (Some(a), Some(b)) => a > b,
        _ => latest.trim_start_matches('v') != installed.trim_start_matches('v') && !latest.is_empty(),
    }
}

/// A version in semver order: a pre-release (`1.0.0-alpha.1`) sorts below its release, and
/// pre-release identifiers compare numerically when both are numbers, else as text, with
/// numbers first. Build metadata (`+...`) is ignored.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Ver {
    core: (u64, u64, u64),
    release: bool,
    pre: Vec<PreId>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum PreId {
    Num(u64),
    Text(String),
}

fn parse_ver(s: &str) -> Option<Ver> {
    let s = s.trim().trim_start_matches('v');
    let s = s.split('+').next()?;
    let (core, pre) = match s.split_once('-') {
        Some((c, p)) => (c, Some(p)),
        None => (s, None),
    };
    let mut p = core.split('.');
    let a = p.next()?.parse().ok()?;
    let b = p.next()?.parse().ok()?;
    let c = p.next().unwrap_or("0").parse().ok()?;
    if p.next().is_some() {
        return None;
    }
    let pre = match pre {
        None => Vec::new(),
        Some(p) => p
            .split('.')
            .map(|id| match id {
                "" => None,
                _ => Some(id.parse().map_or_else(|_| PreId::Text(id.to_string()), PreId::Num)),
            })
            .collect::<Option<Vec<_>>>()?,
    };
    Some(Ver { core: (a, b, c), release: pre.is_empty(), pre })
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn unhex(s: &str) -> Result<Vec<u8>> {
    let s = s.trim();
    if !s.len().is_multiple_of(2) {
        bail!("odd hex length");
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| anyhow::anyhow!("hex: {e}")))
        .collect()
}

pub fn public_key() -> Result<VerifyingKey> {
    public_key_from_hex(PUBLIC_KEY_HEX)
}

pub fn public_key_from_hex(h: &str) -> Result<VerifyingKey> {
    let b = unhex(h)?;
    let arr: [u8; 32] = b.as_slice().try_into().map_err(|_| anyhow::anyhow!("public key must be 32 bytes"))?;
    VerifyingKey::from_bytes(&arr).map_err(|e| anyhow::anyhow!("public key: {e}"))
}

pub fn verify_manifest_bytes(pk: &VerifyingKey, json: &[u8], sig_hex: &str) -> Result<()> {
    let sig_b = unhex(sig_hex)?;
    let arr: [u8; 64] = sig_b.as_slice().try_into().map_err(|_| anyhow::anyhow!("signature must be 64 bytes"))?;
    let sig = Signature::from_bytes(&arr);
    pk.verify_strict(json, &sig).context("latest.json signature")?;
    Ok(())
}

pub fn parse_manifest(json: &[u8]) -> Result<Manifest> {
    serde_json::from_slice(json).context("latest.json")
}

pub fn zip_url(m: &Manifest) -> String {
    if !m.url.trim().is_empty() {
        return m.url.clone();
    }
    format!("https://github.com/spadaval/astromech/releases/download/v{}/{}", m.version.trim_start_matches('v'), m.zip)
}

pub fn sha256_file(path: &Path) -> Result<String> {
    let mut f = File::open(path).with_context(|| format!("open {}", path.display()))?;
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(hex(&h.finalize()))
}

pub fn sign_bytes(secret_hex: &str, msg: &[u8]) -> Result<String> {
    use ed25519_dalek::Signer;
    let b = unhex(secret_hex)?;
    let arr: [u8; 32] = b.as_slice().try_into().map_err(|_| anyhow::anyhow!("secret key must be 32 bytes"))?;
    let sk = ed25519_dalek::SigningKey::from_bytes(&arr);
    Ok(hex(&sk.sign(msg).to_bytes()))
}

pub fn verifying_hex_from_secret(secret_hex: &str) -> Result<String> {
    let b = unhex(secret_hex)?;
    let arr: [u8; 32] = b.as_slice().try_into().map_err(|_| anyhow::anyhow!("secret key must be 32 bytes"))?;
    let sk = ed25519_dalek::SigningKey::from_bytes(&arr);
    Ok(hex(sk.verifying_key().as_bytes()))
}

/// HTTP GET as bytes. `User-Agent` is required by GitHub.
pub fn http_get(url: &str) -> Result<Vec<u8>> {
    let resp = ureq::AgentBuilder::new()
        .try_proxy_from_env(true)
        .timeout_connect(Duration::from_secs(20))
        .timeout_read(Duration::from_secs(120))
        .user_agent(concat!("astromech/", env!("CARGO_PKG_VERSION")))
        .build()
        .get(url)
        .call()
        .with_context(|| format!("GET {url}"))?;
    let mut v = Vec::new();
    resp.into_reader().read_to_end(&mut v).with_context(|| format!("read {url}"))?;
    Ok(v)
}

pub fn http_download(url: &str, dest: &Path, mut progress: impl FnMut(u64, u64)) -> Result<()> {
    let resp = ureq::AgentBuilder::new()
        .try_proxy_from_env(true)
        .timeout_connect(Duration::from_secs(20))
        .timeout_read(Duration::from_secs(120))
        .user_agent(concat!("astromech/", env!("CARGO_PKG_VERSION")))
        .build()
        .get(url)
        .call()
        .with_context(|| format!("GET {url}"))?;
    let total = resp.header("Content-Length").and_then(|s| s.parse().ok()).unwrap_or(0);
    if let Some(d) = dest.parent() {
        fs::create_dir_all(d)?;
    }
    let mut f = File::create(dest).with_context(|| format!("create {}", dest.display()))?;
    let mut r = resp.into_reader();
    let mut buf = vec![0u8; 64 * 1024];
    let mut done = 0u64;
    loop {
        let n = r.read(&mut buf)?;
        if n == 0 {
            break;
        }
        f.write_all(&buf[..n])?;
        done += n as u64;
        progress(done, total);
    }
    Ok(())
}

pub fn extract_zip(zip_path: &Path, dest: &Path) -> Result<()> {
    let f = File::open(zip_path)?;
    let mut z = zip::ZipArchive::new(f).context("open update zip")?;
    fs::create_dir_all(dest)?;
    for i in 0..z.len() {
        let mut ent = z.by_index(i)?;
        let Some(name) = ent.enclosed_name() else {
            bail!("zip path escapes");
        };
        if name.components().next().map(|c| c.as_os_str() == "data").unwrap_or(false) {
            continue;
        }
        let out = dest.join(&name);
        if ent.is_dir() {
            fs::create_dir_all(&out)?;
            continue;
        }
        if let Some(p) = out.parent() {
            fs::create_dir_all(p)?;
        }
        let mut w = File::create(&out)?;
        std::io::copy(&mut ent, &mut w)?;
    }
    Ok(())
}

/// Copy payload files into the install directory. Leaves `data\` and `uninstall.exe` alone
/// unless the payload itself contains an uninstaller.
pub fn copy_payload(from: &Path, install_dir: &Path) -> Result<()> {
    for e in fs::read_dir(from).with_context(|| format!("list {}", from.display()))? {
        let e = e?;
        let name = e.file_name();
        if name == "data" {
            continue;
        }
        let src = e.path();
        let dst = install_dir.join(&name);
        if src.is_dir() {
            copy_dir(&src, &dst)?;
        } else {
            fs::copy(&src, &dst).with_context(|| format!("copy {} → {}", src.display(), dst.display()))?;
        }
    }
    Ok(())
}

fn copy_dir(from: &Path, to: &Path) -> Result<()> {
    fs::create_dir_all(to)?;
    for e in fs::read_dir(from)? {
        let e = e?;
        let src = e.path();
        let dst = to.join(e.file_name());
        if src.is_dir() {
            copy_dir(&src, &dst)?;
        } else {
            fs::copy(&src, &dst)?;
        }
    }
    Ok(())
}

pub fn wait_pid(pid: u32, timeout: Duration) -> Result<()> {
    if pid == 0 {
        return Ok(());
    }
    #[cfg(windows)]
    {
        use windows_sys::Win32::Foundation::{CloseHandle, WAIT_OBJECT_0, WAIT_TIMEOUT};
        use windows_sys::Win32::System::Threading::{OpenProcess, WaitForSingleObject, PROCESS_SYNCHRONIZE};
        unsafe {
            let h = OpenProcess(PROCESS_SYNCHRONIZE, 0, pid);
            if h.is_null() {
                return Ok(());
            }
            let ms = timeout.as_millis().min(u32::MAX as u128) as u32;
            let w = WaitForSingleObject(h, ms);
            CloseHandle(h);
            if w == WAIT_OBJECT_0 || w == WAIT_TIMEOUT {
                // timeout: still try the copy; the original may have exited without signalling
                return Ok(());
            }
        }
        Ok(())
    }
    #[cfg(not(windows))]
    {
        let t0 = std::time::Instant::now();
        let proc = PathBuf::from(format!("/proc/{pid}"));
        while proc.exists() {
            if t0.elapsed() > timeout {
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        let _ = pid;
        Ok(())
    }
}

/// Start the installed core again. `open_window`: also open the app window (it was open when
/// the update started), otherwise tray only.
pub fn relaunch(install_dir: &Path, open_window: bool) -> Result<()> {
    let exe = install_dir.join(if cfg!(windows) { "astromech-core.exe" } else { "astromech-core" });
    let mut c = std::process::Command::new(&exe);
    if open_window {
        c.arg("--route").arg(WINDOW_ROUTE);
    } else {
        c.arg("--background");
    }
    c.current_dir(install_dir);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const DETACHED: u32 = 0x00000008;
        const NEW_GROUP: u32 = 0x00000200;
        const NO_WINDOW: u32 = 0x08000000;
        c.creation_flags(DETACHED | NEW_GROUP | NO_WINDOW);
    }
    c.spawn().with_context(|| format!("relaunch {}", exe.display()))?;
    Ok(())
}

/// The page the window reopens at after an update (where the update status is).
pub const WINDOW_ROUTE: &str = "settings";

/// What the apply helper does once the files are in place.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum After {
    /// Apply on quit: leave astromech off.
    Exit,
    /// Start the core again, tray only.
    Relaunch,
    /// Start the core again and open the app window.
    RelaunchWithWindow,
}

/// Spawn a copy of this exe in %TEMP% that waits for us, copies the payload, and optionally relaunches.
pub fn spawn_apply(payload: &Path, install_dir: &Path, after: After) -> Result<()> {
    let src = std::env::current_exe().context("current exe")?;
    let name = if cfg!(windows) {
        format!("astromech-apply-{}.exe", std::process::id())
    } else {
        format!("astromech-apply-{}", std::process::id())
    };
    let tmp = std::env::temp_dir().join(name);
    fs::copy(&src, &tmp).with_context(|| format!("stage helper {}", tmp.display()))?;
    let mut c = std::process::Command::new(&tmp);
    c.arg("--apply-update")
        .arg("--from")
        .arg(payload)
        .arg("--install-dir")
        .arg(install_dir)
        .arg("--wait-pid")
        .arg(std::process::id().to_string());
    match after {
        After::Exit => {}
        After::Relaunch => {
            c.arg("--relaunch");
        }
        After::RelaunchWithWindow => {
            c.arg("--relaunch").arg("--route").arg(WINDOW_ROUTE);
        }
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const DETACHED: u32 = 0x00000008;
        const NEW_GROUP: u32 = 0x00000200;
        const NO_WINDOW: u32 = 0x08000000;
        c.creation_flags(DETACHED | NEW_GROUP | NO_WINDOW);
    }
    c.spawn().context("start update helper")?;
    Ok(())
}

fn stop_locked_exes() {
    #[cfg(windows)]
    {
        for name in ["astromech-shell.exe", "astromech-voice-worker.exe", "astromech-core.exe"] {
            let _ = std::process::Command::new("taskkill")
                .args(["/F", "/T", "/IM", name])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status();
        }
    }
}

/// Where the helper leaves the reason a copy failed, for the next core to show.
pub fn apply_error_file(app_dir: &Path) -> PathBuf {
    app_dir.join("updates").join("apply-error.txt")
}

/// The `--apply-update` helper. Logs every step (it has no window), and relaunches even when
/// the copy failed so astromech isn't left off; the error is left in [`apply_error_file`].
pub fn apply_cli(from: &Path, install_dir: &Path, wait: Option<u32>, after: After, app_dir: &Path) -> i32 {
    use crate::logging::log;
    log(&format!("update helper: applying {} to {}", from.display(), install_dir.display()));
    if let Some(pid) = wait {
        let _ = wait_pid(pid, Duration::from_secs(45));
    }
    stop_locked_exes();
    let mut last = None;
    for _ in 0..12 {
        match copy_payload(from, install_dir) {
            Ok(()) => {
                last = None;
                break;
            }
            Err(e) => {
                last = Some(e);
                std::thread::sleep(Duration::from_millis(500));
            }
        }
    }
    let err_file = apply_error_file(app_dir);
    let mut code = 0;
    match &last {
        None => {
            log("update helper: files copied");
            let _ = fs::remove_file(&err_file);
        }
        Some(e) => {
            log(&format!("update helper: copy failed: {e:#}"));
            if let Some(d) = err_file.parent() {
                let _ = fs::create_dir_all(d);
            }
            let _ = fs::write(&err_file, format!("{e:#}"));
            code = 1;
        }
    }
    if after != After::Exit {
        match relaunch(install_dir, after == After::RelaunchWithWindow) {
            Ok(()) => log("update helper: astromech restarted"),
            Err(e) => {
                log(&format!("update helper: relaunch failed: {e:#}"));
                code = 1;
            }
        }
    }
    code
}

pub fn fetch_and_verify(settings: &Settings, pk: &VerifyingKey) -> Result<(Manifest, Vec<u8>)> {
    let url = feed_url(settings);
    let json = http_get(&url)?;
    let sig = http_get(&format!("{url}.sig"))?;
    let sig_hex = String::from_utf8_lossy(&sig);
    verify_manifest_bytes(pk, &json, &sig_hex)?;
    let m = parse_manifest(&json)?;
    Ok((m, json))
}

/// Download + extract if `m` is newer. Returns the payload directory.
pub fn stage(m: &Manifest, work: &Path, mut progress: impl FnMut(&str)) -> Result<PathBuf> {
    let dir = work.join(&m.version);
    let payload = dir.join("payload");
    let marker = dir.join("ok");
    if marker.is_file() && payload.is_dir() {
        return Ok(payload);
    }
    fs::create_dir_all(&dir)?;
    let zip_path = dir.join(&m.zip);
    progress("downloading");
    http_download(&zip_url(m), &zip_path, |_, _| {})?;
    progress("verifying");
    let sum = sha256_file(&zip_path)?;
    if sum != m.sha256.to_ascii_lowercase() {
        fs::remove_file(&zip_path).ok();
        bail!("zip sha256 {sum} != {}", m.sha256);
    }
    if payload.exists() {
        fs::remove_dir_all(&payload).ok();
    }
    progress("extracting");
    extract_zip(&zip_path, &payload)?;
    fs::write(&marker, &m.sha256)?;
    Ok(payload)
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_tray_check_item_follows_the_update_status() {
        for s in [Status::Idle, Status::Available, Status::Ready, Status::Error] {
            assert_eq!((s.tray_check_item(), s.busy()), (("Check for updates", true), false), "{s:?}");
        }
        for s in [Status::Checking, Status::Downloading, Status::Applying] {
            let (text, enabled) = s.tray_check_item();
            assert!(!enabled && text.ends_with('…') && s.busy(), "{s:?}");
        }
    }

    use super::*;
    use ed25519_dalek::Signer;

    fn keypair() -> (ed25519_dalek::SigningKey, VerifyingKey) {
        let sk = ed25519_dalek::SigningKey::from_bytes(&[7u8; 32]);
        let pk = sk.verifying_key();
        (sk, pk)
    }

    #[test]
    fn version_order() {
        assert!(newer("0.1.2", "0.1.1"));
        assert!(newer("v0.2.0", "0.1.9"));
        assert!(!newer("0.1.1", "0.1.1"));
        assert!(!newer("0.1.0", "0.1.1"));
        // Pre-release tags (v1.0.0-alpha.1) sort below their release, in semver order: 0.1.5 is
        // offered the alpha, the alpha is offered later alphas and 1.0.0, and nothing older or equal
        // is ever offered (the updater installs whatever this says is newer).
        assert!(newer("v1.0.0-alpha.1", "0.1.5"));
        assert!(newer("1.0.0-alpha.2", "1.0.0-alpha.1"));
        assert!(newer("1.0.0-alpha.10", "1.0.0-alpha.2"));
        assert!(newer("1.0.0-beta.1", "1.0.0-alpha.9"));
        assert!(newer("1.0.0-alpha.beta", "1.0.0-alpha.1"));
        assert!(!newer("1.0.0-alpha.1", "1.0.0-alpha.beta"));
        assert!(newer("1.0.0-alpha.1", "1.0.0-alpha"));
        assert!(newer("1.0.0", "1.0.0-alpha.1"));
        assert!(newer("1.0.1-alpha.1", "1.0.0"));
        assert!(!newer("1.0.0-alpha.1", "1.0.0-alpha.1"));
        assert!(!newer("v1.0.0-alpha.1", "1.0.0-alpha.1"));
        assert!(!newer("0.1.5", "1.0.0-alpha.1"));
        assert!(!newer("1.0.0-alpha.1", "1.0.0-alpha.2"));
        assert!(!newer("1.0.0-alpha.1", "1.0.0"));
        assert!(!newer("1.0.0+build.2", "1.0.0"));
    }

    #[test]
    fn signs_and_verifies_manifest() {
        let (sk, pk) = keypair();
        let json = br#"{"version":"0.1.2","notes":"x","zip":"a.zip","sha256":"ab"}"#;
        let sig = hex(&sk.sign(json).to_bytes());
        verify_manifest_bytes(&pk, json, &sig).unwrap();
        assert!(verify_manifest_bytes(&pk, json, &hex(&[0u8; 64])).is_err());
        let m = parse_manifest(json).unwrap();
        assert_eq!(m.version, "0.1.2");
        assert!(zip_url(&m).ends_with("/a.zip"));
    }

    #[test]
    fn copy_skips_data_dir() {
        let d = tempfile::tempdir().unwrap();
        let from = d.path().join("from");
        let to = d.path().join("to");
        fs::create_dir_all(from.join("data")).unwrap();
        fs::write(from.join("astromech-core.exe"), b"new").unwrap();
        fs::write(from.join("data").join("keep.txt"), b"nope").unwrap();
        fs::create_dir_all(to.join("data")).unwrap();
        fs::write(to.join("data").join("user.toml"), b"mine").unwrap();
        fs::write(to.join("uninstall.exe"), b"nsis").unwrap();
        copy_payload(&from, &to).unwrap();
        assert_eq!(fs::read(to.join("astromech-core.exe")).unwrap(), b"new");
        assert_eq!(fs::read(to.join("data").join("user.toml")).unwrap(), b"mine");
        assert!(!to.join("data").join("keep.txt").exists());
        assert_eq!(fs::read(to.join("uninstall.exe")).unwrap(), b"nsis");
    }

    #[test]
    fn zip_roundtrip_and_hash() {
        let d = tempfile::tempdir().unwrap();
        let payload = d.path().join("in");
        fs::create_dir_all(&payload).unwrap();
        fs::write(payload.join("astromech-shell.exe"), b"shell").unwrap();
        let zip_path = d.path().join("u.zip");
        {
            let f = File::create(&zip_path).unwrap();
            let mut z = zip::ZipWriter::new(f);
            z.start_file("astromech-shell.exe", zip::write::SimpleFileOptions::default()).unwrap();
            z.write_all(b"shell").unwrap();
            z.finish().unwrap();
        }
        let out = d.path().join("out");
        extract_zip(&zip_path, &out).unwrap();
        assert_eq!(fs::read(out.join("astromech-shell.exe")).unwrap(), b"shell");
        assert_eq!(sha256_file(&zip_path).unwrap().len(), 64);
    }

    #[test]
    fn baked_public_key_parses() {
        assert_eq!(PUBLIC_KEY_HEX.len(), 64);
        assert_ne!(PUBLIC_KEY_HEX, "0000000000000000000000000000000000000000000000000000000000000000");
        public_key().unwrap();
    }
}
