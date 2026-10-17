//! Write text to the Windows clipboard (CF_UNICODETEXT). The route follower puts the next
//! waypoint here so it can be pasted into the galaxy map's search box (docs/PLOTTING.md).

/// Replace the clipboard with `text`.
#[cfg(windows)]
pub fn set_text(text: &str) -> Result<(), String> {
    use std::ptr::null_mut;
    use windows_sys::Win32::Foundation::GlobalFree;
    use windows_sys::Win32::System::DataExchange::{CloseClipboard, EmptyClipboard, OpenClipboard, SetClipboardData};
    use windows_sys::Win32::System::Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE};
    // Win32_System_Ole's CLIPBOARD_FORMAT; not worth the feature for one constant.
    const CF_UNICODETEXT: u32 = 13;

    let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        // Another app may hold the clipboard open for a moment.
        let mut opened = false;
        for _ in 0..10 {
            if OpenClipboard(null_mut()) != 0 {
                opened = true;
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        if !opened {
            return Err("the clipboard is busy (another app has it open)".into());
        }
        let r = (|| {
            if EmptyClipboard() == 0 {
                return Err("couldn't empty the clipboard".to_string());
            }
            let h = GlobalAlloc(GMEM_MOVEABLE, wide.len() * 2);
            if h.is_null() {
                return Err("out of memory for the clipboard".to_string());
            }
            let p = GlobalLock(h) as *mut u16;
            if p.is_null() {
                GlobalFree(h);
                return Err("couldn't lock clipboard memory".to_string());
            }
            std::ptr::copy_nonoverlapping(wide.as_ptr(), p, wide.len());
            GlobalUnlock(h);
            // On success the clipboard owns the memory.
            if SetClipboardData(CF_UNICODETEXT, h).is_null() {
                GlobalFree(h);
                return Err("couldn't set the clipboard text".to_string());
            }
            Ok(())
        })();
        CloseClipboard();
        r
    }
}

#[cfg(not(windows))]
pub fn set_text(_text: &str) -> Result<(), String> {
    Err("the clipboard needs Windows".into())
}
