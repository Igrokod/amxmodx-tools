extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rxxma;
#[macro_use]
extern crate failure;

use clap::{App, Arg};
use failure::Error;
use rxxma::amxmod::Plugin as AmxPlugin;
use rxxma::amxmodx::File as AmxmodxFile;
use rxxma::ast::Decompiler;
use rxxma::ast::TreeElement;
use rxxma::util::TryFrom;
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

fn str_to_err(e: &str) -> Error {
    format_err!("{}", e)
}

fn read_32bit_section(file_path: PathBuf) -> Result<AmxPlugin, Error> {
    let amxmodx_file = AmxmodxFile::try_from(file_path)?;
    let sections = amxmodx_file.sections()?;

    let section_32bit = sections
        .into_iter()
        .find(|ref s| s.cellsize == 4)
        .ok_or("File has no 32 bit sections. 64 bit are not supported")
        .map_err(str_to_err)?;

    trace!("-------------------------------------------");
    trace!(" Reading amxmod plugin from 32 bit section ");
    trace!("-------------------------------------------");
    section_32bit.unpack_section()
}

fn decompile(file_path: PathBuf) -> Result<String, Error> {
    let amxmod_plugin = read_32bit_section(file_path)?;

    let mut decompiler = Decompiler::from(amxmod_plugin);
    decompiler.opcodes_into_functions();
    decompiler.decompile_opcodes_by_templates().unwrap();
    let ast_plugin = decompiler.into_tree();

    Ok(ast_plugin.to_string(0).map_err(str_to_err)?)
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
