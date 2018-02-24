extern crate clap;

use std::io::prelude::*;
use std::fs::File;
use clap::{Arg, App};

// TODO: Custom panic handler
macro_rules! die {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ({
        print!(concat!($fmt, "\n"), $($arg)*);
        std::process::exit(-1);
    });
}

fn main() {
    let matches = App::new("rxxma")
                   .version("0.0.1")
                   .about("Amxmodx plugin reverse utility")
                   .author("Fedcomp")
                   .arg(Arg::with_name("file")
                        .value_name("FILE")
                        .help("amxmodx file to analyze")
                        .required(true)
                        .takes_value(true))
                   .get_matches();

    let file_path = matches.value_of("file").unwrap();
    let mut file = match File::open(file_path) {
        Ok(bin) => bin,
        Err(e) => die!("Cannot open file: {}", e)
    };

    let mut contents: Vec<u8> = vec![];
    match file.read_to_end(&mut contents) {
        Ok(_) => (),
        Err(e) => die!("Cannot read file: {}", e)
    };
}
