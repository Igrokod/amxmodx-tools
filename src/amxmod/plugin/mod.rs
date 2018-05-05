mod try_from_vec_u8;

use super::{Native, Opcode, Public};
use super::super::util::ReadByteString;
use byteorder::{LittleEndian, ReadBytesExt};
use failure::{Error, ResultExt};
use std::ffi::CString;
use std::io::Cursor;
use std::str;

pub enum ConstantParam {
    Cell(u32),
    String(CString),
}

#[derive(Debug, PartialEq)]
pub struct Plugin {
    flags: u16,
    defsize: u16,
    cod: usize,
    dat: usize,
    hea: usize,
    stp: usize,
    cip: usize,
    publics: usize,
    natives: usize,
    libraries: usize,
    pubvars: usize,
    tags: usize,
    nametable: usize,
    pub bin: Vec<u8>,
}

const AMXMOD_MAGIC: u16 = 0xF1E0;
const FILE_VERSION: u8 = 8;
const AMX_VERSION: u8 = 8;
pub const CELLSIZE: usize = 4;

impl Plugin {
    fn cod_slice(&self) -> Result<&[u8], Error> {
        self.bin.get(self.cod..self.dat).ok_or(format_err!(
            "cod slice mismatch"
        ))
    }

    fn dat_slice(&self) -> Result<&[u8], Error> {
        self.bin.get(self.dat..self.hea).ok_or(format_err!(
            "dat slice mismatch"
        ))
    }

    fn publics_slice(&self) -> Result<&[u8], Error> {
        self.bin.get(self.publics..self.natives).ok_or(format_err!(
            "publics slice mismatch"
        ))
    }

    fn natives_slice(&self) -> Result<&[u8], Error> {
        self.bin.get(self.natives..self.libraries).ok_or(
            format_err!(
                "natives slice mismatch"
            ),
        )
    }

    pub fn opcodes(&self) -> Result<Vec<Opcode>, Error> {
        let mut cod_reader = Cursor::new(self.cod_slice()?);

        // Skip first two opcodes for some reason
        cod_reader.read_u32::<LittleEndian>().context(
            "EOF on first opcode skip",
        )?;
        cod_reader.read_u32::<LittleEndian>().context(
            "EOF on second opcode skip",
        )?;

        let mut opcodes: Vec<Opcode> = Vec::new();
        loop {
            match Opcode::read_from(&mut cod_reader) {
                // TODO: Test all cases
                Ok(Some(o)) => opcodes.extend(o),
                Ok(None) => break,
                Err(e) => return Err(format_err!("{}", e)),
            }
        }

        Ok(opcodes)
    }

    pub fn natives(&self) -> Result<Vec<Native>, Error> {
        let slice = self.natives_slice().unwrap();
        let result = slice.chunks(8) // Take natives by native struct
           .map(|n_struct| {
               // FIXME: Error handling
               let mut address = &n_struct[0..4];
               let address = address.read_u32::<LittleEndian>().unwrap() as usize;
               let mut name_offset = &n_struct[4..8];
               let name_offset = name_offset.read_u32::<LittleEndian>().unwrap() as usize;
               let name = self.bin[name_offset..].read_string_zero().unwrap();

               Native {
                   name: name,
                   address: address,
               }
           }).collect();
        Ok(result)
    }

    pub fn publics(&self) -> Result<Vec<Public>, Error> {
        let slice = self.publics_slice()?;
        let result = slice.chunks(8) // Take natives by native struct
           .map(|n_struct| {
               // FIXME: Error handling
               let mut address = &n_struct[0..4];
               let address = address.read_u32::<LittleEndian>().unwrap() as usize;
               let mut name_offset = &n_struct[4..8];
               let name_offset = name_offset.read_u32::<LittleEndian>().unwrap() as usize;
               let name = self.bin[name_offset..].read_string_zero().unwrap();

               Public {
                   name: name,
                   address: address,
               }
           }).collect();
        Ok(result)
    }

    pub fn read_constant_auto_type(&self, addr: usize) -> Result<ConstantParam, &str> {
        if !(addr <= (self.hea - self.dat)) {
            return Ok(ConstantParam::Cell(addr as u32));
        }

        // TODO: Error handling
        let byte_slice: Vec<u8> = self.dat_slice().unwrap()[addr..]
            .chunks(CELLSIZE)
            .map(|x| x[0])
            .take_while(|&x| x != 0)
            .collect();

        let string = CString::new(byte_slice).unwrap();
        Ok(ConstantParam::String(string))
    }
}

#[cfg(test)]
mod tests {
    use super::ConstantParam;
    use super::Native;
    use super::Plugin;
    use super::Public;
    use std::ffi::CString;
    use util::tests::load_fixture;
    use util::try_from::TryFrom;

    // TODO: Support amx extraction in programm itself
    // fn extract_section_to_file(amxmodx_bin: &[u8], section_number: usize) {
    //     use super::super::super::amxmodx::File as AmxxFile;
    //     use std::fs::File;
    //     use std::io::prelude::*;
    //
    //     let amxmodx_plugin = AmxxFile::from(&amxmodx_bin).unwrap();
    //     let sections = amxmodx_plugin.sections().unwrap();
    //     let amxmod_plugin = sections[section_number]
    //         .unpack_section(&amxmodx_bin)
    //         .unwrap();
    //
    //     let mut file = File::create("unpacked.amx").unwrap();
    //     file.write_all(&amxmod_plugin.bin).unwrap();
    // }

    #[test]
    fn it_read_opcodes() {
        let amxmod_bin = load_fixture("simple.amx183");
        let amxmod_plugin = Plugin::try_from(amxmod_bin).unwrap();
        amxmod_plugin.opcodes().unwrap();
    }

    #[test]
    fn it_read_natives() {
        let amxmod_bin = load_fixture("two_natives.amx183");
        let amxmod_plugin = Plugin::try_from(amxmod_bin).unwrap();
        let natives = amxmod_plugin.natives().unwrap();
        let expected_natives = [
            Native {
                name: CString::new("native_one").unwrap(),
                address: 0,
            },
            Native {
                name: CString::new("native_two").unwrap(),
                address: 0,
            },
        ];

        assert_eq!(natives, expected_natives);
    }

    #[test]
    fn it_read_publics() {
        let amxmod_bin = load_fixture("two_natives.amx183");
        let amxmod_plugin = Plugin::try_from(amxmod_bin).unwrap();
        let publics = amxmod_plugin.publics().unwrap();
        let expected_publics = [
            Public {
                name: CString::new("func").unwrap(),
                address: 8,
            },
        ];

        assert_eq!(publics, expected_publics);
    }

    #[test]
    fn it_read_string_by_addr() {
        let amxmod_bin = load_fixture("cell_constants.amx183");
        let amx_plugin = Plugin::try_from(amxmod_bin).unwrap();
        let string = match amx_plugin.read_constant_auto_type(0).unwrap() {
            ConstantParam::String(s) => s,
            _ => panic!("invalid result"),
        };

        assert_eq!("simple plugin", string.into_string().unwrap());
    }

    #[test]
    fn it_read_cell_by_addr() {
        let amxmod_bin = load_fixture("cell_constants.amx183");
        let amx_plugin = Plugin::try_from(amxmod_bin).unwrap();
        let number = match amx_plugin.read_constant_auto_type(99999999).unwrap() {
            ConstantParam::Cell(n) => n,
            _ => panic!("invalid result"),
        };

        assert_eq!(99999999, number);
    }
}
