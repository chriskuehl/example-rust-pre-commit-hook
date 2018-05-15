extern crate clap;

use clap::{App, Arg};
use std::fs;


fn capitalize(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Ok(contents) => {
            match fs::write(path, contents.to_uppercase()) {
                Ok(_) => Ok(()),
                Err(err) => Err(format!("Error writing `{}`: {}", path, err)),
            }
        },
        Err(err) => Err(format!("Error reading `{}`: {}", path, err)),
    }
}

fn main() {
    let matches = App::new("capitalize")
        .version(env!("CARGO_PKG_VERSION"))
        .about("capitalize files in-place")
        .arg(Arg::with_name("path")
             .help("path(s) to files to capitalize")
             .multiple(true)
        )
        .get_matches();

    std::process::exit(
        matches.values_of("path").unwrap().map(|path| {
            match capitalize(path) {
                Ok(_) => 0,
                Err(err) => {
                    eprintln!("{}", err);
                    1
                }
            }
        }).max().unwrap(),
    );
}
