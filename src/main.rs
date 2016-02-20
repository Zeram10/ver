extern crate getopts;
extern crate recital; 

use getopts::{Matches, Options};
use recital::prelude::*;
use std::{env, io, process};
use std::io::Write;

/// The current version of this program.
const VERSION: &'static str = "0.1.0";

/// Processes the command line arguments.
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut rules = Options::new();

    rules.optflag("1", "major",
                  "Increment the major version number.");

    rules.optflag("2", "minor",
                  "Increment the minor version number.");

    rules.optflag("3", "patch",
                  "Increment the patch version number.");

    rules.optflag("e", "equal-to",
                  "Confirm VERSION is equal to VERSION.");

    rules.optflag("g", "greater-than",
                  "Confirm VERSION is greater than VERSION.");

    rules.optflag("h", "help",
                  "Displays this help mssage.");

    rules.optflag("l", "less-than",
                  "Confirm VERSION is less than VERSION.");

    rules.optflag("v", "version",
                  "Displays the version of this program.");

    let options = match rules.parse(&args[1..]) {
        Ok(result) => result,
        Err(error) => exit(1, &error.to_string()),
    };

    if options.opt_present("h") {
        usage(&args[0], &rules);
    }

    if options.opt_present("v") {
        println!("{} v{}", &args[0], &VERSION);

        process::exit(0);
    }

    match options.free.len() {
        0 => usage(&args[0], &rules),
        1 => modify(&options),
        2 => compare(&options),
        _ => exit(1, "Too many arguments provided."),
    }
}

/// Compares the given version numbers.
fn compare(options: &Matches) {
    let left = parse(&options.free[0]);
    let right = parse(&options.free[1]);

    if options.opt_present("g") && options.opt_present("l") {
        exit(1, "Cannot combine --greater-than and --less-than.");
    }

    if options.opt_present("e") {
        if left == right {
            return;
        }
    }

    if options.opt_present("g") {
        if left > right {
            return;
        }
    }

    if options.opt_present("l") {
        if left < right {
            return;
        }
    }

    process::exit(1);
}

/// Exits the program early with a message.
fn exit(code: i32, message: &str) -> ! {
    let _ = writeln!(io::stderr(), "{}", message);

    process::exit(code);
}

/// Modifies the given version number.
fn modify(options: &Matches) {
    let mut version = parse(&options.free[0]);

    if options.opt_present("1") {
        version.increment_major();
    }

    if options.opt_present("2") {
        version.increment_minor();
    }

    if options.opt_present("3") {
        version.increment_patch();
    }

    println!("{}", version);
}

/// Parses a version number.
fn parse(version: &str) -> Version {
    match version.parse() {
        Ok(result) => result,
        Err(_) => exit(1, &format!("Invalid version number: {}", version)),
    }
}

/// Displays the usage message.
fn usage(program: &str, options: &Options) {
    let b = format!("Usage: {} [OPTIONS] VERSION [VERSION]", program);
    let _ = writeln!(io::stderr(), "{}", options.usage(&b));

    process::exit(0);
}
