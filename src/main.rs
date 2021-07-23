use std::path::{Path, PathBuf};
use std::time::Instant;

use colored::*;
use norad::Font;
use rayon::prelude::*;
use structopt::StructOpt;

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

    // ~~~~~~~~~~~~~~~~~~~~~~
    // UFO dir validity check
    // ~~~~~~~~~~~~~~~~~~~~~~
    let invalid_paths: Vec<&PathBuf> =
        argv.ufopaths.iter().filter(|ufopath| !ufopath.exists()).collect();

    if !invalid_paths.is_empty() {
        let error_str = "[ERROR]".red().bold();
        for invalid_ufo_path in invalid_paths.iter() {
            eprintln!("{} {:?} is not a valid UFO directory path", error_str, invalid_ufo_path);
        }
        std::process::exit(1);
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Source formatting execution
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let now = Instant::now();
    let results: Vec<Result<_, norad::Error>> =
        argv.ufopaths.par_iter().map(|ufopath| format_ufo(ufopath)).collect();
    let duration = now.elapsed().as_millis();

    if argv.time {
        println!("Total duration: {} ms", duration);
    }

    // An error was identified if any process returned a u8 value of 1
    // If there was no error, the sum = 0
    if results.iter().any(|v| v.is_err()) {
        std::process::exit(1);
    }
}

/// Read/write roundtrip through the norad library. Returns Result that propagates
/// norad::Error from the norad library
fn format_ufo(ufopath: &Path) -> Result<(), norad::Error> {
    match Font::load(ufopath) {
        Ok(ufo) => match ufo.save(ufopath) {
            Ok(_) => Ok(()),
            Err(e) => {
                let error_str = "[ERROR]".red().bold();
                eprintln!("{} Write error in {:?}: {}", error_str, ufopath, e);
                Err(e)
            }
        },
        Err(e) => {
            let error_str = "[ERROR]".red().bold();
            eprintln!("{} Read error in {:?}: {}", error_str, ufopath, e);
            Err(e)
        }
    }
}
