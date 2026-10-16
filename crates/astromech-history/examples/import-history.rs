//! Import a journal folder into a history database and print how long it took and how big
//! the database is (astromech-aonb, astromech-tqdh):
//! `cargo run --release -p astromech-history --example import-history -- <journal dir> <history.sqlite>`.

use astromech_history::{History, ImportOptions};
use std::path::PathBuf;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args_os().skip(1);
    let (Some(dir), Some(db)) = (args.next().map(PathBuf::from), args.next().map(PathBuf::from)) else {
        eprintln!("usage: import-history <journal dir> <history.sqlite>");
        std::process::exit(2);
    };
    let mut h = History::open(&db)?;
    let t = Instant::now();
    let r = h.import(&dir, &ImportOptions::default())?;
    let secs = t.elapsed().as_secs_f64();
    let c = h.conn();
    c.execute_batch("PRAGMA wal_checkpoint(TRUNCATE)")?;
    let count = |table: &str| c.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |r| r.get::<_, i64>(0));
    let mb = |b: u64| b as f64 / 1_048_576.0;
    let db_size = std::fs::metadata(&db)?.len();
    println!(
        "files {} | lines {} | events {} | malformed {} | shrunk {:?}",
        r.files, r.lines, r.events, r.malformed, r.shrunk
    );
    println!(
        "read {:.1} MB in {secs:.2} s ({:.1} MB/s, {:.0} lines/s)",
        mb(r.bytes),
        mb(r.bytes) / secs,
        r.lines as f64 / secs
    );
    println!("database {:.1} MB ({:.2}x the journals)", mb(db_size), db_size as f64 / r.bytes.max(1) as f64);
    println!(
        "ledgers: visits {} | earnings {} | ships {} | scans {}",
        count("visits")?,
        count("earnings")?,
        count("ships")?,
        count("scans")?
    );
    Ok(())
}
