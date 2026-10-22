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
//! ([`Stars::cell_of`], [`Stars::cell`]). Opening the file loads the header, cells and positions
//! (about 80 MB for 5 M stars) and closes it; [`NeutronStars::names`] reads a route's names from
//! disk, after checking the file is still the one loaded. `crate::neutron_plot` searches it.
//! The app keeps it loaded in a [`Kept`], which loads it again when a download replaces it.
//!
//! The `neutron` pre-release publishes the file gzipped, with its SHA-256
//! (`.github/workflows/neutron.yml`). [`download`] fetches it into the data folder only when the
//! commander asks: the gzip is checked and unpacked into a temporary file on the way in, which
//! is renamed over `neutron.bin` only once the checksum and header are right, so a failed or
//! interrupted download leaves the old file (or none) in place.

use anyhow::{anyhow, bail, ensure, Context, Result};
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
    File::open(path)
        .and_then(|mut f| f.read_exact(&mut b))
        .map_err(|e| anyhow!("Couldn't read {}: {e}.", path.display()))?;
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

/// Neutron star positions grouped by grid cell: what the plotter searches.
pub struct Stars {
    /// Grid cell edge, light years.
    cell_ly: u32,
    /// Cell keys and first stars, sorted by key.
    cells: Vec<([i16; 3], u32)>,
    /// Positions in 1/32 ly, in cell order.
    pos: Vec<[i32; 3]>,
    /// The non-empty (x, y) columns of cells, sorted, each with its first cell; the next
    /// column's first (or the cell count) ends it.
    columns: Vec<([i16; 2], u32)>,
}

impl Stars {
    /// An index over positions in 1/32 ly, ordered as the file orders them (so star `i` here is
    /// not the `i`-th position given). For stars that don't come from a file.
    pub fn new(cell_ly: u32, mut pos: Vec<[i32; 3]>) -> Result<Self> {
        ensure!(cell_ly > 0, "zero cell edge");
        let span = cell_ly as i32 * 32;
        let key = |q: &[i32; 3]| q.map(|v| v.div_euclid(span));
        pos.sort_by_key(|q| (key(q), *q));
        let mut cells: Vec<([i16; 3], u32)> = Vec::new();
        for (i, q) in pos.iter().enumerate() {
            let fits = |v: i32| i16::try_from(v).map_err(|_| anyhow!("star at {q:?} (1/32 ly) is outside the grid"));
            let [x, y, z] = key(q);
            let k = [fits(x)?, fits(y)?, fits(z)?];
            if cells.last().is_none_or(|c| c.0 != k) {
                cells.push((k, i as u32));
            }
        }
        Ok(Stars::index(cell_ly, cells, pos))
    }

    /// Adds the column index to sorted cells.
    fn index(cell_ly: u32, cells: Vec<([i16; 3], u32)>, pos: Vec<[i32; 3]>) -> Self {
        let mut columns: Vec<([i16; 2], u32)> = Vec::new();
        for (i, c) in cells.iter().enumerate() {
            let xy = [c.0[0], c.0[1]];
            if columns.last().is_none_or(|l| l.0 != xy) {
                columns.push((xy, i as u32));
            }
        }
        Stars { cell_ly, cells, pos, columns }
    }

    pub fn len(&self) -> usize {
        self.pos.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pos.is_empty()
    }

    /// Grid cell edge, light years.
    pub fn cell_ly(&self) -> u32 {
        self.cell_ly
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
        let edge = f64::from(self.cell_ly);
        p.map(|v| (v / edge).floor() as i32)
    }

    /// The non-empty cells from `lo` to `hi` inclusive (cell keys), with their stars: per row
    /// of x, a binary search for the columns that exist, then per column one for the cells.
    pub fn cells_in(&self, lo: [i32; 3], hi: [i32; 3]) -> impl Iterator<Item = ([i32; 3], Range<usize>)> + '_ {
        let fit = |v: i32| v.clamp(i16::MIN.into(), i16::MAX.into()) as i16;
        let (lo16, hi16) = (lo.map(fit), hi.map(fit));
        (lo16[0]..=hi16[0])
            .flat_map(move |x| {
                let first = self.columns.partition_point(|c| c.0 < [x, lo16[1]]);
                self.columns[first..]
                    .iter()
                    .enumerate()
                    .map_while(move |(k, c)| (c.0 <= [x, hi16[1]]).then_some(first + k))
            })
            .flat_map(move |k| {
                let end = self.columns.get(k + 1).map_or(self.cells.len(), |c| c.1 as usize);
                let column = &self.cells[self.columns[k].1 as usize..end];
                let start = self.columns[k].1 as usize + column.partition_point(|e| e.0[2] < lo16[2]);
                (start..end).map_while(move |i| {
                    let key = self.cells[i].0;
                    (key[2] <= hi16[2]).then(|| {
                        let stop = self.cells.get(i + 1).map_or(self.len(), |e| e.1 as usize);
                        (key.map(i32::from), self.cells[i].1 as usize..stop)
                    })
                })
            })
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
}

