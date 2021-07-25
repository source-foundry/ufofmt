use std::path::{Path, PathBuf};
use std::time::Instant;

use colored::*;

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
    let results: Vec<Result<&Path, errors::UfofmtError>> =
        argv.ufopaths.par_iter().map(|ufopath| formatters::format_ufo(ufopath)).collect();
    let duration = now.elapsed().as_millis();

    let ok_indicator = "[OK]".green().bold();
    for result in &results {
        match result {
            Ok(path) => {
                println!("{} {:?}", &ok_indicator, path);
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
