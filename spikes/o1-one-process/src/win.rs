//! What both modes share: options, the spike's folder and log, the single-instance handover,
//! astromech's Raw Input listener as a key counter, dark menus, restart registration and the
//! crash kinds.

use crate::{app, control, plain};
use serde_json::{json, Value};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;
use std::time::Instant;

pub static STARTED: OnceLock<Instant> = OnceLock::new();
/// Key-downs astromech's listener has seen.
pub static SINK_KEYS: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Sink {
    None,
    BeforeBuild,
    AfterBuild,
}

#[derive(Clone, Debug)]
pub struct Opts {
    pub background: bool,
    pub route: Option<String>,
    pub plain_tray: bool,
    pub sink: Sink,
    pub crash_hook: bool,
    pub restarted: bool,
}

impl Opts {
    fn parse(args: &[String]) -> Opts {
        let has = |f: &str| args.iter().any(|a| a == f);
        let route = args.iter().position(|a| a == "--route").and_then(|i| args.get(i + 1)).cloned();
        let sink = if has("--sink-before-build") {
            Sink::BeforeBuild
        } else if has("--sink-after-build") {
            Sink::AfterBuild
        } else {
            Sink::None
        };
        Opts {
            background: has("--background"),
            route,
            plain_tray: has("--plain-tray"),
            sink,
            crash_hook: has("--crash-hook"),
            restarted: has("--restarted"),
        }
    }
}

/// `%LOCALAPPDATA%\astromech-spike`, read from the environment so a probe can point it at a
/// scratch folder.
pub fn dir() -> PathBuf {
    let base = std::env::var_os("LOCALAPPDATA").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
    base.join("astromech-spike")
}

pub fn log(line: &str) {
    let ms = STARTED.get().map_or(0, |t| t.elapsed().as_millis());
    let _ = std::fs::create_dir_all(dir());
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(dir().join("spike.log")) {
        let _ = writeln!(f, "[{} +{ms}ms] {line}", std::process::id());
    }
}

pub fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Start astromech's own listener (crates/astromech-input) and count the key-downs it sees.
pub fn start_sink(when: &str) {
    let (tx, rx) = std::sync::mpsc::channel();
    match astromech_input::listen::win::rawinput::start(tx, Instant::now()) {
        Ok(()) => log(&format!("sink: raw input listener registered {when}")),
        Err(e) => log(&format!("sink: raw input listener failed {when}: {e}")),
    }
    std::thread::Builder::new()
        .name("sink-count".into())
        .spawn(move || {
            use astromech_input::listen::model::SampleData;
            use astromech_input::listen::win::Msg;
            for m in rx {
                if let Msg::Sample(s) = m {
                    if let SampleData::Key { down: true, .. } = s.data {
                        SINK_KEYS.fetch_add(1, Ordering::SeqCst);
                    }
                }
            }
        })
        .expect("sink thread");
}

/// Dark context menus, as crates/astromech-core/src/tray.rs:35-54 does: uxtheme ordinals 135
/// (`SetPreferredAppMode`) and 136 (`FlushMenuThemes`).
pub fn dark_menus() {
    use windows_sys::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
    type Proc = unsafe extern "system" fn() -> isize;
    const FORCE_DARK: i32 = 2;
    let name = wide("uxtheme.dll");
    unsafe {
        let h = LoadLibraryW(name.as_ptr());
        if h.is_null() {
            return;
        }
        if let Some(f) = GetProcAddress(h, 135usize as *const u8) {
            let set_mode = std::mem::transmute::<Proc, unsafe extern "system" fn(i32) -> i32>(f);
            set_mode(FORCE_DARK);
        }
        if let Some(f) = GetProcAddress(h, 136usize as *const u8) {
            let flush = std::mem::transmute::<Proc, unsafe extern "system" fn()>(f);
            flush();
        }
    }
}

