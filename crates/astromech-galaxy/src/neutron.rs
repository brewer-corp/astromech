//! The neutron star file the local neutron plotter searches: every known neutron star's
//! position, grouped by grid cell, with names kept on disk until a route's stops need them.
//! `scripts/gen-neutron.py` builds it from Spansh's neutron dump; the app downloads it as
//! `neutron.bin` into the data folder.
//!
//! Format, version 2 (all integers little-endian; the script's docstring is the same spec):
//!
//! | Part | Size | Contents |
//! | --- | --- | --- |
//! | header | 36 bytes | magic `ASTRNEUT`, u32 version, u32 cell edge in ly (128), u32 star count N, u32 cell count C, i64 build time (Unix seconds, UTC), u32 name group G (256) |
//! | cells | C × 10 bytes | i16 cx, i16 cy, i16 cz, u32 first star; non-empty cells only, sorted by (cx, cy, cz) |
//! | positions | N × 12 bytes | i32 x, y, z in 1/32 ly, in cell order, and within a cell by (x, y, z, name) |
//! | name index | ⌈N / G⌉ × u32 | byte offset in the name text of star G·i's name |
//! | name text | to the end | the names in star order, UTF-8, each followed by `\n` |
//!
//! A star at (x, y, z) in 1/32 ly is in cell `floor(q / (edge · 32))` per axis, and a cell's
//! stars are the contiguous range from its first star to the next cell's (N for the last).
//! Neighbours within r ly of a point are found by scanning the cells that overlap the sphere
//! ([`NeutronStars::cell_of`], [`NeutronStars::cell`]). Opening the file loads the header,
//! cells and positions (about 75 MB for 5 M stars); [`NeutronStars::name`] reads one name
//! from disk.
//!
//! The `neutron` pre-release publishes the file gzipped, with its SHA-256
//! (`.github/workflows/neutron.yml`). [`download`] fetches it into the data folder only when the
//! commander asks: the gzip is checked and unpacked into a temporary file on the way in, which
//! is renamed over `neutron.bin` only once the checksum and header are right, so a failed or
//! interrupted download leaves the old file (or none) in place.

use anyhow::{bail, ensure, Context, Result};
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::ops::Range;
use std::path::{Path, PathBuf};

/// The file's name in the data folder.
pub const FILE_NAME: &str = "neutron.bin";

const MAGIC: &[u8; 8] = b"ASTRNEUT";
const VERSION: u32 = 2;
const HEADER_LEN: u64 = 36;
const CELL_LEN: usize = 10;
const POS_LEN: usize = 12;

/// What the header says.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct Header {
    /// Grid cell edge, light years.
    pub cell_ly: u32,
    pub stars: u32,
    pub cells: u32,
    /// Build time, Unix seconds UTC.
    pub built: i64,
    /// Names per entry in the name index.
    pub name_group: u32,
}

impl Header {
    fn parse(b: &[u8; HEADER_LEN as usize]) -> Result<Self> {
        ensure!(&b[..8] == MAGIC, "not a neutron star file");
        let version = u32_at(b, 8);
        ensure!(version == VERSION, "neutron star file version {version}; this build reads {VERSION}");
        let h = Header {
            cell_ly: u32_at(b, 12),
            stars: u32_at(b, 16),
            cells: u32_at(b, 20),
            built: i64::from_le_bytes(b[24..32].try_into().expect("8 bytes")),
            name_group: u32_at(b, 32),
        };
        ensure!(h.cell_ly > 0 && h.name_group > 0, "neutron star file has a zero cell edge or name group");
        Ok(h)
    }

    fn name_groups(&self) -> usize {
        (self.stars as usize).div_ceil(self.name_group as usize)
    }

    /// Where the name index starts.
    fn names_at(&self) -> u64 {
        HEADER_LEN + (self.cells as usize * CELL_LEN + self.stars as usize * POS_LEN) as u64
    }
}

/// Reads only the header (for the build date and star count without loading the stars).
pub fn read_header(path: &Path) -> Result<Header> {
    let mut b = [0u8; HEADER_LEN as usize];
    File::open(path).and_then(|mut f| f.read_exact(&mut b)).with_context(|| format!("reading {}", path.display()))?;
    Header::parse(&b)
}

fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().expect("4 bytes"))
}

fn i32_at(b: &[u8], at: usize) -> i32 {
    i32::from_le_bytes(b[at..at + 4].try_into().expect("4 bytes"))
}

fn i16_at(b: &[u8], at: usize) -> i16 {
    i16::from_le_bytes(b[at..at + 2].try_into().expect("2 bytes"))
}

