// Copyright 2026-present Adam Burucs. MIT license.

mod model;
mod named_colors;
mod search;
mod utility;

#[cfg(test)]
mod search_tests;

use std::fs::{self};
use std::io::Read;
use std::println;

use crate::utility::print_info;
use clap::{CommandFactory, Parser};

const VERSION_INFO: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " — ",
    env!("CARGO_PKG_DESCRIPTION"),
    "\nDisplay color statistics of CSS and SCSS files"
);

#[derive(Parser)]
#[command(version = VERSION_INFO)]
#[command(
    long_about = "Show color stats of hex, named colors, rgb and rgba values",
    after_help = "Examples:\n  farbe style.css"
)]
struct Cli {
    /// Stylesheet file path
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let cmd = Cli::command();

    print_info(cmd);

    if args.path.exists() {
        let metadata = fs::metadata(&args.path)?;
        let mut contents = String::with_capacity(metadata.len() as usize);

        println!("Reading file: {:?} ({} bytes)", args.path, metadata.len());
        fs::File::open(&args.path)?.read_to_string(&mut contents)?;
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found",
        ))
    }
}
