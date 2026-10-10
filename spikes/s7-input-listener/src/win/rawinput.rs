//! Raw Input listener on its own thread (adapted from the S7/S9 tool).
//!
//! Registers keyboards (1/6), mice (1/2), joysticks (1/4), gamepads (1/5) and multi-axis
//! controllers (1/8) with RIDEV_INPUTSINK on a never-shown window, so WM_INPUT keeps arriving
//! while Elite is in the foreground. No RIDEV_NOLEGACY / RIDEV_CAPTUREMOUSE: registering only
//! adds us as an extra listener and takes nothing away from the game. Nothing is ever sent.

use super::{foreground_title, wide, Msg};
use crate::model::{axis_names, hat_direction, DevKind, DeviceMeta, Sample, SampleData};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::time::Instant;
use windows_sys::Win32::Devices::HumanInterfaceDevice::*;
use windows_sys::Win32::Foundation::{CloseHandle, GetLastError, HANDLE, HWND, INVALID_HANDLE_VALUE, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::Storage::FileSystem::{CreateFileW, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetKeyNameTextW;
use windows_sys::Win32::UI::Input::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

struct AxisCap {
    link: u16,
    usage: u16,
    name: String,
    lmin: i64,
    lmax: i64,
    bits: u16,
}

struct HidDev {
    key: String,
    preparsed: Vec<u64>,
    hats: Vec<(u16, i64, i64)>,
    axes: Vec<AxisCap>,
}

struct State {
    tx: Sender<Msg>,
    start: Instant,
    hid: HashMap<isize, Option<HidDev>>,
    other: HashMap<isize, String>,
}

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

/// Start the listener thread. Returns once registration has been attempted.
pub fn start(tx: Sender<Msg>, start: Instant) -> Result<(), String> {
    let (rtx, rrx) = std::sync::mpsc::channel();
    std::thread::Builder::new()
        .name("rawinput".into())
        .spawn(move || unsafe { thread_main(tx, start, rtx) })
        .map_err(|e| e.to_string())?;
    rrx.recv().unwrap_or(Err("raw input thread died".into()))
}

unsafe fn thread_main(tx: Sender<Msg>, start: Instant, ready: Sender<Result<(), String>>) {
    let hinst = GetModuleHandleW(std::ptr::null());
    let class = wide("S7InputListenerSink");
    let wc = WNDCLASSEXW { cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32, lpfnWndProc: Some(wndproc), hInstance: hinst, lpszClassName: class.as_ptr(), ..std::mem::zeroed() };
    RegisterClassExW(&wc);
    let title = wide("S7 input listener (hidden)");
    let hwnd = CreateWindowExW(0, class.as_ptr(), title.as_ptr(), WS_OVERLAPPED, 0, 0, 0, 0, std::ptr::null_mut(), std::ptr::null_mut(), hinst, std::ptr::null());
    if hwnd.is_null() {
        let _ = ready.send(Err(format!("CreateWindowExW failed, error {}", GetLastError())));
        return;
    }
    STATE.with(|s| *s.borrow_mut() = Some(State { tx, start, hid: HashMap::new(), other: HashMap::new() }));
    let usages: [(u16, u16); 5] = [(1, 6), (1, 2), (1, 4), (1, 5), (1, 8)];
    let rids: Vec<RAWINPUTDEVICE> = usages.iter().map(|&(p, u)| RAWINPUTDEVICE { usUsagePage: p, usUsage: u, dwFlags: RIDEV_INPUTSINK | RIDEV_DEVNOTIFY, hwndTarget: hwnd }).collect();
    let ok = RegisterRawInputDevices(rids.as_ptr(), rids.len() as u32, std::mem::size_of::<RAWINPUTDEVICE>() as u32);
    let _ = ready.send(if ok != 0 { Ok(()) } else { Err(format!("RegisterRawInputDevices failed, error {}", GetLastError())) });
    let mut msg: MSG = std::mem::zeroed();
    while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_INPUT => {
            handle_input(lparam as HRAWINPUT);
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
        WM_INPUT_DEVICE_CHANGE => {
            if wparam as u32 == GIDC_REMOVAL {
                STATE.with(|s| {
                    if let Some(st) = s.borrow_mut().as_mut() {
                        st.hid.remove(&lparam);
                        st.other.remove(&lparam);
                    }
                });
            }
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe fn handle_input(h: HRAWINPUT) {
    let hdr = std::mem::size_of::<RAWINPUTHEADER>() as u32;
    let mut size = 0u32;
    if GetRawInputData(h, RID_INPUT, std::ptr::null_mut(), &mut size, hdr) != 0 || size == 0 {
        return;
    }
    let mut buf = vec![0u64; (size as usize).div_ceil(8)];
    if GetRawInputData(h, RID_INPUT, buf.as_mut_ptr() as *mut _, &mut size, hdr) == u32::MAX {
        return;
    }
    let raw = &*(buf.as_ptr() as *const RAWINPUT);
    let dev = raw.header.hDevice;
    STATE.with(|s| {
        let mut sref = s.borrow_mut();
        let Some(st) = sref.as_mut() else { return };
        let t_ms = st.start.elapsed().as_secs_f64() * 1000.0;
        let injected = dev.is_null();
        let mut samples: Vec<(String, SampleData)> = vec![];
        match raw.header.dwType {
            RIM_TYPEKEYBOARD => {
                let k = raw.data.keyboard;
                if k.VKey == 0xFF {
                    return;
                }
                let key = other_key(st, dev, DevKind::Keyboard);
                let e0 = k.Flags & 2 != 0;
                samples.push((key, SampleData::Key { vkey: k.VKey, scan: k.MakeCode, e0, down: k.Flags & 1 == 0, name: key_label(k.MakeCode, e0) }));
            }
            RIM_TYPEMOUSE => {
                let flags = raw.data.mouse.Anonymous.Anonymous.usButtonFlags as u32;
                if flags & 0x3FF == 0 {
                    return; // movement/wheel only: not logged
                }
                let key = other_key(st, dev, DevKind::Mouse);
                for (n, dn, up) in [(1u8, 0x1u32, 0x2u32), (2, 0x4, 0x8), (3, 0x10, 0x20), (4, 0x40, 0x80), (5, 0x100, 0x200)] {
                    if flags & dn != 0 {
                        samples.push((key.clone(), SampleData::MouseButton { n, down: true }));
                    }
                    if flags & up != 0 {
                        samples.push((key.clone(), SampleData::MouseButton { n, down: false }));
                    }
                }
            }
            RIM_TYPEHID => {
                let hid = raw.data.hid;
                if !st.hid.contains_key(&(dev as isize)) {
                    let (meta, hd) = load_hid(dev);
                    let _ = st.tx.send(Msg::Device(meta));
                    st.hid.insert(dev as isize, hd);
                }
                let Some(Some(hd)) = st.hid.get(&(dev as isize)) else { return };
                let data = (&raw.data.hid.bRawData) as *const u8;
                for i in 0..hid.dwCount as usize {
                    let report = data.add(i * hid.dwSizeHid as usize) as *mut u8;
                    samples.push((hd.key.clone(), parse_report(hd, report, hid.dwSizeHid)));
                }
            }
            _ => {}
        }
        if samples.is_empty() {
            return;
        }
        let fg = foreground_title();
        for (dev, data) in samples {
            let _ = st.tx.send(Msg::Sample(Sample { t_ms, dev, fg: fg.clone(), injected, data }));
        }
    });
}

fn key_label(scan: u16, e0: bool) -> String {
    let lparam = ((scan as i32) << 16) | if e0 { 1 << 24 } else { 0 };
    let mut buf = [0u16; 64];
    let n = unsafe { GetKeyNameTextW(lparam, buf.as_mut_ptr(), buf.len() as i32) };
    if n > 0 { format!("key {}", String::from_utf16_lossy(&buf[..n as usize])) } else { format!("key scan 0x{scan:02X}") }
}

unsafe fn other_key(st: &mut State, dev: HANDLE, kind: DevKind) -> String {
    if let Some(k) = st.other.get(&(dev as isize)) {
        return k.clone();
    }
    let meta = if dev.is_null() {
        DeviceMeta { key: format!("{kind:?}:injected"), kind, vid: 0, pid: 0, product: format!("software-generated {}", if kind == DevKind::Keyboard { "keyboard" } else { "mouse" }), usage: 0, buttons_declared: 0, axes_declared: vec![], hats_declared: 0 }
    } else {
        basic_meta(dev, kind)
    };
    let k = meta.key.clone();
    let _ = st.tx.send(Msg::Device(meta));
    st.other.insert(dev as isize, k.clone());
    k
}

pub unsafe fn device_name(dev: HANDLE) -> String {
    let mut n = 0u32;
    GetRawInputDeviceInfoW(dev, RIDI_DEVICENAME, std::ptr::null_mut(), &mut n);
    if n == 0 {
        return String::new();
    }
    let mut buf = vec![0u16; n as usize + 1];
    if GetRawInputDeviceInfoW(dev, RIDI_DEVICENAME, buf.as_mut_ptr() as *mut _, &mut n) == u32::MAX {
        return String::new();
    }
    let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..end])
}

unsafe fn product_string(path: &str) -> String {
    if path.is_empty() {
        return String::new();
    }
    let w = wide(path);
    // Zero access rights: enough for HidD_Get*String; never opens the device for I/O.
    let h = CreateFileW(w.as_ptr(), 0, FILE_SHARE_READ | FILE_SHARE_WRITE, std::ptr::null(), OPEN_EXISTING, 0, std::ptr::null_mut());
    if h.is_null() || h == INVALID_HANDLE_VALUE {
        return String::new();
    }
    let mut buf = [0u16; 128];
    let ok = HidD_GetProductString(h, buf.as_mut_ptr() as *mut _, (buf.len() * 2) as u32);
    CloseHandle(h);
    if !ok {
        return String::new();
    }
    let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..end]).trim().to_string()
}

fn parse_vid_pid(path: &str) -> (u16, u16) {
    let up = path.to_ascii_uppercase();
    let grab = |tag: &str| up.find(tag).and_then(|i| up.get(i + tag.len()..i + tag.len() + 4)).and_then(|h| u16::from_str_radix(h, 16).ok()).unwrap_or(0);
    (grab("VID_"), grab("PID_"))
}

unsafe fn basic_meta(dev: HANDLE, kind: DevKind) -> DeviceMeta {
    let path = device_name(dev);
    let (mut vid, mut pid) = parse_vid_pid(&path);
    let mut di: RID_DEVICE_INFO = std::mem::zeroed();
    di.cbSize = std::mem::size_of::<RID_DEVICE_INFO>() as u32;
    let mut sz = di.cbSize;
    let mut usage = match kind {
        DevKind::Keyboard => 6,
        DevKind::Mouse => 2,
        _ => 0,
    };
    if GetRawInputDeviceInfoW(dev, RIDI_DEVICEINFO, &mut di as *mut _ as *mut _, &mut sz) != u32::MAX && di.dwType == RIM_TYPEHID {
        let h = di.Anonymous.hid;
        vid = h.dwVendorId as u16;
        pid = h.dwProductId as u16;
        usage = h.usUsage;
    }
    let mut product = product_string(&path);
    if product.is_empty() {
        product = match kind {
            DevKind::Keyboard => "Keyboard".into(),
            DevKind::Mouse => "Mouse".into(),
            _ => String::new(),
        };
    }
    DeviceMeta { key: if path.is_empty() { format!("{kind:?}:{}", dev as isize) } else { path }, kind, vid, pid, product, usage, buttons_declared: 0, axes_declared: vec![], hats_declared: 0 }
}

unsafe fn load_hid(dev: HANDLE) -> (DeviceMeta, Option<HidDev>) {
    let mut meta = basic_meta(dev, DevKind::Hid);
    let mut n = 0u32;
    GetRawInputDeviceInfoW(dev, RIDI_PREPARSEDDATA, std::ptr::null_mut(), &mut n);
    let mut pre = vec![0u64; (n as usize).div_ceil(8).max(1)];
    if n == 0 || GetRawInputDeviceInfoW(dev, RIDI_PREPARSEDDATA, pre.as_mut_ptr() as *mut _, &mut n) == u32::MAX {
        return (meta, None);
    }
    let pp = pre.as_ptr() as PHIDP_PREPARSED_DATA;
    let mut caps: HIDP_CAPS = std::mem::zeroed();
    let mut hats = vec![];
    let mut axes = vec![];
    if HidP_GetCaps(pp, &mut caps) == HIDP_STATUS_SUCCESS {
        let mut nb = caps.NumberInputButtonCaps;
        if nb > 0 {
            let mut bc: Vec<HIDP_BUTTON_CAPS> = vec![std::mem::zeroed(); nb as usize];
            if HidP_GetButtonCaps(HidP_Input, bc.as_mut_ptr(), &mut nb, pp) == HIDP_STATUS_SUCCESS {
                for c in &bc[..nb as usize] {
                    if c.UsagePage == 9 {
                        meta.buttons_declared += if c.IsRange { (c.Anonymous.Range.UsageMax as u32).saturating_sub(c.Anonymous.Range.UsageMin as u32) + 1 } else { 1 };
                    }
                }
            }
        }
        let mut nv = caps.NumberInputValueCaps;
        if nv > 0 {
            let mut vc: Vec<HIDP_VALUE_CAPS> = vec![std::mem::zeroed(); nv as usize];
            if HidP_GetValueCaps(HidP_Input, vc.as_mut_ptr(), &mut nv, pp) == HIDP_STATUS_SUCCESS {
                let mut found: Vec<(u16, u16, i64, i64, u16)> = vec![];
                for c in &vc[..nv as usize] {
                    if c.UsagePage != 1 {
                        continue;
                    }
                    let (lo, hi) = if c.IsRange { (c.Anonymous.Range.UsageMin, c.Anonymous.Range.UsageMax) } else { (c.Anonymous.NotRange.Usage, c.Anonymous.NotRange.Usage) };
                    for u in lo..=hi {
                        if u == 0x39 {
                            hats.push((c.LinkCollection, c.LogicalMin as i64, c.LogicalMax as i64));
                        } else if (0x30..=0x38).contains(&u) {
                            found.push((u, c.LinkCollection, c.LogicalMin as i64, c.LogicalMax as i64, c.BitSize));
                        }
                    }
                }
                found.sort_by_key(|f| f.0);
                let names = axis_names(&found.iter().map(|f| f.0).collect::<Vec<_>>());
                for (f, (_, name)) in found.iter().zip(names) {
                    axes.push(AxisCap { link: f.1, usage: f.0, name, lmin: f.2, lmax: f.3, bits: f.4 });
                }
            }
        }
    }
    meta.hats_declared = hats.len() as u32;
    meta.axes_declared = axes.iter().map(|a| a.name.clone()).collect();
    let key = meta.key.clone();
    (meta, Some(HidDev { key, preparsed: pre, hats, axes }))
}

unsafe fn parse_report(hd: &HidDev, report: *mut u8, len: u32) -> SampleData {
    let pp = hd.preparsed.as_ptr() as PHIDP_PREPARSED_DATA;
    let mut buttons = vec![];
    let max = HidP_MaxUsageListLength(HidP_Input, 9, pp);
    if max > 0 {
        let mut list = vec![0u16; max as usize];
        let mut n = max;
        if HidP_GetUsages(HidP_Input, 9, 0, list.as_mut_ptr(), &mut n, pp, report, len) == HIDP_STATUS_SUCCESS {
            buttons.extend_from_slice(&list[..n as usize]);
        }
    }
    let mut hats = vec![];
    for &(link, lmin, lmax) in &hd.hats {
        let mut v = 0u32;
        let d = if HidP_GetUsageValue(HidP_Input, 1, link, 0x39, &mut v, pp, report as *const u8, len) == HIDP_STATUS_SUCCESS { hat_direction(v as i64, lmin, lmax) } else { None };
        hats.push(d);
    }
    let mut axes = vec![];
    for a in &hd.axes {
        let mut v = 0u32;
        if HidP_GetUsageValue(HidP_Input, 1, a.link, a.usage, &mut v, pp, report as *const u8, len) == HIDP_STATUS_SUCCESS {
            let mut x = v as i64;
            if a.lmin < 0 && a.bits > 0 && a.bits < 32 && x & (1 << (a.bits - 1)) != 0 {
                x -= 1 << a.bits; // sign-extend
            }
            let (lo, hi) = if a.lmax > a.lmin { (a.lmin, a.lmax) } else { (0, (1i64 << a.bits.min(31)) - 1) };
            let f = ((x - lo) as f64 / (hi - lo).max(1) as f64).clamp(0.0, 1.0);
            axes.push((a.name.clone(), f));
        }
    }
    SampleData::Hid { buttons, hats, axes }
}

/// Attached devices at start (so idle devices appear in the report).
pub fn list_devices() -> Vec<DeviceMeta> {
    unsafe {
        let mut n = 0u32;
        let sz = std::mem::size_of::<RAWINPUTDEVICELIST>() as u32;
        GetRawInputDeviceList(std::ptr::null_mut(), &mut n, sz);
        let mut v: Vec<RAWINPUTDEVICELIST> = vec![std::mem::zeroed(); n as usize];
        let got = GetRawInputDeviceList(v.as_mut_ptr(), &mut n, sz);
        if got == u32::MAX {
            return vec![];
        }
        v.truncate(got as usize);
        v.iter()
            .filter_map(|d| match d.dwType {
                RIM_TYPEHID => {
                    let (m, _) = load_hid(d.hDevice);
                    if matches!(m.usage, 4 | 5 | 8) { Some(m) } else { None }
                }
                RIM_TYPEKEYBOARD => Some(basic_meta(d.hDevice, DevKind::Keyboard)),
                RIM_TYPEMOUSE => Some(basic_meta(d.hDevice, DevKind::Mouse)),
                _ => None,
            })
            .collect()
    }
}