/// The neutron star file, loaded except for the names.
pub struct NeutronStars {
    path: PathBuf,
    header: Header,
    /// Cell keys and first stars, sorted by key.
    cells: Vec<([i16; 3], u32)>,
    /// Positions in 1/32 ly.
    pos: Vec<[i32; 3]>,
    /// Byte offsets of each group of names in the name text.
    groups: Vec<u32>,
    file_len: u64,
}

impl NeutronStars {
    pub fn open(path: &Path) -> Result<Self> {
        let ctx = || format!("reading {}", path.display());
        let f = File::open(path).with_context(ctx)?;
        let file_len = f.metadata().with_context(ctx)?.len();
        let mut f = BufReader::new(f);
        let mut hb = [0u8; HEADER_LEN as usize];
        f.read_exact(&mut hb).with_context(ctx)?;
        let header = Header::parse(&hb)?;
        let (n, c) = (header.stars as usize, header.cells as usize);
        let text_at = header.names_at() + 4 * header.name_groups() as u64;
        ensure!(file_len >= text_at, "neutron star file is cut short ({file_len} bytes)");

        let mut b = vec![0u8; c * CELL_LEN];
        f.read_exact(&mut b).with_context(ctx)?;
        let cells: Vec<([i16; 3], u32)> = b
            .as_chunks::<CELL_LEN>()
            .0
            .iter()
            .map(|e| ([i16_at(e, 0), i16_at(e, 2), i16_at(e, 4)], u32_at(e, 6)))
            .collect();
        let ordered = cells.windows(2).all(|w| w[0].0 < w[1].0 && w[0].1 < w[1].1);
        ensure!(ordered && cells.first().is_none_or(|e| e.1 == 0), "neutron star file cells are out of order");
        ensure!(cells.last().is_none_or(|e| (e.1 as usize) < n), "neutron star file cells overrun the stars");

        let mut b = vec![0u8; n * POS_LEN];
        f.read_exact(&mut b).with_context(ctx)?;
        let pos = b.as_chunks::<POS_LEN>().0.iter().map(|e| [i32_at(e, 0), i32_at(e, 4), i32_at(e, 8)]).collect();

        let mut b = vec![0u8; 4 * header.name_groups()];
        f.read_exact(&mut b).with_context(ctx)?;
        let groups = b.as_chunks::<4>().0.iter().map(|e| u32::from_le_bytes(*e)).collect();
        Ok(NeutronStars { path: path.to_path_buf(), header, cells, pos, groups, file_len })
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn len(&self) -> usize {
        self.pos.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pos.is_empty()
    }

    /// Star `i`'s position in ly.
    pub fn pos(&self, i: usize) -> [f64; 3] {
        self.pos[i].map(|q| f64::from(q) / 32.0)
    }

    /// Star `i`'s position in 1/32 ly, as stored.
    pub fn pos32(&self, i: usize) -> [i32; 3] {
        self.pos[i]
    }

    /// The cell holding a point in ly.
    pub fn cell_of(&self, p: [f64; 3]) -> [i32; 3] {
        let edge = f64::from(self.header.cell_ly);
        p.map(|v| (v / edge).floor() as i32)
    }

    /// The stars in a cell (empty if it has none).
    pub fn cell(&self, key: [i32; 3]) -> Range<usize> {
        let (Ok(x), Ok(y), Ok(z)) = (i16::try_from(key[0]), i16::try_from(key[1]), i16::try_from(key[2])) else {
            return 0..0;
        };
        let key = [x, y, z];
        match self.cells.binary_search_by(|e| e.0.cmp(&key)) {
            Ok(i) => {
                let end = self.cells.get(i + 1).map_or(self.len(), |e| e.1 as usize);
                self.cells[i].1 as usize..end
            }
            Err(_) => 0..0,
        }
    }

    /// Star `i`'s name, read from disk.
    pub fn name(&self, i: usize) -> Result<String> {
        ensure!(i < self.len(), "no neutron star {i}");
        let group = self.header.name_group as usize;
        let (g, k) = (i / group, i % group);
        let text_at = self.header.names_at() + 4 * self.groups.len() as u64;
        let start = text_at + u64::from(self.groups[g]);
        let end = self.groups.get(g + 1).map_or(self.file_len, |o| text_at + u64::from(*o));
        ensure!(start <= end && end <= self.file_len, "neutron star file name index is corrupt");
        let mut f = File::open(&self.path).with_context(|| format!("reading {}", self.path.display()))?;
        f.seek(SeekFrom::Start(start))?;
        let mut b = vec![0u8; (end - start) as usize];
        f.read_exact(&mut b)?;
        let Some(name) = b.split_inclusive(|c| *c == b'\n').nth(k).and_then(|s| s.strip_suffix(b"\n")) else {
            bail!("neutron star file has no name for star {i}");
        };
        String::from_utf8(name.to_vec()).context("neutron star name isn't UTF-8")
    }
}

/// Where the `neutron` pre-release's files are.
pub const RELEASE_URL: &str = "https://github.com/brewer-corp/astromech/releases/download/neutron";
/// The published gzip and its checksum file (`sha256sum` output).
const GZ_NAME: &str = "neutron.bin.gz";
/// Next to `neutron.bin`: the SHA-256 of the gzip it was unpacked from, to tell a newer one.
const SUM_NAME: &str = "neutron.bin.sha256";
/// The download unpacks here, then is renamed to `neutron.bin`.
const PART_NAME: &str = "neutron.bin.part";

/// The file in `dir` (the data folder): `None` if there is none, an error if it can't be read.
pub fn installed(dir: &Path) -> Result<Option<Header>> {
    let path = dir.join(FILE_NAME);
    if !path.exists() {
        return Ok(None);
    }
    read_header(&path).map(Some)
}

/// The SHA-256 (lowercase hex) of the gzip published at `base`.
pub fn latest_sha256(base: &str) -> Result<String> {
    let text = astromech_data::download::text(&format!("{base}/{GZ_NAME}.sha256"))?;
    let sum = text.split_whitespace().next().unwrap_or_default().to_ascii_lowercase();
    ensure!(sum.len() == 64 && sum.bytes().all(|b| b.is_ascii_hexdigit()), "The published checksum isn't readable.");
    Ok(sum)
}

/// Whether the file published at `base` differs from the one in `dir` (true if there is none).
pub fn newer_available(dir: &Path, base: &str) -> Result<bool> {
    let latest = latest_sha256(base)?;
    let have = std::fs::read_to_string(dir.join(SUM_NAME)).unwrap_or_default();
    Ok(!dir.join(FILE_NAME).exists() || have.trim() != latest)
}

/// Removes the partial file unless the download got as far as the rename.
struct Part<'a>(&'a Path);

