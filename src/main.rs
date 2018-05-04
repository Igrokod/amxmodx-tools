extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rxxma;

use clap::{App, Arg};
use rxxma::amxmod::Plugin as AmxPlugin;
use rxxma::amxmodx::File as AmxmodxFile;
use rxxma::ast::Decompiler;
use rxxma::ast::TreeElement;
use rxxma::util::TryFrom;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

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

fn io_to_str(e: std::io::Error) -> String {
    e.to_string()
}

fn read_from_file(file_path: PathBuf) -> Result<Vec<u8>, String> {
    File::open(&file_path)
        .and_then(|mut f| {
            let mut file_contents: Vec<u8> = Vec::new();
            f.read_to_end(&mut file_contents)?;
            Ok(file_contents)
        })
        .map_err(io_to_str)
}

fn read_32bit_section(file_path: PathBuf) -> Result<AmxPlugin, String> {
    let file_contents = read_from_file(file_path)?;

    let amxmodx_file = AmxmodxFile::try_from(file_contents.clone())?;
    let sections = amxmodx_file.sections()?;

    let section_32bit = sections.into_iter().find(|ref s| s.cellsize == 4).ok_or(
        "File has no 32 bit sections. 64 bit are not supported",
    )?;

    trace!("-------------------------------------------");
    trace!(" Reading amxmod plugin from 32 bit section ");
    trace!("-------------------------------------------");
    section_32bit.unpack_section().map_err(|e| e.to_string())
}

fn decompile(file_path: PathBuf) -> Result<String, String> {
    let amxmod_plugin = read_32bit_section(file_path)?;

    let mut decompiler = Decompiler::from(amxmod_plugin);
    decompiler.opcodes_into_functions();
    decompiler.decompile_opcodes_by_templates().unwrap();
    let ast_plugin = decompiler.into_tree();

    Ok(ast_plugin.to_string(0)?)
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
    let file_path_buf = PathBuf::from(file_path);

    let source = {
        match decompile(file_path_buf) {
            Ok(s) => s,
            Err(e) => die!("{}", e),
        }
    };

    println!("{}", source);
}
