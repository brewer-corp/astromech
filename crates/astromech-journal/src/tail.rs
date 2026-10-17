//! Follow one journal file from a byte offset, as the game appends to it.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

/// Tail one journal file from a byte offset. The file is opened on every read, so the length is
/// the current one even where a directory listing lags (Windows, astromech-txzq).
#[derive(Debug)]
pub struct JournalTail {
    pub path: PathBuf,
    pub offset: u64,
    partial: Vec<u8>,
    /// How many times the file turned out shorter than the offset and was read again from the start.
    restarts: u32,
}

impl JournalTail {
    pub fn new(path: PathBuf, offset: u64) -> Self {
        Self { path, offset, partial: vec![], restarts: 0 }
    }

    /// Complete new lines since the last call.
    pub fn read_new(&mut self) -> Vec<String> {
        self.read_to(u64::MAX)
    }

    /// Complete lines from the current offset up to byte `limit` of the file; the rest stays for
    /// the next call. With `limit` at a line start (a [`JournalTail::committed`] of an earlier
    /// read) the split falls between two lines.
    pub fn read_to(&mut self, limit: u64) -> Vec<String> {
        let Ok(mut f) = File::open(&self.path) else { return vec![] };
        let Ok(len) = f.metadata().map(|m| m.len()) else { return vec![] };
        if len < self.offset {
            // Truncated or replaced: start over.
            self.offset = 0;
            self.partial.clear();
            self.restarts += 1;
        }
        if f.seek(SeekFrom::Start(self.offset)).is_err() {
            return vec![];
        }
        let mut buf = vec![];
        if f.take(limit.saturating_sub(self.offset)).read_to_end(&mut buf).is_err() {
            return vec![];
        }
        self.offset += buf.len() as u64;
        self.partial.extend_from_slice(&buf);
        let mut out = vec![];
        while let Some(p) = self.partial.iter().position(|b| *b == b'\n') {
            let line: Vec<u8> = self.partial.drain(..=p).collect();
            let s = String::from_utf8_lossy(&line).trim().to_string();
            if !s.is_empty() {
                out.push(s);
            }
        }
        out
    }

    /// Offset of the last complete line (what we persist).
    pub fn committed(&self) -> u64 {
        self.offset - self.partial.len() as u64
    }

    /// The number of times the file shrank under the tail and was read again from the start.
    pub fn restarts(&self) -> u32 {
        self.restarts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn tail_keeps_partial_lines() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("Journal.2026-10-10T120000.01.log");
        let mut f = std::fs::File::create(&p).unwrap();
        write!(f, "{{\"timestamp\":\"t\",\"event\":\"A\"}}\r\n{{\"timestamp\":\"t\",\"ev").unwrap();
        f.flush().unwrap();
        let mut t = JournalTail::new(p.clone(), 0);
        assert_eq!(t.read_new().len(), 1);
        let c = t.committed();
        write!(f, "ent\":\"B\"}}\r\n").unwrap();
        f.flush().unwrap();
        let l = t.read_new();
        assert_eq!(l, vec!["{\"timestamp\":\"t\",\"event\":\"B\"}".to_string()]);
        assert!(t.committed() > c);
    }

    #[test]
    fn read_to_stops_at_the_limit_and_read_new_takes_the_rest() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("Journal.2026-10-10T120000.01.log");
        std::fs::write(&p, "{\"event\":\"A\"}\n{\"event\":\"B\"}\n{\"event\":\"C\"}\n").unwrap();
        let first = std::fs::metadata(&p).unwrap().len() / 3 * 2;
        let mut t = JournalTail::new(p, 0);
        assert_eq!(t.read_to(first).len(), 2);
        assert_eq!(t.read_new(), vec!["{\"event\":\"C\"}".to_string()]);
    }

    #[test]
    fn a_file_that_shrinks_is_read_again_from_the_start() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("Journal.2026-10-10T120000.01.log");
        std::fs::write(&p, "{\"event\":\"A\"}\n{\"event\":\"B\"}\n").unwrap();
        let mut t = JournalTail::new(p.clone(), 0);
        assert_eq!(t.read_new().len(), 2);
        std::fs::write(&p, "{\"event\":\"C\"}\n").unwrap();
        assert_eq!(t.read_new(), vec!["{\"event\":\"C\"}".to_string()]);
        assert_eq!(t.restarts(), 1);
    }
}
