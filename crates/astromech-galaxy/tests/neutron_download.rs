//! Downloading the neutron star file from a local HTTP server standing in for the `neutron`
//! pre-release: a good download, a wrong checksum, a missing file. No test touches the network.

use astromech_galaxy::neutron::{download, installed, newer_available, FILE_NAME};
use sha2::{Digest, Sha256};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::path::Path;

/// Serves `files` (path, body) on 127.0.0.1 until the test ends; anything else is a 404.
fn serve(files: Vec<(&'static str, Vec<u8>)>) -> String {
    let l = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", l.local_addr().unwrap());
    std::thread::spawn(move || {
        for s in l.incoming() {
            let Ok(mut s) = s else { continue };
            let mut r = BufReader::new(s.try_clone().unwrap());
            let mut first = String::new();
            r.read_line(&mut first).unwrap();
            let mut line = String::new();
            while r.read_line(&mut line).unwrap() > 2 {
                line.clear();
            }
            let path = first.split_whitespace().nth(1).unwrap_or("");
            match files.iter().find(|(p, _)| *p == path) {
                Some((_, body)) => {
                    write!(s, "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len())
                        .unwrap();
                    s.write_all(body).unwrap();
                }
                None => write!(s, "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").unwrap(),
            }
        }
    });
    base
}

fn fixture() -> Vec<u8> {
    std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/neutron-sample.bin")).unwrap()
}

fn gzip(b: &[u8]) -> Vec<u8> {
    let mut e = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    e.write_all(b).unwrap();
    e.finish().unwrap()
}

fn sha(b: &[u8]) -> String {
    Sha256::digest(b).iter().map(|x| format!("{x:02x}")).collect()
}

fn files_in(dir: &Path) -> Vec<String> {
    let mut v: Vec<String> =
        std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().file_name().into_string().unwrap()).collect();
    v.sort();
    v
}

#[test]
fn downloads_checks_and_unpacks_the_file() {
    let gz = gzip(&fixture());
    let sum = format!("{}  neutron.bin.gz\n", sha(&gz));
    let base = serve(vec![("/neutron.bin.gz.sha256", sum.into_bytes()), ("/neutron.bin.gz", gz.clone())]);
    let dir = tempfile::tempdir().unwrap();
    assert_eq!(installed(dir.path()).unwrap(), None);
    assert!(newer_available(dir.path(), &base).unwrap(), "no file yet: the published one is newer");

    let mut seen = vec![];
    let h = download(dir.path(), &base, |done, total| seen.push((done, total))).unwrap();
    assert_eq!(h.stars, 35);
    assert_eq!(seen.last(), Some(&(gz.len() as u64, Some(gz.len() as u64))));
    assert_eq!(std::fs::read(dir.path().join(FILE_NAME)).unwrap(), fixture());
    assert_eq!(installed(dir.path()).unwrap(), Some(h));
    assert_eq!(files_in(dir.path()), ["neutron.bin", "neutron.bin.sha256"]);
    assert!(!newer_available(dir.path(), &base).unwrap(), "the file just downloaded is the latest");

    // A rebuilt file is published: its checksum differs.
    let other = serve(vec![("/neutron.bin.gz.sha256", format!("{}\n", "ab".repeat(32)).into_bytes())]);
    assert!(newer_available(dir.path(), &other).unwrap());
}

#[test]
fn a_wrong_checksum_leaves_no_file() {
    let gz = gzip(&fixture());
    let base = serve(vec![
        ("/neutron.bin.gz.sha256", format!("{}  neutron.bin.gz\n", "0".repeat(64)).into_bytes()),
        ("/neutron.bin.gz", gz),
    ]);
    let dir = tempfile::tempdir().unwrap();
    let err = download(dir.path(), &base, |_, _| {}).unwrap_err().to_string();
    assert!(err.contains("checksum doesn't match"), "{err}");
    assert!(files_in(dir.path()).is_empty(), "left {:?}", files_in(dir.path()));
}

#[test]
fn a_failed_download_keeps_the_old_file() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join(FILE_NAME), fixture()).unwrap();

    // Damaged in transit: the gzip doesn't unpack.
    let mut gz = gzip(&fixture());
    let mid = gz.len() / 2;
    gz[mid..mid + 16].fill(0xff);
    let base = serve(vec![("/neutron.bin.gz.sha256", format!("{}\n", sha(&gz)).into_bytes()), ("/neutron.bin.gz", gz)]);
    assert!(download(dir.path(), &base, |_, _| {}).is_err());
    assert_eq!(files_in(dir.path()), ["neutron.bin"]);
    assert_eq!(std::fs::read(dir.path().join(FILE_NAME)).unwrap(), fixture());

    // Not published.
    let none = serve(vec![]);
    let err = download(dir.path(), &none, |_, _| {}).unwrap_err().to_string();
    assert!(err.contains("HTTP 404"), "{err}");
    assert_eq!(files_in(dir.path()), ["neutron.bin"]);
}
