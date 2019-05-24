pub mod opcode;
pub mod opcode_type;
pub mod opcodes_iterator;
pub mod parser;

use failure::Fail;
use opcodes_iterator::OpcodesIterator;
use std::mem::size_of;

pub type Cell = u32;

bitflags! {
    pub struct Flags: u16 {
        // const AMX_FLAG_CHAR16 = 0x01; // no longer used
        const DEBUG = 0x02; // symbolic info available
        const COMPACT = 0x04; // compact encoding
        const BYTEOPC = 0x08; // opcode is a byte (not a cell)
        const NOCHECKS = 0x10; // no array bounds checking; no STMT opcode
        const NTVREG = 0x1000; // all native functions are registered
        const JITC = 0x2000; // abstract machine is JIT compiled
        const BROWSE = 0x4000; // busy browsing
        const RELOC = 0x8000; // jump/call addresses relocated
    }
}

#[derive(Debug, Fail)]
pub enum ParseError {
    #[fail(display = "Cod section got invalid offset")]
    CodSectionMismatch,
}

#[derive(Debug, PartialEq)]
pub struct File {
    bin: Vec<u8>,
    flags: Flags,
    defsize: u16,
    cod: u32,
    dat: u32,
    hea: u32,
    stp: u32,
    cip: u32,
    publics: u32,
    natives: u32,
    libraries: u32,
    pubvars: u32,
    tags: u32,
    nametable: u32,
}

impl File {
    // TODO: Test
    // TODO: DEFSIZE check?
    pub fn cod_slice(&self) -> Result<&[u8], ParseError> {
        self.bin
            .get((self.cod as usize)..(self.dat as usize))
            .ok_or_else(|| ParseError::CodSectionMismatch)
    }

    pub fn opcodes(&self) -> Result<OpcodesIterator, ParseError> {
        Ok(OpcodesIterator::new(self.cod_slice()?))
    }
}

#[cfg(test)]
mod tests {
    use super::{File as AmxFile, Flags};
    use std::convert::TryFrom;
    use std::fs::File as IoFile;
    use std::io::{self, Read};

    fn _read_file(path: &str) -> io::Result<Vec<u8>> {
        let mut file = IoFile::open(path)?;
        let mut plugin = vec![];
        file.read_to_end(&mut plugin)?;

        Ok(plugin)
    }

    fn read_file(path: &str) -> Vec<u8> {
        _read_file(path).expect(&format!("Could not read {} file", path))
    }

    #[test]
    fn it_returns_opcode_iterator() {
        let unpacked_bin = read_file("test/fixtures/amxx/simple.cellsize4.amx183");
        let file = AmxFile::try_from(&unpacked_bin[..]).expect("Plugin should be correctly parsed");
        let opcodes_iterator = file
            .opcodes()
            .expect("Should correctly parse cod and return iterator");

        // TODO: Test cod parsing correctness
    }
}