impl Drop for Part<'_> {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(self.0);
    }
}

/// Hashes the gzip as it arrives and unpacks it into the partial file.
struct Unpack {
    sha: sha2::Sha256,
    gz: flate2::write::GzDecoder<std::io::BufWriter<File>>,
}

impl std::io::Write for Unpack {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        let n = self.gz.write(b)?;
        sha2::Digest::update(&mut self.sha, &b[..n]);
        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.gz.flush()
    }
}

/// Downloads the file published at `base` into `dir` as `neutron.bin`, calling
/// `progress(bytes, total)` as the gzip comes in. The old file stays until the new one is
/// complete and its checksum matches.
pub fn download(dir: &Path, base: &str, progress: impl FnMut(u64, Option<u64>)) -> Result<Header> {
    use std::io::Write;
    let sum = latest_sha256(base)?;
    std::fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
    let part = dir.join(PART_NAME);
    let _cleanup = Part(&part);
    let file = File::create(&part).with_context(|| format!("writing {}", part.display()))?;
    let mut w = Unpack { sha: sha2::Digest::new(), gz: flate2::write::GzDecoder::new(std::io::BufWriter::new(file)) };
    match astromech_data::download::stream(&format!("{base}/{GZ_NAME}"), &mut w, progress) {
        Ok(_) => {}
        Err(astromech_data::download::DownloadError::Write(e)) => {
            bail!("The download couldn't be written or unpacked ({e}). Try again.")
        }
        Err(e) => return Err(e.into()),
    }
    let got = hex(&sha2::Digest::finalize(w.sha));
    ensure!(got == sum, "The download was damaged (its checksum doesn't match). Try again.");
    let unpacked = (|| {
        let mut out = w.gz.finish()?;
        out.flush()?;
        out.into_inner().map_err(|e| e.into_error())?.sync_all()
    })();
    unpacked.context("The download couldn't be unpacked. Try again.")?;
    let header = read_header(&part).context("The downloaded file isn't a neutron star file this version reads.")?;
    std::fs::rename(&part, dir.join(FILE_NAME))
        .with_context(|| format!("replacing {}", dir.join(FILE_NAME).display()))?;
    std::fs::write(dir.join(SUM_NAME), &sum).with_context(|| format!("writing {}", dir.join(SUM_NAME).display()))?;
    Ok(header)
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