/// The neutron star file, loaded except for the names. It holds no handle on the file between
/// calls: the download can replace the file while it is loaded (on Windows an open handle would
/// block that), and [`NeutronStars::names`] refuses to read names from a file that is no longer
/// the one loaded.
pub struct NeutronStars {
    path: PathBuf,
    header: Header,
    stars: Stars,
    /// Byte offsets of each group of names in the name text.
    groups: Vec<u32>,
    file_len: u64,
}

impl NeutronStars {
    pub fn open(path: &Path) -> Result<Self> {
        let ctx = || format!("Couldn't read {}", path.display());
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
        let stars = Stars::index(header.cell_ly, cells, pos);
        Ok(NeutronStars { path: path.to_path_buf(), header, stars, groups, file_len })
    }

    pub fn header(&self) -> Header {
        self.header
    }

    /// The positions, to search.
    pub fn stars(&self) -> &Stars {
        &self.stars
    }

    /// Whether the file on disk is still the one loaded (false if it is gone or can't be read).
    pub fn is_current(&self) -> bool {
        File::open(&self.path).and_then(|mut f| self.same_file(&mut f)).unwrap_or(false)
    }

    /// The same header and length as the file loaded: a newer download has a newer build time or
    /// other counts. Reads the header from the start of `f`.
    fn same_file(&self, f: &mut File) -> std::io::Result<bool> {
        let mut hb = [0u8; HEADER_LEN as usize];
        f.read_exact(&mut hb)?;
        Ok(Header::parse(&hb).is_ok_and(|h| h == self.header) && f.metadata()?.len() == self.file_len)
    }

    /// Star `i`'s name, read from disk ([`NeutronStars::names`]).
    pub fn name(&self, i: usize) -> Result<String> {
        Ok(self.names(&[i])?.remove(0))
    }

    /// The names of stars `ids`, read from disk through one handle. The file must still be the
    /// one loaded: the same header and length (a newer download gives a newer build time or
    /// other counts). If it has been replaced, the error says to plot again, which loads the new
    /// file.
    pub fn names(&self, ids: &[usize]) -> Result<Vec<String>> {
        let ctx = || format!("Couldn't read {}", self.path.display());
        let mut f = File::open(&self.path).with_context(ctx)?;
        ensure!(
            self.same_file(&mut f).with_context(ctx)?,
            "The neutron star file changed while it was loaded (a new download). Plot the route again."
        );
        let group = self.header.name_group as usize;
        let text_at = self.header.names_at() + 4 * self.groups.len() as u64;
        ids.iter()
            .map(|&i| {
                ensure!(i < self.stars.len(), "no neutron star {i}");
                let (g, k) = (i / group, i % group);
                let start = text_at + u64::from(self.groups[g]);
                let end = self.groups.get(g + 1).map_or(self.file_len, |o| text_at + u64::from(*o));
                ensure!(start <= end && end <= self.file_len, "neutron star file name index is corrupt");
                f.seek(SeekFrom::Start(start)).with_context(ctx)?;
                let mut b = vec![0u8; (end - start) as usize];
                f.read_exact(&mut b).with_context(ctx)?;
                let Some(name) = b.split_inclusive(|c| *c == b'\n').nth(k).and_then(|s| s.strip_suffix(b"\n")) else {
                    bail!("neutron star file has no name for star {i}");
                };
                String::from_utf8(name.to_vec()).context("neutron star name isn't UTF-8")
            })
            .collect()
    }
}

/// The neutron star file in a data folder as the app keeps it: loaded on first use (0.1–0.2 s)
/// and held (about 80 MB), and loaded again when the file on disk is no longer the one loaded,
/// so a download that replaced it is picked up by the next [`Kept::get`]. A file that fails to
/// load is not tried again until it changes.
pub struct Kept {
    dir: PathBuf,
    state: std::sync::Mutex<KeptState>,
}

#[derive(Default)]
struct KeptState {
    file: Option<std::sync::Arc<NeutronStars>>,
    /// The last load failed, for the file with this length and modified time.
    failed: Option<((u64, std::time::SystemTime), String)>,
}

/// A file's length and modified time, to tell whether a file that failed to load has changed.
fn stamp(path: &Path) -> Option<(u64, std::time::SystemTime)> {
    let m = std::fs::metadata(path).ok()?;
    Some((m.len(), m.modified().ok()?))
}

