//! # ufofmt
//!
//! A highly opinionated UFO source formatter.  Built with Norad.
//!
//! The `ufofmt` executable formats UFO source files with the specification
//! defined in the Rust [norad library](https://github.com/linebender/norad).
//!
//! ## Resources
//! - Source repository: [https://github.com/source-foundry/ufofmt](https://github.com/source-foundry/ufofmt)
//! - License: [Apache License 2.0](https://github.com/source-foundry/ufofmt/blob/main/LICENSE)
//! - [Issue tracker](https://github.com/source-foundry/ufofmt/issues)
//! - [Changelog](https://github.com/source-foundry/ufofmt/blob/main/CHANGELOG.md)
//! - [Developer documentation](https://github.com/source-foundry/ufofmt/blob/main/README.md)
//!
//! ## Installation
//! Install the `ufofmt` executable with:
//!
//! ```
//! $ cargo install ufofmt
//! ```
//!
//! Upgrade a previously installed `ufofmt` executable to the latest release with:
//!
//! ```
//! $ cargo install --force ufofmt
//!```
//!
//! ## Usage
//! The command line syntax is:
//!
//! ```
//! $ ufofmt [OPTIONS] [UFO PATH 1] ... [UFO PATH n]
//! ```
//!
//! Enter `ufofmt --help` to view help documentation with all available command line options.

use std::path::{Path, PathBuf};
use std::time::Instant;

use rayon::prelude::*;
use structopt::StructOpt;

// ufofmt library modules
pub mod lib;

use crate::lib::errors;
use crate::lib::formatters;

#[derive(StructOpt, Debug)]
#[structopt(about = "A highly opinionated UFO source formatter.  Built with Norad.")]
struct Opt {
    /// Display timing data
    #[structopt(short = "t", long = "time", help = "Display timing data")]
    time: bool,

    /// UFO source file paths
    #[structopt(help = "UFO source path(s)")]
    ufopaths: Vec<PathBuf>,
}

fn main() {
    let argv = Opt::from_args();

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Source formatting execution
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let now = Instant::now();
    let results: Vec<errors::Result<&Path>> =
        argv.ufopaths.par_iter().map(|ufopath| formatters::format_ufo(ufopath)).collect();
    let duration = now.elapsed().as_millis();

    for result in &results {
        match result {
            Ok(path) => {
                println!("{} {}", *errors::OK_INDICATOR, path.display());
            }
            Err(err) => {
                errors::print_error(err);
            }
        }
    }

    if argv.time {
        println!("Total duration: {} ms", duration);
    }

    // An error was identified if any process returned a u8 value of 1
    // If there was no error, the sum = 0
    if results.iter().any(|v| v.is_err()) {
        std::process::exit(1);
    }
}
