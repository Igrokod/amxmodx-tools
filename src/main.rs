extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rxxma;

use std::io::prelude::*;
use std::fs::File;
use clap::{Arg, App};
use rxxma::amxmodx::File as AmxmodxFile;
use rxxma::ast::Plugin as AstPlugin;
use rxxma::ast::TreeElement;

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
    env_logger::init();

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

    let section_32bit = match sections.into_iter().find(|ref s| s.cellsize == 4) {
        Some(s) => s,
        None => die!("File has no 32 bit sections. 64 bit are not supported"),
    };

    trace!("-----------------------------------------");
    trace!("Reading amxmod plugin from 32 bit section");
    trace!("-----------------------------------------");
    let amxmod_plugin = match section_32bit.unpack_section(&file_contents) {
        Ok(p) => p,
        Err(e) => die!("Amxmod unpack/parse error: {}", e),
    };

    // let opcodes = amxmod_plugin.opcodes().unwrap();
    // for op in opcodes.iter() {
    //     if let Some(ref p) = op.param {
    //         println!("0x{:X} {}  0x{:X}", op.address, op.code, p);
    //     } else {
    //         println!("0x{:X} {}", op.address, op.code);
    //     }
    // }

    // let natives = amxmod_plugin.natives();
    // println!("\n\nNatives list:");
    // for native in natives {
    //     println!("{}", native.name.to_str().unwrap());
    // }
    //
    // let publics = amxmod_plugin.publics();
    // println!("\n\nPublics list:");
    // for public in publics {
    //     println!("{}", public.name.to_str().unwrap());
    // }

    println!("\n\n");
    let mut ast_plugin = match AstPlugin::from(&amxmod_plugin) {
        Ok(p) => p,
        Err(e) => die!("Cannot convert plugin opcodes to AST tree: {}", e),
    };
    ast_plugin.opcodes_into_functions();
    println!("{}", ast_plugin.to_string().unwrap());
}