impl Kept {
    /// The file `neutron.bin` in `dir` (the data folder). Nothing is read until [`Kept::get`].
    pub fn new(dir: &Path) -> Self {
        Kept { dir: dir.to_path_buf(), state: Default::default() }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, KeptState> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// The loaded file, loading it first if it isn't loaded or has changed on disk. `None` if
    /// there is no file; an error (a sentence) if it can't be read.
    pub fn get(&self) -> Result<Option<std::sync::Arc<NeutronStars>>> {
        let mut s = self.lock();
        if let Some(f) = s.file.as_ref().filter(|f| f.is_current()) {
            return Ok(Some(f.clone()));
        }
        s.file = None; // free the old one before loading the new
        let path = self.dir.join(FILE_NAME);
        if !path.exists() {
            s.failed = None;
            return Ok(None);
        }
        let now = stamp(&path);
        if let Some((_, e)) = s.failed.as_ref().filter(|(at, _)| Some(*at) == now) {
            bail!("{e}");
        }
        match NeutronStars::open(&path) {
            Ok(f) => {
                s.failed = None;
                let f = std::sync::Arc::new(f);
                s.file = Some(f.clone());
                Ok(Some(f))
            }
            Err(e) => {
                let e = format!("{e:#}");
                s.failed = now.map(|at| (at, e.clone()));
                bail!("{e}")
            }
        }
    }

    /// [`installed`], unless the file failed to load since it last changed: then why.
    pub fn installed(&self) -> Result<Option<Header>> {
        let found = installed(&self.dir)?;
        let s = self.lock();
        if let Some((at, e)) = &s.failed {
            if found.is_some() && stamp(&self.dir.join(FILE_NAME)) == Some(*at) {
                bail!("{e}");
            }
        }
        Ok(found)
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
/// A partial file left by a download that was cut off (the app closed mid-download) is removed.
pub fn installed(dir: &Path) -> Result<Option<Header>> {
    if !Downloading::running(dir) {
        let _ = std::fs::remove_file(dir.join(PART_NAME));
    }
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

/// The data folders a download is running into in this process: one at a time per folder, and
/// [`installed`] leaves its partial file alone.
static DOWNLOADS: std::sync::Mutex<Vec<PathBuf>> = std::sync::Mutex::new(Vec::new());

struct Downloading(PathBuf);

impl Downloading {
    fn start(dir: &Path) -> Result<Self> {
        let mut d = DOWNLOADS.lock().unwrap_or_else(|e| e.into_inner());
        ensure!(!d.iter().any(|x| x == dir), "The neutron star file is already downloading.");
        d.push(dir.to_path_buf());
        Ok(Downloading(dir.to_path_buf()))
    }

    fn running(dir: &Path) -> bool {
        DOWNLOADS.lock().unwrap_or_else(|e| e.into_inner()).iter().any(|x| x == dir)
    }
}

impl Drop for Downloading {
    fn drop(&mut self) {
        DOWNLOADS.lock().unwrap_or_else(|e| e.into_inner()).retain(|x| *x != self.0);
    }
}

/// Removes the partial file (a stale one from a download that was cut off, or this one) unless
/// the download got as far as the rename.
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
/// complete and its checksum matches. Errors are sentences for the Route page, with the cause
/// and what to do.
pub fn download(dir: &Path, base: &str, progress: impl FnMut(u64, Option<u64>)) -> Result<Header> {
    use std::io::Write;
    let _running = Downloading::start(dir)?;
    let part = dir.join(PART_NAME);
    let _cleanup = Part(&part);
    let sum = latest_sha256(base)?;
    const SPACE: &str = "Check there is about 200 MB free in the data folder, then press Download again.";
    std::fs::create_dir_all(dir)
        .map_err(|e| anyhow!("Couldn't create the data folder {}: {e}. {SPACE}", dir.display()))?;
    let file = File::create(&part).map_err(|e| anyhow!("Couldn't write {}: {e}. {SPACE}", part.display()))?;
    let mut w = Unpack { sha: sha2::Digest::new(), gz: flate2::write::GzDecoder::new(std::io::BufWriter::new(file)) };
    match astromech_data::download::stream(&format!("{base}/{GZ_NAME}"), &mut w, progress) {
        Ok(_) => {}
        Err(astromech_data::download::DownloadError::Write(e)) => {
            bail!("The download couldn't be written or unpacked: {e}. {SPACE}")
        }
        Err(e) => return Err(e.into()),
    }
    let got = hex(&sha2::Digest::finalize(w.sha));
    ensure!(got == sum, "The download was damaged (its checksum doesn't match). Press Download again.");
    let unpacked = (|| {
        let mut out = w.gz.finish()?;
        out.flush()?;
        out.into_inner().map_err(|e| e.into_error())?.sync_all()
    })();
    unpacked.map_err(|e| anyhow!("The download couldn't be unpacked: {e}. {SPACE}"))?;
    let header = read_header(&part).map_err(|e| {
        anyhow!("The downloaded file isn't one this version of astromech reads ({e:#}). Update astromech, then press Download again.")
    })?;
    let path = dir.join(FILE_NAME);
    std::fs::rename(&part, &path).map_err(|e| {
        anyhow!(
            "Couldn't replace {}: {e}. Another program (an antivirus scan, or a route being plotted) may have it open. Wait a minute, then press Download again.",
            path.display()
        )
    })?;
    // Without it, Check for a newer file only says there is one; the file itself is in place.
    let _ = std::fs::write(dir.join(SUM_NAME), &sum);
    Ok(header)
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
