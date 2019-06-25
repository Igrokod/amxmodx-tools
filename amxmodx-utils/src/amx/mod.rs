pub mod opcode;
pub mod opcode_type;
pub mod opcodes_iterator;
pub mod parser;
pub mod function;

use crate::utils::ByteStringExt;
use bytes::Buf;
use failure::Fail;
use function::{Native, Public};
use opcodes_iterator::OpcodesIterator;
use std::io::Cursor;
use getset::Getters;

pub type UCell = u32;
const SUPPORTED_DEFSIZE: usize = 8;

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
    #[fail(display = "Cod section have invalid range")]
    CodSection,
    #[fail(display = "Dat section have invalid range")]
    DatSection,
    #[fail(display = "Publics section have invalid range")]
    PublicsSection,
    #[fail(display = "Some of publics have invalid name offset")]
    InvalidPublicNameOffset,
    #[fail(display = "Natives section have invalid range")]
    NativesSection,
    #[fail(display = "Some of natives have invalid name offset")]
    InvalidNativeNameOffset,
}

#[derive(Debug, PartialEq, Getters)]
#[get = "pub"]
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
            .ok_or(ParseError::CodSection)
    }

    // TODO: Test
    pub fn dat_slice(&self) -> Result<&[u8], ParseError> {
        self.bin
            .get((self.dat as usize)..(self.hea as usize))
            .ok_or(ParseError::DatSection)
    }

    // TODO: Test
    fn publics_slice(&self) -> Result<&[u8], ParseError> {
        self.bin
            .get((self.publics as usize)..(self.natives as usize))
            .ok_or(ParseError::PublicsSection)
    }

    // TODO: Test
    fn natives_slice(&self) -> Result<&[u8], ParseError> {
        self.bin
            .get((self.natives as usize)..(self.libraries as usize))
            .ok_or(ParseError::NativesSection)
    }

    // Iterator is used to keep proper responsibility principle
    // and it gives more flexibility when iterator is returned
    pub fn opcodes(&self) -> Result<OpcodesIterator, ParseError> {
        Ok(OpcodesIterator::new(self.cod_slice()?))
    }

    // TODO: United Natives/Publics iterator for responsibility principle?
    // TODO: Test
    pub fn native_functions(&self) -> Result<Vec<Native>, ParseError> {
        let natives_slice = self.natives_slice()?;
        let natives_count: f32 = natives_slice.len() as f32 / SUPPORTED_DEFSIZE as f32;
        if natives_count != 0f32 {
            return Err(ParseError::NativesSection);
        }

        let mut results: Vec<Native> = vec![];
        let mut reader = Cursor::new(natives_slice);

        for _ in 0..(natives_count as usize) {
            let address = reader.get_u32_le();
            let name_pointer = reader.get_u32_le();
            let name = self
                .bin
                .get((name_pointer as usize)..)
                .and_then(ByteStringExt::read_cstr)
                .ok_or(ParseError::InvalidNativeNameOffset)?;

            let native = Native::new(name.to_string_lossy(), address);
            results.push(native);
        }

        Ok(results)
    }

    // TODO: United Natives/Publics iterator for responsibility principle?
    // TODO: Test
    pub fn public_functions(&self) -> Result<Vec<Public>, ParseError> {
        let publics_slice = self.publics_slice()?;
        let publics_count: f32 = publics_slice.len() as f32 / SUPPORTED_DEFSIZE as f32;
        if publics_count != 0f32 {
            return Err(ParseError::PublicsSection);
        }

        let mut results: Vec<Public> = vec![];
        let mut reader = Cursor::new(publics_slice);

        for _ in 0..(publics_count as usize) {
            let address = reader.get_u32_le();
            let name_pointer = reader.get_u32_le();
            let name = self
                .bin
                .get((name_pointer as usize)..)
                .and_then(ByteStringExt::read_cstr)
                .ok_or(ParseError::InvalidPublicNameOffset)?;

            let public = Public::new(name.to_string_lossy(), address);
            results.push(public);
        }

        Ok(results)
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
