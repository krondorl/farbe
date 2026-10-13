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

use crate::named_colors::COLORS;
use crate::search::{
    search_hex3, search_hex4, search_hex6, search_hex8, search_named_colors, search_rgb,
    search_rgba,
};
use crate::utility::{is_stylesheet, print_info};
use clap::{CommandFactory, Parser};

const FILE_SIZE_LIMIT: u64 = 16 * 1024 * 1024; // 16 MB

const VERSION_INFO: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " — ",
    env!("CARGO_PKG_DESCRIPTION")
);

#[derive(Parser)]
#[command(version = VERSION_INFO)]
#[command(
    long_about = "Show color stats of hex, named colors, rgb and rgba values",
    after_help = "Examples:\n  farbe style.css\n  farbe colors.scss\n  farbe --help"
)]
struct Cli {
    /// Stylesheet file path
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let cmd = Cli::command();

    print_info(cmd);

    if is_stylesheet(&args.path) {
        if args.path.exists() {
            let metadata = fs::metadata(&args.path)?;
            let file_size = metadata.len();
            if file_size < FILE_SIZE_LIMIT {
                println!("File size: {} bytes", file_size);
                let mut contents = String::with_capacity(file_size as usize);

                println!("Reading file: {:?} ({} bytes)", args.path, file_size);
                fs::File::open(&args.path)?.read_to_string(&mut contents)?;

                println!("File read successfully. Processing...");
                let mut stats = Vec::new();

                search_named_colors(&contents, &COLORS, &mut stats);
                search_hex3(&contents, &mut stats);
                search_hex4(&contents, &mut stats);
                search_hex6(&contents, &mut stats);
                search_hex8(&contents, &mut stats);
                search_rgb(&contents, &mut stats);
                search_rgba(&contents, &mut stats);

                println!();
                println!("Stats: {:?}", stats);
                println!();

                Ok(())
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "File size exceeds the 16 MB limit.",
                ))
            }
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found.",
            ))
        }
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "File extension is not supported. Please provide a .css or .scss file.",
        ))
    }
}
