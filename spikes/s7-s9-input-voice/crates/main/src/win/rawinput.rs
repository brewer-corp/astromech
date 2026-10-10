//! Raw Input listener on its own thread.
//!
//! Registers keyboards (1/6), mice (1/2), joysticks (1/4), gamepads (1/5) and multi-axis
//! controllers (1/8) with RIDEV_INPUTSINK so WM_INPUT keeps arriving while another window
//! (Elite) is in the foreground. RIDEV_INPUTSINK requires a target window; by default we use
//! a never-shown top-level window (`--message-only` tries a message-only window instead).
//! We do not use RIDEV_NOLEGACY or RIDEV_CAPTUREMOUSE, so nothing is taken away from the game:
//! Raw Input registration only adds us as an extra listener.
//!
//! HID button state is parsed from each input report with the device's preparsed data
//! (HidP_GetUsages for buttons, HidP_GetUsageValue for hat switches) and turned into
//! press/release transitions. The first report of a device only sets the baseline.

use crate::input::{diff_sets, hat_direction, parse_vid_pid, Control, DeviceInfo, InputEvent, Source};
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::time::Instant;
use windows_sys::Win32::Devices::HumanInterfaceDevice::*;
use windows_sys::Win32::Foundation::{CloseHandle, GetLastError, HANDLE, HWND, INVALID_HANDLE_VALUE, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::Storage::FileSystem::{CreateFileW, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::SystemInformation::GetTickCount;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetKeyNameTextW;
use windows_sys::Win32::UI::Input::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

pub struct Listener {
    pub message_only: bool,
    pub registered: Result<(), String>,
}

struct HidDev {
    info: Arc<DeviceInfo>,
    preparsed: Vec<u64>,
    pages: Vec<u16>,
    hats: Vec<(u16, i64, i64)>, // (link collection, logical min, logical max)
    prev: Option<BTreeSet<Control>>,
}

enum Dev {
    Hid(Box<HidDev>),
    Other(Arc<DeviceInfo>),
}

struct State {
    tx: Sender<InputEvent>,
    devices: HashMap<isize, Dev>,
    mouse_down: [bool; 6],
}

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

/// Start the listener thread. Events go to `tx`. Returns once registration has been attempted.
pub fn start(tx: Sender<InputEvent>, message_only: bool) -> Listener {
    let (rtx, rrx) = std::sync::mpsc::channel();
    std::thread::Builder::new()
        .name("rawinput".into())
        .spawn(move || unsafe { thread_main(tx, message_only, rtx) })
        .expect("spawn raw input thread");
    rrx.recv().unwrap_or(Listener { message_only, registered: Err("raw input thread died".into()) })
}

unsafe fn thread_main(tx: Sender<InputEvent>, message_only: bool, ready: Sender<Listener>) {
    let hinst = GetModuleHandleW(std::ptr::null());
    let class = super::wide("S7RawInputSink");
    let wc = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        lpfnWndProc: Some(wndproc),
        hInstance: hinst,
        lpszClassName: class.as_ptr(),
        ..std::mem::zeroed()
    };
    RegisterClassExW(&wc);
    let parent: HWND = if message_only { HWND_MESSAGE } else { std::ptr::null_mut() };
    let title = super::wide("S7 input listener");
    let hwnd = CreateWindowExW(0, class.as_ptr(), title.as_ptr(), WS_OVERLAPPED, 0, 0, 0, 0, parent, std::ptr::null_mut(), hinst, std::ptr::null());
    if hwnd.is_null() {
        let _ = ready.send(Listener { message_only, registered: Err(format!("CreateWindowExW failed, error {}", GetLastError())) });
        return;
    }
    STATE.with(|s| *s.borrow_mut() = Some(State { tx, devices: HashMap::new(), mouse_down: [false; 6] }));
    let flags = RIDEV_INPUTSINK | RIDEV_DEVNOTIFY;
    let usages: [(u16, u16); 5] = [(1, 6), (1, 2), (1, 4), (1, 5), (1, 8)];
    let rids: Vec<RAWINPUTDEVICE> = usages.iter().map(|&(p, u)| RAWINPUTDEVICE { usUsagePage: p, usUsage: u, dwFlags: flags, hwndTarget: hwnd }).collect();
    let ok = RegisterRawInputDevices(rids.as_ptr(), rids.len() as u32, std::mem::size_of::<RAWINPUTDEVICE>() as u32);
    let registered = if ok != 0 { Ok(()) } else { Err(format!("RegisterRawInputDevices failed, error {}", GetLastError())) };
    let _ = ready.send(Listener { message_only, registered });
    let mut msg: MSG = std::mem::zeroed();
    while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_INPUT => {
            let at = Instant::now();
            let queue_ms = GetTickCount().wrapping_sub(GetMessageTime() as u32);
            handle_input(lparam as HRAWINPUT, at, queue_ms);
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
        WM_INPUT_DEVICE_CHANGE => {
            if wparam as u32 == GIDC_REMOVAL {
                STATE.with(|s| {
                    if let Some(st) = s.borrow_mut().as_mut() {
                        st.devices.remove(&lparam);
                    }
                });
            }
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

pub fn foreground_title() -> String {
    unsafe {
        let h = GetForegroundWindow();
        if h.is_null() {
            return String::new();
        }
        let mut buf = [0u16; 512];
        let n = GetWindowTextW(h, buf.as_mut_ptr(), buf.len() as i32);
        String::from_utf16_lossy(&buf[..n.max(0) as usize])
    }
}

unsafe fn handle_input(h: HRAWINPUT, at: Instant, queue_ms: u32) {
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
    let mut events: Vec<(Arc<DeviceInfo>, Control, bool, String, bool)> = vec![];
    STATE.with(|s| {
        let mut sref = s.borrow_mut();
        let Some(st) = sref.as_mut() else { return };
        match raw.header.dwType {
            RIM_TYPEKEYBOARD => {
                let k = raw.data.keyboard;
                if k.VKey == 0xFF {
                    return; // fake key from an E1/E0 prefix sequence
                }
                let info = other_info(st, dev, Source::RawKeyboard);
                let e0 = k.Flags & 2 != 0;
                let down = k.Flags & 1 == 0;
                let c = Control::Key { vkey: k.VKey, scan: k.MakeCode, e0 };
                events.push((info, c, down, key_label(k.MakeCode, e0, k.VKey), dev.is_null()));
            }
            RIM_TYPEMOUSE => {
                let flags = raw.data.mouse.Anonymous.Anonymous.usButtonFlags as u32;
                let info = other_info(st, dev, Source::RawMouse);
                for (n, dn, up) in [(1u8, 0x1u32, 0x2u32), (2, 0x4, 0x8), (3, 0x10, 0x20), (4, 0x40, 0x80), (5, 0x100, 0x200)] {
                    for (bit, down) in [(dn, true), (up, false)] {
                        if flags & bit != 0 && st.mouse_down[n as usize] != down {
                            st.mouse_down[n as usize] = down;
                            let c = Control::MouseButton { n };
                            events.push((info.clone(), c, down, c.describe(), dev.is_null()));
                        }
                    }
                }
            }
            RIM_TYPEHID => {
                let hid = raw.data.hid;
                let entry = st.devices.entry(dev as isize).or_insert_with(|| load_device(dev, Source::RawHid));
                let Dev::Hid(hd) = entry else { return };
                let data = (&raw.data.hid.bRawData) as *const u8;
                for i in 0..hid.dwCount as usize {
                    let report = data.add(i * hid.dwSizeHid as usize) as *mut u8;
                    let cur = parse_report(hd, report, hid.dwSizeHid);
                    if let Some(prev) = &hd.prev {
                        for (c, down) in diff_sets(prev, &cur) {
                            events.push((hd.info.clone(), c, down, c.describe(), false));
                        }
                    }
                    hd.prev = Some(cur);
                }
            }
            _ => {}
        }
        if !events.is_empty() {
            let fg = foreground_title();
            for (device, control, down, label, injected) in events.drain(..) {
                let _ = st.tx.send(InputEvent { device, control, label, down, at, queue_ms: Some(queue_ms), foreground: fg.clone(), injected });
            }
        }
    });
}

fn key_label(scan: u16, e0: bool, vkey: u16) -> String {
    let lparam = ((scan as i32) << 16) | if e0 { 1 << 24 } else { 0 };
    let mut buf = [0u16; 64];
    let n = unsafe { GetKeyNameTextW(lparam, buf.as_mut_ptr(), buf.len() as i32) };
    if n > 0 {
        format!("key {}", String::from_utf16_lossy(&buf[..n as usize]))
    } else {
        Control::Key { vkey, scan, e0 }.describe()
    }
}

fn other_info(st: &mut State, dev: HANDLE, source: Source) -> Arc<DeviceInfo> {
    if dev.is_null() {
        return Arc::new(DeviceInfo { source, path: "(injected)".into(), vid: 0, pid: 0, product: "software-generated input".into(), usage_page: 1, usage: 0 });
    }
    match st.devices.entry(dev as isize).or_insert_with(|| unsafe { load_device(dev, source) }) {
        Dev::Other(i) => i.clone(),
        Dev::Hid(h) => h.info.clone(),
    }
}

pub unsafe fn device_name(dev: HANDLE) -> String {
    let mut n = 0u32;
    GetRawInputDeviceInfoW(dev, RIDI_DEVICENAME, std::ptr::null_mut(), &mut n);
    if n == 0 {
        return String::new();
    }
    let mut buf = vec![0u16; n as usize + 1];
    let r = GetRawInputDeviceInfoW(dev, RIDI_DEVICENAME, buf.as_mut_ptr() as *mut _, &mut n);
    if r == u32::MAX {
        return String::new();
    }
    let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..end])
}

pub unsafe fn product_string(path: &str) -> String {
    if path.is_empty() {
        return String::new();
    }
    let w = super::wide(path);
    // Zero access rights: enough for HidD_Get*String, and never opens the device for I/O.
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

pub unsafe fn device_info(dev: HANDLE, source: Source) -> DeviceInfo {
    let path = device_name(dev);
    let (mut vid, mut pid) = parse_vid_pid(&path);
    let mut di: RID_DEVICE_INFO = std::mem::zeroed();
    di.cbSize = std::mem::size_of::<RID_DEVICE_INFO>() as u32;
    let mut sz = di.cbSize;
    let (mut page, mut usage) = (1u16, 0u16);
    if GetRawInputDeviceInfoW(dev, RIDI_DEVICEINFO, &mut di as *mut _ as *mut _, &mut sz) != u32::MAX && di.dwType == RIM_TYPEHID {
        let h = di.Anonymous.hid;
        vid = h.dwVendorId as u16;
        pid = h.dwProductId as u16;
        page = h.usUsagePage;
        usage = h.usUsage;
    } else if source == Source::RawKeyboard {
        usage = 6;
    } else if source == Source::RawMouse {
        usage = 2;
    }
    let mut product = product_string(&path);
    if product.is_empty() {
        product = match source {
            Source::RawKeyboard => "Keyboard".into(),
            Source::RawMouse => "Mouse".into(),
            _ => String::new(),
        };
    }
    DeviceInfo { source, path, vid, pid, product, usage_page: page, usage }
}

unsafe fn load_device(dev: HANDLE, source: Source) -> Dev {
    let info = Arc::new(device_info(dev, source));
    if source != Source::RawHid {
        return Dev::Other(info);
    }
    let mut n = 0u32;
    GetRawInputDeviceInfoW(dev, RIDI_PREPARSEDDATA, std::ptr::null_mut(), &mut n);
    let mut pre = vec![0u64; (n as usize).div_ceil(8).max(1)];
    if n == 0 || GetRawInputDeviceInfoW(dev, RIDI_PREPARSEDDATA, pre.as_mut_ptr() as *mut _, &mut n) == u32::MAX {
        return Dev::Hid(Box::new(HidDev { info, preparsed: vec![], pages: vec![], hats: vec![], prev: None }));
    }
    let pp = pre.as_ptr() as PHIDP_PREPARSED_DATA;
    let mut caps: HIDP_CAPS = std::mem::zeroed();
    let mut pages = vec![];
    let mut hats = vec![];
    if HidP_GetCaps(pp, &mut caps) == HIDP_STATUS_SUCCESS {
        let mut nb = caps.NumberInputButtonCaps;
        if nb > 0 {
            let mut bc: Vec<HIDP_BUTTON_CAPS> = vec![std::mem::zeroed(); nb as usize];
            if HidP_GetButtonCaps(HidP_Input, bc.as_mut_ptr(), &mut nb, pp) == HIDP_STATUS_SUCCESS {
                for c in &bc[..nb as usize] {
                    if !pages.contains(&c.UsagePage) {
                        pages.push(c.UsagePage);
                    }
                }
            }
        }
        let mut nv = caps.NumberInputValueCaps;
        if nv > 0 {
            let mut vc: Vec<HIDP_VALUE_CAPS> = vec![std::mem::zeroed(); nv as usize];
            if HidP_GetValueCaps(HidP_Input, vc.as_mut_ptr(), &mut nv, pp) == HIDP_STATUS_SUCCESS {
                for c in &vc[..nv as usize] {
                    let is_hat = c.UsagePage == 1
                        && if c.IsRange { c.Anonymous.Range.UsageMin <= 0x39 && 0x39 <= c.Anonymous.Range.UsageMax } else { c.Anonymous.NotRange.Usage == 0x39 };
                    if is_hat {
                        hats.push((c.LinkCollection, c.LogicalMin as i64, c.LogicalMax as i64));
                    }
                }
            }
        }
    }
    Dev::Hid(Box::new(HidDev { info, preparsed: pre, pages, hats, prev: None }))
}

unsafe fn parse_report(hd: &HidDev, report: *mut u8, len: u32) -> BTreeSet<Control> {
    let mut set = BTreeSet::new();
    if hd.preparsed.is_empty() {
        return set;
    }
    let pp = hd.preparsed.as_ptr() as PHIDP_PREPARSED_DATA;
    for &page in &hd.pages {
        let max = HidP_MaxUsageListLength(HidP_Input, page, pp);
        if max == 0 {
            continue;
        }
        let mut list = vec![0u16; max as usize];
        let mut n = max;
        if HidP_GetUsages(HidP_Input, page, 0, list.as_mut_ptr(), &mut n, pp, report, len) == HIDP_STATUS_SUCCESS {
            for &u in &list[..n as usize] {
                set.insert(Control::HidButton { page, usage: u });
            }
        }
    }
    for (i, &(link, lmin, lmax)) in hd.hats.iter().enumerate() {
        let mut v = 0u32;
        if HidP_GetUsageValue(HidP_Input, 1, link, 0x39, &mut v, pp, report as *const u8, len) == HIDP_STATUS_SUCCESS {
            if let Some(dir) = hat_direction(v as i64, lmin, lmax) {
                set.insert(Control::Hat { index: i as u8, dir });
            }
        }
    }
    set
}

/// Read-only list of attached Raw Input devices (for --dry-run and the S7 intro).
pub fn list_devices() -> Vec<DeviceInfo> {
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
            .map(|d| {
                let src = match d.dwType {
                    RIM_TYPEKEYBOARD => Source::RawKeyboard,
                    RIM_TYPEMOUSE => Source::RawMouse,
                    _ => Source::RawHid,
                };
                device_info(d.hDevice, src)
            })
            .collect()
    }
}
