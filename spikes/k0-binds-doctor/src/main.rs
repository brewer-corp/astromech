//! k0-binds-doctor: read-only inspector for Elite Dangerous bindings and controllers.
//! It never writes, renames or moves anything in the game folders; its only output is a report.

mod analyze;
mod binds;
mod devices;
mod game;
mod report;

use anyhow::{bail, Context, Result};
use clap::Parser;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about = "Read-only check of Elite Dangerous .binds files against your connected controllers")]
struct Args {
    /// Bindings folder (default: %LOCALAPPDATA%\Frontier Developments\Elite Dangerous\Options\Bindings)
    #[arg(long)]
    bindings_dir: Option<PathBuf>,
    /// Game install folder (Steam "Elite Dangerous" folder, Frontier/Epic install, or a Products\<product> folder)
    #[arg(long)]
    game_dir: Option<PathBuf>,
    /// Extra folder of game client logs to scan
    #[arg(long)]
    logs_dir: Option<PathBuf>,
    /// Use devices from a JSON file instead of live enumeration (testing)
    #[arg(long)]
    fake_devices: Option<PathBuf>,
    /// Where to write the report (default: next to the exe)
    #[arg(long)]
    out_dir: Option<PathBuf>,
    /// Don't wait for Enter before exiting
    #[arg(long)]
    no_pause: bool,
}

fn is_within(child: &Path, parent: &Path) -> bool {
    let norm = |p: &Path| std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf()).to_string_lossy().to_lowercase().replace('/', "\\").trim_end_matches('\\').to_string();
    let (c, p) = (norm(child), norm(parent));
    c == p || c.starts_with(&(p + "\\"))
}

fn main() {
    let double_clicked = std::env::args().len() == 1 && std::io::stdin().is_terminal();
    let args = Args::parse();
    let code = match run(&args) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("\nError: {e:#}");
            1
        }
    };
    if double_clicked && !args.no_pause {
        eprintln!("\nPress Enter to close.");
        let mut s = String::new();
        let _ = std::io::stdin().read_line(&mut s);
    }
    std::process::exit(code);
}

fn run(args: &Args) -> Result<()> {
    let bindings = match &args.bindings_dir {
        Some(p) => p.clone(),
        None => game::default_bindings_dir().context("LOCALAPPDATA is not set; pass --bindings-dir")?,
    };
    let mut notes = vec![];
    let installs = game::locate_installs(args.game_dir.as_deref(), &mut notes);
    let inventory = match &args.fake_devices {
        Some(p) => devices::load_fake(p).with_context(|| format!("reading --fake-devices {}", p.display()))?,
        None => devices::enumerate_live(),
    };
    let out_dir = match &args.out_dir {
        Some(p) => p.clone(),
        None => std::env::current_exe()?.parent().context("exe has no folder")?.to_path_buf(),
    };
    // Safety: never write into the game's folders.
    let mut protected = vec![bindings.clone()];
    if let Some(p) = bindings.parent().and_then(|p| p.parent()) {
        // ...\Frontier Developments\Elite Dangerous (Options, Logs), only when the layout is the game's own.
        if p.file_name().map(|n| n.to_string_lossy().eq_ignore_ascii_case("Elite Dangerous")).unwrap_or(false) {
            protected.push(p.to_path_buf());
        }
    }
    protected.extend(installs.iter().map(|g| g.product_dir.clone()));
    if let Some(g) = &args.game_dir {
        protected.push(g.clone());
    }
    for p in &protected {
        if is_within(&out_dir, p) {
            bail!("refusing to write the report inside a game folder ({}). Move the exe elsewhere (e.g. Desktop or Downloads) or pass --out-dir.", p.display());
        }
    }
    let mut extra = vec![];
    if let Some(d) = game::default_logs_dir() {
        extra.push(d);
    }
    if let Some(d) = &args.logs_dir {
        extra.push(d.clone());
    }
    let rep = analyze::run(analyze::Inputs { bindings_dir: &bindings, installs, inventory, extra_log_dirs: extra, notes });

    let red = report::Redactor::from_env();
    let stamp = chrono::Local::now().format("%Y%m%d-%H%M%S");
    let base = out_dir.join(format!("binds-doctor-{stamp}"));
    let mut json = serde_json::to_value(&rep)?;
    red.apply_json(&mut json);
    let json_path = base.with_extension("json");
    let html_path = base.with_extension("html");
    std::fs::write(&json_path, serde_json::to_string_pretty(&json)?).with_context(|| format!("writing {}", json_path.display()))?;
    std::fs::write(&html_path, red.apply(&report::html(&rep))).with_context(|| format!("writing {}", html_path.display()))?;
    print!("{}", red.apply(&report::console(&rep)));
    println!("\nReport written (nothing in your game folders was changed):\n  {}\n  {}", red.apply(&html_path.display().to_string()), red.apply(&json_path.display().to_string()));
    Ok(())
}
