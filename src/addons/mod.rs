use crate::{ADDON_DIR, ADDON_TRANSPOSE};
use anyhow::Result;
use clap::{ArgGroup, Parser};
use std::{path::PathBuf, process::Command};

#[derive(Parser, Debug)]
#[command(
    name = "NoctaVox",
    version,
    about = "A TUI music player for local files"
)]
#[command(group(
      ArgGroup::new("mode")
          .args(["import_playlist", "export_playlist", "list"]),
  ))]

struct Cli {
    /// Import a playlist from a csv or m3u file
    #[arg(long, short)]
    import_playlist: bool,

    /// Export a playlist to m3u, csv, or json format
    #[arg(long, short)]
    export_playlist: bool,

    /// List playlists in the library
    #[arg(long)]
    list: bool,
}

pub fn parse_args() {
    let cli = Cli::parse();

    let _ = if cli.import_playlist {
        run_addon(ADDON_TRANSPOSE, &["--import"])
    } else if cli.export_playlist {
        run_addon(ADDON_TRANSPOSE, &["--export"])
    } else if cli.list {
        run_addon(ADDON_TRANSPOSE, &["--list"])
    } else {
        return;
    };
}

fn addon_path(name: &str) -> PathBuf {
    let filename = format!("{name}{}", std::env::consts::EXE_SUFFIX);
    ADDON_DIR.join(filename)
}

fn run_addon(name: &str, params: &[&str]) -> Result<i32> {
    let bin = addon_path(name);
    if !bin.exists() {
        eprintln!("Addon `{name}` not found.\nExpected at: {}", bin.display());
        std::process::exit(1)
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(&bin)?.permissions().mode();
        if mode & 0o111 == 0 {
            eprintln!("addon at {} is not executable (chmod +x it)", bin.display());
            std::process::exit(1);
        }
    }

    let status = Command::new(&bin)
        .args(params)
        .status()
        .unwrap_or_else(|e| {
            eprintln!("Error: {e}");
            std::process::exit(1)
        });

    Ok(status.code().unwrap_or(1))
}
