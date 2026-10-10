//! Download -> SHA-256 check -> extract, with progress reporting.
//!
//! Layout under `base` (the exe folder):
//!   downloads/<archive>          temporary; deleted after a successful extract
//!   <asset.dest>/...             extracted files
//!   <asset.dest>/.<id>.ok        marker containing the verified sha256
//!
//! If the user drops a manually downloaded archive into `downloads/`, it is used
//! (after the same checksum check) instead of downloading again.
//!
//! Zip-member assets (one DLL out of a NuGet package) are fetched with HTTP range
//! requests: the zip's central directory, then only that member's compressed bytes.

use crate::catalog::Asset;
use anyhow::{bail, Context, Result};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub enum Phase {
    Download,
    Verify,
    Extract,
}

#[derive(Debug, Clone, Copy)]
pub struct Progress {
    pub phase: Phase,
    pub done: u64,
    pub total: u64,
    pub elapsed: Duration,
}

pub fn marker_path(base: &Path, a: &Asset) -> PathBuf {
    base.join(a.dest).join(format!(".{}.ok", a.id))
}

/// Installed = marker with the right digest AND the extracted file/folder still there
/// (so a folder deleted by hand or by antivirus is fetched again instead of failing later).
pub fn is_installed(base: &Path, a: &Asset) -> bool {
    fs::read_to_string(marker_path(base, a)).map(|s| s.trim() == a.sha256).unwrap_or(false)
        && base.join(a.dest).join(a.installed_path()).exists()
}

pub fn sha256_file(path: &Path, mut progress: impl FnMut(u64)) -> Result<String> {
    let mut f = BufReader::with_capacity(1 << 20, File::open(path)?);
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    let mut done = 0u64;
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
        done += n as u64;
        progress(done);
    }
    Ok(hex(&h.finalize()))
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// Make sure `a` is downloaded, verified and extracted. Returns the extraction folder.
pub fn ensure(base: &Path, a: &Asset, progress: &mut dyn FnMut(Progress)) -> Result<PathBuf> {
    let dest = base.join(a.dest);
    if is_installed(base, a) {
        return Ok(dest);
    }
    if let Some(m) = a.zip_member {
        fs::create_dir_all(&dest)?;
        fetch_zip_member(&a.url(), m, &dest.join(a.file), a, progress)?;
        fs::write(marker_path(base, a), a.sha256)?;
        return Ok(dest);
    }
    let dl_dir = base.join("downloads");
    fs::create_dir_all(&dl_dir)?;
    let archive = dl_dir.join(a.file);
    let t0 = Instant::now();

    let mut have_verified = false;
    if archive.is_file() {
        let total = fs::metadata(&archive)?.len();
        let sum = sha256_file(&archive, |d| {
            progress(Progress { phase: Phase::Verify, done: d, total, elapsed: t0.elapsed() })
        })?;
        if sum == a.sha256 {
            have_verified = true;
        } else {
            fs::remove_file(&archive).ok();
        }
    }
    if !have_verified {
        fetch(&a.url(), &archive, a, progress)?;
    }
    extract(&archive, &dest, a, progress)?;
    fs::write(marker_path(base, a), a.sha256)?;
    fs::remove_file(&archive).ok();
    Ok(dest)
}

fn agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .try_proxy_from_env(true)
        .timeout_connect(Duration::from_secs(20))
        .timeout_read(Duration::from_secs(60))
        .user_agent(concat!("s7-s9-input-voice/", env!("CARGO_PKG_VERSION")))
        .build()
}

