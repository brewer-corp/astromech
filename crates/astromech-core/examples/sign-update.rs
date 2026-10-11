//! Sign `latest.json` (or print a new keypair).
//!
//!   cargo run -p astromech-core --example sign-update -- --gen
//!   cargo run -p astromech-core --example sign-update -- --key $ASTROMECH_UPDATE_KEY latest.json

use anyhow::{bail, Result};
use astromech_core::update;
use clap::Parser;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    /// Print a new secret/public pair (hex) and exit
    #[arg(long)]
    gen: bool,
    /// 32-byte secret seed as hex (`ASTROMECH_UPDATE_KEY`)
    #[arg(long)]
    key: Option<String>,
    /// `latest.json` to sign; writes `<file>.sig`
    file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    if args.gen {
        let mut seed = [0u8; 32];
        std::fs::File::open("/dev/urandom")?.read_exact(&mut seed)?;
        let hex = update::hex(&seed);
        println!("ASTROMECH_UPDATE_KEY={hex}");
        println!("public={}", update::verifying_hex_from_secret(&hex)?);
        return Ok(());
    }
    let key = args
        .key
        .or_else(|| std::env::var("ASTROMECH_UPDATE_KEY").ok())
        .ok_or_else(|| anyhow::anyhow!("pass --key or set ASTROMECH_UPDATE_KEY"))?;
    let Some(file) = args.file else { bail!("pass the latest.json path") };
    let json = std::fs::read(&file)?;
    let sig = update::sign_bytes(&key, &json)?;
    let out = file.with_file_name(format!("{}.sig", file.file_name().unwrap_or_default().to_string_lossy()));
    std::fs::write(&out, format!("{sig}\n"))?;
    eprintln!("wrote {} (public {})", out.display(), update::verifying_hex_from_secret(&key)?);
    Ok(())
}
