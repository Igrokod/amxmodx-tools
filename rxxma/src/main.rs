#![cfg_attr(feature = "strict", deny(warnings))]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate failure;

pub mod ast;
pub mod util;

use failure::format_err;
use log::trace;

use std::convert::TryFrom;
use std::path::Path;

use clap::{App, Arg};
use failure::Error;

use amxmodx_utils::amx::File as AmxFile;
use amxmodx_utils::amxx::File as AmxxFile;
use ast::Decompiler;
use ast::TreeElement;

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

fn decompile(file_path: &str) -> Result<String, Error> {
    let amxx_file = AmxxFile::try_from(Path::new(file_path))?;
    let mut amx_file: Option<AmxFile> = None;

    for section_result in amxx_file.sections() {
        let section = section_result?;

        if section.metadata().cellsize != 4 {
            continue;
        }

        let section_body = section.unpack_body()?;
        amx_file = Some(AmxFile::try_from(&section_body[..])?);
    }

    let amx_file = match amx_file {
        Some(amx_file) => amx_file,
        None => return Err(format_err!("No 32 bit section found in file")),
    };

    let mut decompiler = Decompiler::from(amx_file);
    decompiler.opcodes_into_functions();
    decompiler.decompile_opcodes_by_templates().unwrap();
    let ast_plugin = decompiler.into_tree();
    let ast_string = ast_plugin.to_string(0).map_err(|e| format_err!("{}", e))?;

    Ok(ast_string)
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

    let file_path = matches
        .value_of("file")
        .expect("File path is always required in clap");

    let source = {
        match decompile(file_path) {
            Ok(s) => s,
            Err(e) => die!("{}", e),
        }
    };

    println!("{}", source);
}