/// GET `range` ("bytes=a-b" or "bytes=-n") of `url`; returns (body, total size of the file).
fn get_range(agent: &ureq::Agent, url: &str, range: &str, mut progress: impl FnMut(u64)) -> Result<(Vec<u8>, u64)> {
    let resp = agent.get(url).set("Range", range).call().with_context(|| format!("download {url}"))?;
    if resp.status() != 206 {
        bail!("{url} does not support partial downloads (HTTP {}); cannot fetch one file out of it", resp.status());
    }
    let total = resp
        .header("Content-Range")
        .and_then(|v| v.rsplit('/').next())
        .and_then(|v| v.trim().parse::<u64>().ok())
        .ok_or_else(|| anyhow::anyhow!("{url}: missing Content-Range"))?;
    let mut body = Vec::new();
    let mut r = resp.into_reader();
    let mut buf = vec![0u8; 256 * 1024];
    loop {
        let n = r.read(&mut buf).with_context(|| format!("reading {url}"))?;
        if n == 0 {
            break;
        }
        body.extend_from_slice(&buf[..n]);
        progress(body.len() as u64);
    }
    Ok((body, total))
}

fn le16(b: &[u8], i: usize) -> usize {
    u16::from_le_bytes([b[i], b[i + 1]]) as usize
}
fn le32(b: &[u8], i: usize) -> u64 {
    u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]) as u64
}

/// One entry of a zip central directory.
#[derive(Debug, PartialEq)]
struct ZipEntry {
    method: usize,
    compressed: u64,
    uncompressed: u64,
    local_header: u64,
}

/// Find the central directory (offset, size) from the tail of a zip.
fn zip_eocd(tail: &[u8]) -> Result<(u64, u64)> {
    let i = (0..tail.len().saturating_sub(21))
        .rev()
        .find(|&i| tail[i..i + 4] == [0x50, 0x4b, 0x05, 0x06])
        .ok_or_else(|| anyhow::anyhow!("not a zip file (no end-of-central-directory record)"))?;
    let (size, off) = (le32(tail, i + 12), le32(tail, i + 16));
    if size == 0xffff_ffff || off == 0xffff_ffff {
        bail!("zip64 archives are not supported");
    }
    Ok((off, size))
}

fn zip_find(cd: &[u8], name: &str) -> Result<ZipEntry> {
    let mut i = 0;
    while i + 46 <= cd.len() && cd[i..i + 4] == [0x50, 0x4b, 0x01, 0x02] {
        let (n, e, c) = (le16(cd, i + 28), le16(cd, i + 30), le16(cd, i + 32));
        if i + 46 + n > cd.len() {
            break;
        }
        if &cd[i + 46..i + 46 + n] == name.as_bytes() {
            return Ok(ZipEntry {
                method: le16(cd, i + 10),
                compressed: le32(cd, i + 20),
                uncompressed: le32(cd, i + 24),
                local_header: le32(cd, i + 42),
            });
        }
        i += 46 + n + e + c;
    }
    bail!("{name} is not in the package")
}

