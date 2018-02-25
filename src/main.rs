extern crate rxxma;
extern crate clap;

use std::io::prelude::*;
use std::fs::File;
use clap::{Arg, App};
use rxxma::amxmodx::File as AmxmodxFile;

// TODO: Custom panic handler
macro_rules! die {
    ($fmt:expr) => ({
        eprintln!($fmt);
        std::process::exit(-1);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        eprintln!($fmt, $($arg)*);
        std::process::exit(-1);
    });
}

fn main() {
    let matches = App::new("rxxma")
        .version("0.0.1")
        .about("Amxmodx plugin reverse utility")
        .author("Fedcomp")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("amxmodx file to analyze")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let file_path = matches.value_of("file").unwrap();
    let mut file = match File::open(file_path) {
        Ok(bin) => bin,
        Err(e) => die!("Cannot open file: {}", e),
    };

    let mut file_contents: Vec<u8> = Vec::new();
    match file.read_to_end(&mut file_contents) {
        Ok(_) => (),
        Err(e) => die!("Cannot read file: {}", e),
    };

    let amxmodx_file = match AmxmodxFile::from(&file_contents) {
        Ok(a) => a,
        Err(e) => die!("File parsing error: {}", e),
    };

    let sections = match amxmodx_file.sections() {
        Ok(s) => s,
        Err(e) => die!("Sections read error: {}", e),
    };

    println!("AmXModX file sections: {:?}", sections);

    let section_32bit = match sections.into_iter().find(|ref s| s.cellsize == 4) {
        Some(s) => s,
        None => die!("File has no 32 bit sections. 64 bit are not supported"),
    };

    let amxmod_plugin = match section_32bit.unpack_section(&file_contents) {
        Ok(p) => p,
        Err(e) => die!("Amxmod unpack/parse error: {}", e),
    };

    println!("{:?}", amxmod_plugin.opcodes());
}
