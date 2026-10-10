//! CPU / GPU / RAM / OS description for the results file.

use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct SystemInfo {
    pub os: String,
    pub cpu: String,
    pub logical_cpus: usize,
    pub ram_gb: f64,
    pub gpus: Vec<String>,
}

impl SystemInfo {
    /// A real GPU (not the "Microsoft Basic Render Driver" fallback) is listed.
    pub fn has_gpu(&self) -> bool {
        self.gpus.iter().any(|g| !g.to_lowercase().contains("basic render"))
    }
    pub fn lines(&self) -> Vec<String> {
        let mut v = vec![
            format!("OS:   {}", self.os),
            format!("CPU:  {} ({} logical processors)", self.cpu, self.logical_cpus),
            format!("RAM:  {:.1} GB", self.ram_gb),
        ];
        for g in &self.gpus {
            v.push(format!("GPU:  {g}"));
        }
        v
    }
}

pub fn collect() -> SystemInfo {
    let mut s = platform();
    s.logical_cpus = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(0);
    s
}

#[cfg(windows)]
fn platform() -> SystemInfo {
    crate::win::sys::collect()
}

#[cfg(not(windows))]
fn platform() -> SystemInfo {
    let read = |p: &str| std::fs::read_to_string(p).unwrap_or_default();
    let cpu = read("/proc/cpuinfo").lines().find(|l| l.starts_with("model name")).and_then(|l| l.split(':').nth(1)).map(|s| s.trim().to_string()).unwrap_or_default();
    let ram_kb: f64 = read("/proc/meminfo").lines().find(|l| l.starts_with("MemTotal")).and_then(|l| l.split_whitespace().nth(1)).and_then(|v| v.parse().ok()).unwrap_or(0.0);
    let os = read("/etc/os-release").lines().find(|l| l.starts_with("PRETTY_NAME=")).map(|l| l[12..].trim_matches('"').to_string()).unwrap_or_else(|| "Linux".into());
    SystemInfo { os, cpu, ram_gb: ram_kb / 1024.0 / 1024.0, ..Default::default() }
}
