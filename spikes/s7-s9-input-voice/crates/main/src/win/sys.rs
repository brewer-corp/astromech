//! Windows system info from the registry and GlobalMemoryStatusEx (read-only).

use crate::sysinfo::SystemInfo;
use windows_sys::Win32::System::Registry::{RegGetValueW, HKEY_LOCAL_MACHINE, RRF_RT_REG_SZ};
use windows_sys::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

fn reg_sz(subkey: &str, value: &str) -> Option<String> {
    let k = super::wide(subkey);
    let v = super::wide(value);
    let mut buf = [0u16; 512];
    let mut len = (buf.len() * 2) as u32;
    let r = unsafe { RegGetValueW(HKEY_LOCAL_MACHINE, k.as_ptr(), v.as_ptr(), RRF_RT_REG_SZ, std::ptr::null_mut(), buf.as_mut_ptr() as *mut _, &mut len) };
    if r != 0 {
        return None;
    }
    let n = (len as usize / 2).min(buf.len());
    let end = buf[..n].iter().position(|&c| c == 0).unwrap_or(n);
    Some(String::from_utf16_lossy(&buf[..end]).trim().to_string())
}

pub fn collect() -> SystemInfo {
    let nt = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion";
    let build: u32 = reg_sz(nt, "CurrentBuild").and_then(|b| b.parse().ok()).unwrap_or(0);
    let mut product = reg_sz(nt, "ProductName").unwrap_or_else(|| "Windows".into());
    if build >= 22000 {
        product = product.replace("Windows 10", "Windows 11");
    }
    let os = format!("{product} {} (build {build})", reg_sz(nt, "DisplayVersion").unwrap_or_default());
    let cpu = reg_sz(r"HARDWARE\DESCRIPTION\System\CentralProcessor\0", "ProcessorNameString").unwrap_or_default();
    let mut gpus = vec![];
    for i in 0..16 {
        let key = format!(r"SYSTEM\CurrentControlSet\Control\Class\{{4d36e968-e325-11ce-bfc1-08002be10318}}\{i:04}");
        if let Some(d) = reg_sz(&key, "DriverDesc") {
            let ver = reg_sz(&key, "DriverVersion").unwrap_or_default();
            let g = format!("{d} (driver {ver})");
            if !gpus.contains(&g) {
                gpus.push(g);
            }
        }
    }
    let mut m: MEMORYSTATUSEX = unsafe { std::mem::zeroed() };
    m.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
    let ram_gb = if unsafe { GlobalMemoryStatusEx(&mut m) } != 0 { m.ullTotalPhys as f64 / 1024.0 / 1024.0 / 1024.0 } else { 0.0 };
    SystemInfo { os, cpu, ram_gb, gpus, ..Default::default() }
}
