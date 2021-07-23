use std::path::PathBuf;
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
    let mut bad_ufo_path = false;
    for ufo_testpath in &argv.ufopaths {
        match ufo_testpath.exists() {
            true => (),
            false => {
                let error_str = "[ERROR]".red().bold();
                eprintln!("{} {:?} is not a valid UFO directory path", error_str, ufo_testpath);
                bad_ufo_path = true;
            }
        }
    }
    if bad_ufo_path {
        std::process::exit(1);
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Source formatting execution
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let now = Instant::now();
    let results: Vec<u8> = argv.ufopaths.par_iter().map(|ufopath| format_ufo(ufopath)).collect();
    let duration = now.elapsed().as_millis();

    if argv.time {
        println!("Total duration: {} ms", duration);
    }

    // An error was identified if any process returned a u8 value of 1
    // If there was no error, the sum = 0
    if results.par_iter().sum::<u8>() > 0 {
        std::process::exit(1);
    }
}

/// Read/write roundtrip through the norad library. Returns a 1 if an error was encountered
/// and 0 if no error was encountered
fn format_ufo(ufopath: &PathBuf) -> u8 {
    match Font::load(ufopath) {
        Ok(ufo) => match ufo.save(ufopath) {
            Ok(_) => 0,
            Err(e) => {
                let error_str = "[ERROR]".red().bold();
                eprintln!("{} Write error in {:?}: {}", error_str, ufopath, e);
                1
            }
        },
        Err(e) => {
            let error_str = "[ERROR]".red().bold();
            eprintln!("{} Read error in {:?}: {}", error_str, ufopath, e);
            1
        }
    }
}