/// Download one member of a remote zip with range requests, inflate, check size + SHA-256,
/// then write it to `out` (via a .part file).
fn fetch_zip_member(
    url: &str,
    m: crate::catalog::ZipMember,
    out: &Path,
    a: &Asset,
    progress: &mut dyn FnMut(Progress),
) -> Result<()> {
    let agent = agent();
    let t0 = Instant::now();
    let (tail, total) = get_range(&agent, url, "bytes=-65557", |_| {})?;
    let tail_start = total - tail.len() as u64;
    let (cd_off, cd_size) = zip_eocd(&tail)?;
    let cd = if cd_off >= tail_start && cd_off + cd_size <= total {
        tail[(cd_off - tail_start) as usize..(cd_off - tail_start + cd_size) as usize].to_vec()
    } else {
        get_range(&agent, url, &format!("bytes={}-{}", cd_off, cd_off + cd_size - 1), |_| {})?.0
    };
    let e = zip_find(&cd, m.path)?;
    if e.uncompressed != m.uncompressed_size {
        bail!(
            "{} in {url} is {} bytes, expected {} - the package changed; not using it",
            m.path,
            e.uncompressed,
            m.uncompressed_size
        );
    }
    // Local header is 30 bytes + name + extra; the extra length can differ from the central
    // directory's, so over-fetch a little and parse it.
    let want = 30 + m.path.len() as u64 + 1024 + e.compressed;
    let end = (e.local_header + want).min(total) - 1;
    let mut last = Instant::now() - Duration::from_secs(1);
    let (blob, _) = get_range(&agent, url, &format!("bytes={}-{}", e.local_header, end), |d| {
        if last.elapsed() >= Duration::from_millis(200) {
            last = Instant::now();
            progress(Progress { phase: Phase::Download, done: d, total: want, elapsed: t0.elapsed() });
        }
    })?;
    progress(Progress {
        phase: Phase::Download,
        done: blob.len() as u64,
        total: blob.len() as u64,
        elapsed: t0.elapsed(),
    });
    if blob.len() < 30 || blob[..4] != [0x50, 0x4b, 0x03, 0x04] {
        bail!("bad zip local header for {}", m.path);
    }
    let start = 30 + le16(&blob, 26) + le16(&blob, 28);
    let data =
        blob.get(start..start + e.compressed as usize).ok_or_else(|| anyhow::anyhow!("short read for {}", m.path))?;
    let bytes = match e.method {
        0 => data.to_vec(),
        8 => {
            let mut v = Vec::with_capacity(e.uncompressed as usize);
            flate2::read::DeflateDecoder::new(data)
                .read_to_end(&mut v)
                .with_context(|| format!("inflate {}", m.path))?;
            v
        }
        x => bail!("unsupported zip compression method {x} for {}", m.path),
    };
    let sum = hex(&Sha256::digest(&bytes));
    if bytes.len() as u64 != m.uncompressed_size || sum != a.sha256 {
        bail!("checksum mismatch for {} from {url}: expected {}, got {sum} ({} bytes)", m.path, a.sha256, bytes.len());
    }
    let part = out.with_extension("part");
    fs::write(&part, &bytes).with_context(|| format!("write {}", part.display()))?;
    fs::rename(&part, out)?;
    Ok(())
}

fn fetch(url: &str, archive: &Path, a: &Asset, progress: &mut dyn FnMut(Progress)) -> Result<()> {
    let part = archive.with_extension("part");
    let agent = agent();
    let resp = agent.get(url).call().with_context(|| format!("download {url}"))?;
    let total = resp.header("Content-Length").and_then(|v| v.parse().ok()).unwrap_or(a.size);
    let mut reader = resp.into_reader();
    let mut out = File::create(&part).with_context(|| format!("create {}", part.display()))?;
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 256 * 1024];
    let mut done = 0u64;
    let t0 = Instant::now();
    let mut last = Instant::now() - Duration::from_secs(1);
    loop {
        let n = reader.read(&mut buf).with_context(|| format!("reading {url}"))?;
        if n == 0 {
            break;
        }
        out.write_all(&buf[..n])?;
        h.update(&buf[..n]);
        done += n as u64;
        if last.elapsed() >= Duration::from_millis(200) {
            last = Instant::now();
            progress(Progress { phase: Phase::Download, done, total, elapsed: t0.elapsed() });
        }
    }
    out.flush()?;
    drop(out);
    progress(Progress { phase: Phase::Download, done, total, elapsed: t0.elapsed() });
    let sum = hex(&h.finalize());
    if sum != a.sha256 {
        fs::remove_file(&part).ok();
        bail!("checksum mismatch for {}: expected {}, got {sum} ({done} bytes). The partial file was deleted; please retry.", a.file, a.sha256);
    }
    fs::rename(&part, archive)?;
    Ok(())
}

/// Counts compressed bytes read so extraction progress can be shown.
struct Counting<R> {
    inner: R,
    n: std::rc::Rc<std::cell::Cell<u64>>,
}
impl<R: Read> Read for Counting<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let k = self.inner.read(buf)?;
        self.n.set(self.n.get() + k as u64);
        Ok(k)
    }
}