/// Windows' restart registration: after a crash (with at least 60 s of uptime) WER starts us
/// again with these arguments.
fn register_restart() {
    use windows_sys::Win32::System::Recovery::{RegisterApplicationRestart, RESTART_NO_PATCH, RESTART_NO_REBOOT};
    let args = wide("--background --restarted");
    let hr = unsafe { RegisterApplicationRestart(args.as_ptr(), RESTART_NO_PATCH | RESTART_NO_REBOOT) };
    log(&format!("restart: RegisterApplicationRestart returned 0x{hr:08X}"));
    log_crash_env();
}

/// What a parent hands down that can keep WER away from a crash: the error mode
/// (SEM_NOGPFAULTERRORBOX = 0x2 skips WER) and a job object's limits
/// (JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION = 0x400 ends the process without WER).
fn log_crash_env() {
    use windows_sys::Win32::System::Diagnostics::Debug::GetErrorMode;
    use windows_sys::Win32::System::JobObjects::{
        IsProcessInJob, JobObjectExtendedLimitInformation, QueryInformationJobObject,
        JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
    };
    use windows_sys::Win32::System::Threading::GetCurrentProcess;
    unsafe {
        let mode = GetErrorMode();
        let mut in_job = 0;
        IsProcessInJob(GetCurrentProcess(), std::ptr::null_mut(), &mut in_job);
        let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = std::mem::zeroed();
        let ok = QueryInformationJobObject(
            std::ptr::null_mut(),
            JobObjectExtendedLimitInformation,
            &mut info as *mut _ as *mut _,
            std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
            std::ptr::null_mut(),
        );
        let flags = if ok != 0 { format!("0x{:X}", info.BasicLimitInformation.LimitFlags) } else { "none".into() };
        log(&format!("restart: error mode 0x{mode:X}, in a job: {}, job limit flags {flags}", in_job != 0));
    }
}

/// The `crash` command's kinds, other than the main-thread panic (the Tauri mode posts that to
/// its main thread). Runs on a fresh thread after the reply has gone out.
pub fn crash_soon(kind: &str) -> Result<(), String> {
    let kind = kind.to_string();
    if kind != "abort" && kind != "access_violation" {
        return Err(format!("unknown crash kind {kind}"));
    }
    log(&format!("crash: {kind} in 300 ms"));
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(300));
        if kind == "abort" {
            std::process::abort();
        }
        // A write through a null pointer the optimiser can't see: an access violation
        // (0xC0000005) on this worker thread.
        let p = std::hint::black_box(0usize) as *mut u32;
        unsafe { p.write_volatile(1) };
    });
    Ok(())
}

/// The second start: if another instance holds the mutex, hand it the route over the control
/// port and return its exit code.
fn hand_over(opts: &Opts) -> Option<i32> {
    use windows_sys::Win32::Foundation::{GetLastError, ERROR_ALREADY_EXISTS};
    use windows_sys::Win32::System::Threading::CreateMutexW;
    let name = wide("Local\\astromech-spike-o1");
    let h = unsafe { CreateMutexW(std::ptr::null(), 0, name.as_ptr()) };
    if h.is_null() || unsafe { GetLastError() } != ERROR_ALREADY_EXISTS {
        // Ours: the handle stays open for the life of the process.
        return None;
    }
    let args = json!({ "route": opts.route });
    let r = control::call("open_window", Some(args));
    log(&format!("second start: open_window {:?} answered {r:?}", opts.route));
    Some(if r.is_ok() { 0 } else { 1 })
}

pub fn main() {
    STARTED.get_or_init(Instant::now);
    let args: Vec<String> = std::env::args().collect();
    let opts = Opts::parse(&args);
    log(&format!("start: {}", args[1..].join(" ")));
    if opts.plain_tray {
        plain::run(&opts);
        return;
    }
    if let Some(code) = hand_over(&opts) {
        std::process::exit(code);
    }
    if opts.crash_hook {
        register_restart();
    }
    app::run(opts);
}

/// The `ui_state` fields both modes share.
pub fn base_state() -> Value {
    json!({ "sink_keys": SINK_KEYS.load(Ordering::SeqCst), "pid": std::process::id() })
}
