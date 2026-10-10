//! Connected-device inventory: live HID/XInput enumeration on Windows (read-only queries),
//! or a JSON file for testing (`--fake-devices`).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Controller {
    pub vid: String,
    pub pid: String,
    /// Elite's raw device id, VVVVPPPP uppercase hex.
    pub elite_id: String,
    pub product: String,
    pub manufacturer: String,
    /// HID usage (page 1): 4 joystick, 5 gamepad, 8 multi-axis controller.
    pub usage: u16,
    pub kind: String,
    pub buttons: u32,
    /// Axis slots in Elite naming: X, Y, Z, RX, RY, RZ, U, V (sliders/dials map to U then V; see README).
    pub axes: Vec<String>,
    pub hats: u32,
    /// Extra axis-like usages Elite has no slot for (e.g. Wheel 0x38).
    pub other_values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct SimpleDevice {
    pub vid: String,
    pub pid: String,
    pub product: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Inventory {
    /// "live" or "fake: <file>".
    pub source: String,
    pub controllers: Vec<Controller>,
    pub keyboards: Vec<SimpleDevice>,
    pub mice: Vec<SimpleDevice>,
    /// Connected XInput user slots (0-3).
    pub xinput_slots: Vec<u32>,
    pub notes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FakeFile {
    #[serde(default)]
    controllers: Vec<FakeController>,
    #[serde(default)]
    keyboards: Vec<SimpleDevice>,
    #[serde(default)]
    mice: Vec<SimpleDevice>,
    #[serde(default)]
    xinput_slots: Vec<u32>,
}

#[derive(Debug, Deserialize)]
struct FakeController {
    vid: String,
    pid: String,
    #[serde(default)]
    product: String,
    #[serde(default)]
    manufacturer: String,
    #[serde(default = "four")]
    usage: u16,
    #[serde(default)]
    buttons: u32,
    #[serde(default)]
    axes: Vec<String>,
    #[serde(default)]
    hats: u32,
}
fn four() -> u16 {
    4
}

pub fn kind_name(usage: u16) -> &'static str {
    match usage {
        4 => "Joystick",
        5 => "Gamepad",
        8 => "Multi-axis controller",
        _ => "Other",
    }
}

pub fn elite_id(vid: u16, pid: u16) -> String {
    format!("{vid:04X}{pid:04X}")
}

pub fn load_fake(path: &std::path::Path) -> anyhow::Result<Inventory> {
    let text = std::fs::read_to_string(path)?;
    let f: FakeFile = serde_json::from_str(&text)?;
    let mut inv = Inventory { source: format!("fake: {}", path.display()), ..Default::default() };
    for c in f.controllers {
        let vid = u16::from_str_radix(c.vid.trim_start_matches("0x"), 16)?;
        let pid = u16::from_str_radix(c.pid.trim_start_matches("0x"), 16)?;
        inv.controllers.push(Controller {
            vid: format!("{vid:04X}"),
            pid: format!("{pid:04X}"),
            elite_id: elite_id(vid, pid),
            product: c.product,
            manufacturer: c.manufacturer,
            usage: c.usage,
            kind: kind_name(c.usage).into(),
            buttons: c.buttons,
            axes: c.axes.into_iter().map(|a| a.to_ascii_uppercase()).collect(),
            hats: c.hats,
            other_values: vec![],
        });
    }
    inv.keyboards = f.keyboards;
    inv.mice = f.mice;
    inv.xinput_slots = f.xinput_slots;
    Ok(inv)
}

/// Map HID generic-desktop value usages to Elite axis names.
#[cfg_attr(not(windows), allow(dead_code))]
pub fn axes_from_usages(usages: &[u16]) -> (Vec<String>, u32, Vec<String>) {
    let mut axes = vec![];
    let mut hats = 0;
    let mut sliders = 0;
    let mut other = vec![];
    let mut sorted = usages.to_vec();
    sorted.sort_unstable();
    for u in sorted {
        match u {
            0x30 => axes.push("X".to_string()),
            0x31 => axes.push("Y".to_string()),
            0x32 => axes.push("Z".to_string()),
            0x33 => axes.push("RX".to_string()),
            0x34 => axes.push("RY".to_string()),
            0x35 => axes.push("RZ".to_string()),
            0x36 | 0x37 => sliders += 1,
            0x38 => other.push("Wheel (0x38)".to_string()),
            0x39 => hats += 1,
            u => other.push(format!("usage 0x{u:02X}")),
        }
    }
    if sliders >= 1 {
        axes.push("U".into());
    }
    if sliders >= 2 {
        axes.push("V".into());
    }
    if sliders > 2 {
        other.push(format!("{} extra slider/dial axes", sliders - 2));
    }
    axes.dedup();
    (axes, hats, other)
}

#[cfg(not(windows))]
pub fn enumerate_live() -> Inventory {
    Inventory {
        source: "live".into(),
        notes: vec!["Live device enumeration is only implemented on Windows; use --fake-devices.".into()],
        ..Default::default()
    }
}

#[cfg(windows)]
pub fn enumerate_live() -> Inventory {
    let mut inv = Inventory { source: "live".into(), ..Default::default() };
    unsafe { win::enumerate_hid(&mut inv) };
    unsafe { win::enumerate_xinput(&mut inv) };
    inv
}

#[cfg(windows)]
mod win {
    use super::*;
    use windows_sys::Win32::Devices::DeviceAndDriverInstallation::*;
    use windows_sys::Win32::Devices::HumanInterfaceDevice::*;
    use windows_sys::Win32::Foundation::*;
    use windows_sys::Win32::Storage::FileSystem::*;
    use windows_sys::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
    use windows_sys::Win32::UI::Input::XboxController::XINPUT_STATE;

    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    unsafe fn hid_string(h: HANDLE, f: unsafe extern "system" fn(HANDLE, *mut core::ffi::c_void, u32) -> bool) -> String {
        let mut buf = [0u16; 256];
        if f(h, buf.as_mut_ptr() as *mut _, (buf.len() * 2) as u32) {
            let n = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
            String::from_utf16_lossy(&buf[..n]).trim().to_string()
        } else {
            String::new()
        }
    }

    pub unsafe fn enumerate_hid(inv: &mut Inventory) {
        let mut guid = core::mem::zeroed();
        HidD_GetHidGuid(&mut guid);
        let set = SetupDiGetClassDevsW(&guid, core::ptr::null(), core::ptr::null_mut(), DIGCF_PRESENT | DIGCF_DEVICEINTERFACE);
        if set == -1 {
            inv.notes.push("SetupDiGetClassDevsW failed; no HID devices listed.".into());
            return;
        }
        let mut i = 0u32;
        let mut opened_fail = 0;
        loop {
            let mut data: SP_DEVICE_INTERFACE_DATA = core::mem::zeroed();
            data.cbSize = core::mem::size_of::<SP_DEVICE_INTERFACE_DATA>() as u32;
            if SetupDiEnumDeviceInterfaces(set, core::ptr::null(), &guid, i, &mut data) == 0 {
                break;
            }
            i += 1;
            let mut needed = 0u32;
            SetupDiGetDeviceInterfaceDetailW(set, &data, core::ptr::null_mut(), 0, &mut needed, core::ptr::null_mut());
            if needed == 0 {
                continue;
            }
            let mut buf = vec![0u64; (needed as usize).div_ceil(8)];
            let detail = buf.as_mut_ptr() as *mut SP_DEVICE_INTERFACE_DETAIL_DATA_W;
            (*detail).cbSize = core::mem::size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_W>() as u32;
            if SetupDiGetDeviceInterfaceDetailW(set, &data, detail, needed, core::ptr::null_mut(), core::ptr::null_mut()) == 0 {
                continue;
            }
            let path_ptr = core::ptr::addr_of!((*detail).DevicePath) as *const u16;
            // Access 0 = query-only open: attributes and capabilities, no reads/writes of reports.
            let h = CreateFileW(path_ptr, 0, FILE_SHARE_READ | FILE_SHARE_WRITE, core::ptr::null(), OPEN_EXISTING, 0, core::ptr::null_mut());
            if h == INVALID_HANDLE_VALUE {
                opened_fail += 1;
                continue;
            }
            probe(h, inv);
            CloseHandle(h);
        }
        SetupDiDestroyDeviceInfoList(set);
        if opened_fail > 0 {
            inv.notes.push(format!("{opened_fail} HID interface(s) could not be opened for querying (normal for some system devices)."));
        }
    }

    unsafe fn probe(h: HANDLE, inv: &mut Inventory) {
        let mut attr: HIDD_ATTRIBUTES = core::mem::zeroed();
        attr.Size = core::mem::size_of::<HIDD_ATTRIBUTES>() as u32;
        if !HidD_GetAttributes(h, &mut attr) {
            return;
        }
        let mut pp: PHIDP_PREPARSED_DATA = 0;
        if !HidD_GetPreparsedData(h, &mut pp) {
            return;
        }
        let mut caps: HIDP_CAPS = core::mem::zeroed();
        if HidP_GetCaps(pp, &mut caps) != HIDP_STATUS_SUCCESS {
            HidD_FreePreparsedData(pp);
            return;
        }
        let product = hid_string(h, HidD_GetProductString);
        let simple = SimpleDevice { vid: format!("{:04X}", attr.VendorID), pid: format!("{:04X}", attr.ProductID), product: product.clone() };
        if caps.UsagePage == 1 && caps.Usage == 6 {
            inv.keyboards.push(simple);
        } else if caps.UsagePage == 1 && caps.Usage == 2 {
            inv.mice.push(simple);
        } else if caps.UsagePage == 1 && matches!(caps.Usage, 4 | 5 | 8) {
            let manufacturer = hid_string(h, HidD_GetManufacturerString);
            let mut buttons = 0u32;
            let mut n = caps.NumberInputButtonCaps;
            if n > 0 {
                let mut b: Vec<HIDP_BUTTON_CAPS> = vec![core::mem::zeroed(); n as usize];
                if HidP_GetButtonCaps(HidP_Input, b.as_mut_ptr(), &mut n, pp) == HIDP_STATUS_SUCCESS {
                    for c in &b[..n as usize] {
                        if c.UsagePage != 9 {
                            continue;
                        }
                        buttons += if c.IsRange {
                            (c.Anonymous.Range.UsageMax as u32).saturating_sub(c.Anonymous.Range.UsageMin as u32) + 1
                        } else {
                            1
                        };
                    }
                }
            }
            let mut usages = vec![];
            let mut n = caps.NumberInputValueCaps;
            if n > 0 {
                let mut v: Vec<HIDP_VALUE_CAPS> = vec![core::mem::zeroed(); n as usize];
                if HidP_GetValueCaps(HidP_Input, v.as_mut_ptr(), &mut n, pp) == HIDP_STATUS_SUCCESS {
                    for c in &v[..n as usize] {
                        if c.UsagePage != 1 {
                            continue;
                        }
                        if c.IsRange {
                            for u in c.Anonymous.Range.UsageMin..=c.Anonymous.Range.UsageMax {
                                usages.push(u);
                            }
                        } else {
                            usages.push(c.Anonymous.NotRange.Usage);
                        }
                    }
                }
            }
            let (axes, hats, other_values) = axes_from_usages(&usages);
            inv.controllers.push(Controller {
                vid: format!("{:04X}", attr.VendorID),
                pid: format!("{:04X}", attr.ProductID),
                elite_id: elite_id(attr.VendorID, attr.ProductID),
                product,
                manufacturer,
                usage: caps.Usage,
                kind: kind_name(caps.Usage).into(),
                buttons,
                axes,
                hats,
                other_values,
            });
        }
        HidD_FreePreparsedData(pp);
    }

    pub unsafe fn enumerate_xinput(inv: &mut Inventory) {
        let mut lib = core::ptr::null_mut();
        for name in ["xinput1_4.dll", "xinput9_1_0.dll"] {
            lib = LoadLibraryW(wide(name).as_ptr());
            if !lib.is_null() {
                break;
            }
        }
        if lib.is_null() {
            inv.notes.push("XInput DLL not available; XInput pads not checked.".into());
            return;
        }
        let Some(f) = GetProcAddress(lib, b"XInputGetState\0".as_ptr()) else {
            inv.notes.push("XInputGetState not found; XInput pads not checked.".into());
            return;
        };
        let get: unsafe extern "system" fn(u32, *mut XINPUT_STATE) -> u32 = core::mem::transmute(f);
        for slot in 0..4u32 {
            let mut st: XINPUT_STATE = core::mem::zeroed();
            if get(slot, &mut st) == 0 {
                inv.xinput_slots.push(slot);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn axis_mapping() {
        let (a, h, o) = axes_from_usages(&[0x31, 0x30, 0x35, 0x36, 0x39, 0x32, 0x33, 0x34, 0x37, 0x38]);
        assert_eq!(a, vec!["X", "Y", "Z", "RX", "RY", "RZ", "U", "V"]);
        assert_eq!(h, 1);
        assert_eq!(o, vec!["Wheel (0x38)"]);
        assert_eq!(elite_id(0x231d, 0x200), "231D0200");
    }
    #[test]
    fn fake_file() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("f.json");
        std::fs::write(&p, r#"{"controllers":[{"vid":"231d","pid":"3201","product":"VKB","buttons":31,"axes":["x","y"],"hats":4}],"xinput_slots":[0]}"#).unwrap();
        let inv = load_fake(&p).unwrap();
        assert_eq!(inv.controllers[0].elite_id, "231D3201");
        assert_eq!(inv.controllers[0].axes, vec!["X", "Y"]);
        assert_eq!(inv.xinput_slots, vec![0]);
    }
}