pub fn extract(archive: &Path, dest: &Path, a: &Asset, progress: &mut dyn FnMut(Progress)) -> Result<()> {
    fs::create_dir_all(dest)?;
    let total = fs::metadata(archive)?.len();
    let counter = std::rc::Rc::new(std::cell::Cell::new(0u64));
    let f = Counting { inner: BufReader::with_capacity(1 << 20, File::open(archive)?), n: counter.clone() };
    let mut ar = tar::Archive::new(bzip2::read::BzDecoder::new(f));
    let t0 = Instant::now();
    let mut last = Instant::now() - Duration::from_secs(1);
    for entry in ar.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.to_string_lossy().replace('\\', "/");
        if a.skip_member(&path) {
            continue;
        }
        // unpack_in refuses paths that escape `dest`.
        entry.unpack_in(dest).with_context(|| format!("extract {path}"))?;
        if last.elapsed() >= Duration::from_millis(200) {
            last = Instant::now();
            progress(Progress { phase: Phase::Extract, done: counter.get(), total, elapsed: t0.elapsed() });
        }
    }
    progress(Progress { phase: Phase::Extract, done: total, total, elapsed: t0.elapsed() });
    Ok(())
}

/// Recursively find the first file whose name satisfies `pred` (breadth-first, sorted).
pub fn find_file(root: &Path, pred: &dyn Fn(&str) -> bool) -> Option<PathBuf> {
    let mut queue = vec![root.to_path_buf()];
    while !queue.is_empty() {
        let mut next = Vec::new();
        for dir in queue {
            let mut items: Vec<_> = fs::read_dir(&dir).ok()?.filter_map(|e| e.ok()).map(|e| e.path()).collect();
            items.sort();
            for p in &items {
                if p.is_file() && p.file_name().and_then(|n| n.to_str()).map(pred).unwrap_or(false) {
                    return Some(p.clone());
                }
            }
            next.extend(items.into_iter().filter(|p| p.is_dir()));
        }
        queue = next;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::{Kind, BLANK};

    fn make_archive(dir: &Path) -> (PathBuf, String) {
        let tar_path = dir.join("t.tar.bz2");
        let f = File::create(&tar_path).unwrap();
        let enc = bzip2::write::BzEncoder::new(f, bzip2::Compression::fast());
        let mut b = tar::Builder::new(enc);
        for (name, body) in [("t/lib/a.dll", "dll"), ("t/bin/demo.exe", "exe"), ("t/models/tokens.txt", "x 0")] {
            let mut h = tar::Header::new_gnu();
            h.set_size(body.len() as u64);
            h.set_mode(0o644);
            h.set_cksum();
            b.append_data(&mut h, name, body.as_bytes()).unwrap();
        }
        b.into_inner().unwrap().finish().unwrap();
        let sum = sha256_file(&tar_path, |_| {}).unwrap();
        (tar_path, sum)
    }

    #[test]
    fn manual_archive_is_verified_extracted_and_filtered() {
        let base = tempfile::tempdir().unwrap();
        let (src, sum) = make_archive(base.path());
        let sha: &'static str = Box::leak(sum.into_boxed_str());
        let a = Asset {
            id: "t",
            kind: Kind::Runtime,
            tag: "x",
            file: "t.tar.bz2",
            size: 1,
            sha256: sha,
            dest: "runtimes/cpu",
            label: "t",
            ..BLANK
        };
        fs::create_dir_all(base.path().join("downloads")).unwrap();
        fs::rename(&src, base.path().join("downloads/t.tar.bz2")).unwrap();
        let mut phases = Vec::new();
        let out = ensure(base.path(), &a, &mut |p| phases.push(p.phase as u8)).unwrap();
        assert!(out.join("t/lib/a.dll").is_file());
        assert!(!out.join("t/bin/demo.exe").exists(), "bin/ is skipped for runtimes");
        assert!(is_installed(base.path(), &a));
        assert!(!base.path().join("downloads/t.tar.bz2").exists(), "archive removed after extract");
        assert!(phases.contains(&(Phase::Verify as u8)) && phases.contains(&(Phase::Extract as u8)));
        let found = find_file(&out, &|n| n == "tokens.txt").unwrap();
        assert!(found.ends_with("t/models/tokens.txt"));
        // Second call is a no-op.
        ensure(base.path(), &a, &mut |_| panic!("no work expected")).unwrap();
        // Marker present but folder gone (deleted by hand / antivirus) -> not installed.
        fs::remove_dir_all(out.join("t")).unwrap();
        assert!(!is_installed(base.path(), &a));
    }

    #[test]
    fn zip_directory_parsing() {
        // Build a small zip by hand: one stored member, one deflated member.
        use std::io::Write as _;
        let mut z = Vec::new();
        let mut cd = Vec::new();
        let members: [(&str, Vec<u8>, u16); 2] = [("a.txt", b"hello".to_vec(), 0), ("dir/b.dll", vec![7u8; 5000], 8)];
        for (name, data, method) in &members {
            let comp = if *method == 8 {
                let mut e = flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
                e.write_all(data).unwrap();
                e.finish().unwrap()
            } else {
                data.clone()
            };
            let off = z.len() as u32;
            z.extend_from_slice(&[0x50, 0x4b, 0x03, 0x04, 20, 0, 0, 0]);
            z.extend_from_slice(&method.to_le_bytes());
            z.extend_from_slice(&[0; 8]);
            z.extend_from_slice(&(comp.len() as u32).to_le_bytes());
            z.extend_from_slice(&(data.len() as u32).to_le_bytes());
            z.extend_from_slice(&(name.len() as u16).to_le_bytes());
            z.extend_from_slice(&0u16.to_le_bytes());
            z.extend_from_slice(name.as_bytes());
            z.extend_from_slice(&comp);
            cd.extend_from_slice(&[0x50, 0x4b, 0x01, 0x02, 20, 0, 20, 0, 0, 0]);
            cd.extend_from_slice(&method.to_le_bytes());
            cd.extend_from_slice(&[0; 8]);
            cd.extend_from_slice(&(comp.len() as u32).to_le_bytes());
            cd.extend_from_slice(&(data.len() as u32).to_le_bytes());
            cd.extend_from_slice(&(name.len() as u16).to_le_bytes());
            cd.extend_from_slice(&[0; 12]);
            cd.extend_from_slice(&off.to_le_bytes());
            cd.extend_from_slice(name.as_bytes());
        }
        let cd_off = z.len() as u32;
        z.extend_from_slice(&cd);
        z.extend_from_slice(&[0x50, 0x4b, 0x05, 0x06, 0, 0, 0, 0, 2, 0, 2, 0]);
        z.extend_from_slice(&(cd.len() as u32).to_le_bytes());
        z.extend_from_slice(&cd_off.to_le_bytes());
        z.extend_from_slice(&[0, 0]);
        let (off, size) = zip_eocd(&z).unwrap();
        assert_eq!(off, cd_off as u64);
        let e = zip_find(&z[off as usize..(off + size) as usize], "dir/b.dll").unwrap();
        assert_eq!((e.method, e.uncompressed), (8, 5000));
        assert!(zip_find(&z[off as usize..(off + size) as usize], "nope").is_err());
        assert!(zip_eocd(b"not a zip at all, definitely not").is_err());
    }

    #[test]
    fn bad_manual_archive_is_discarded() {
        let base = tempfile::tempdir().unwrap();
        fs::create_dir_all(base.path().join("downloads")).unwrap();
        fs::write(base.path().join("downloads/t.tar.bz2"), b"junk").unwrap();
        let a = Asset {
            id: "t",
            kind: Kind::Stt,
            tag: "x",
            file: "t.tar.bz2",
            size: 1,
            sha256: "00",
            dest: "models",
            label: "t",
            ..BLANK
        };
        // Verification fails -> file removed -> download attempted from a bogus URL -> error.
        let r = ensure(base.path(), &a, &mut |_| {});
        assert!(r.is_err());
        assert!(!base.path().join("downloads/t.tar.bz2").exists());
    }
}
