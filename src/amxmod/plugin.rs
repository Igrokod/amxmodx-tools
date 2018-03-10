use std::io::Cursor;
use std::io::prelude::*;
use byteorder::{ReadBytesExt, LittleEndian};
use std::str;
use super::super::util::ReadByteString;

use super::Opcode;
use super::Native;

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

impl Plugin {
    const AMXMOD_MAGIC: u16 = 0xF1E0;
    const FILE_VERSION: u8 = 8;
    const AMX_VERSION: u8 = 8;

    pub fn from<'a>(bin: &[u8]) -> Result<Plugin, &'a str> {
        let mut reader = Cursor::new(bin);

        let size = match reader.read_u32::<LittleEndian>() {
            Ok(s) => s,
            Err(_) => return Err("EOF on amx size"),
        };
        trace!("size:\t{}", size);

        // TODO: Test incorrect
        match reader.read_u16::<LittleEndian>() {
            Ok(m) => {
                trace!("magic:\t0x{:X}", m);
                if m != Plugin::AMXMOD_MAGIC {
                    return Err("Invalid Amxmod magic");
                }
                m
            }
            Err(_) => return Err("EOF on amx magic"),
        };

        // TODO: Test incorrect
        match reader.read_u8() {
            Ok(v) => {
                trace!("file version {}", v);
                if v != Plugin::FILE_VERSION {
                    return Err("Invalid file version");
                }
                v
            }
            Err(_) => return Err("EOF on amx file version"),
        };

        // TODO: Test incorrect
        match reader.read_u8() {
            Ok(v) => {
                trace!("amx version:\t{}", v);
                if v != Plugin::AMX_VERSION {
                    return Err("Invalid amx version");
                }
                v
            }
            Err(_) => return Err("EOF on amx version"),
        };

        // TODO: Parse flags
        let flags = match reader.read_u16::<LittleEndian>() {
            Ok(f) => f,
            Err(_) => return Err("EOF on amx flags"),
        };

        let defsize = match reader.read_u16::<LittleEndian>() {
            Ok(ds) => ds,
            Err(_) => return Err("EOF on amx defsize"),
        };
        trace!("defsize:\t{}", defsize);

        let cod = match reader.read_u32::<LittleEndian>() {
            Ok(c) => c,
            Err(_) => return Err("EOF on amx cod"),
        };
        trace!("cod:\t0x{:X}", cod);

        let dat = match reader.read_u32::<LittleEndian>() {
            Ok(d) => d,
            Err(_) => return Err("EOF on amx dat"),
        };
        trace!("dat:\t0x{:X}", dat);

        let hea = match reader.read_u32::<LittleEndian>() {
            Ok(h) => h,
            Err(_) => return Err("EOF on amx hea"),
        };
        trace!("hea:\t0x{:X}", hea);

        let stp = match reader.read_u32::<LittleEndian>() {
            Ok(s) => s,
            Err(_) => return Err("EOF on amx stp"),
        };
        trace!("stp:\t0x{:X}", stp);

        let cip = match reader.read_u32::<LittleEndian>() {
            Ok(c) => c,
            Err(_) => return Err("EOF on amx cip"),
        };
        trace!("cip:\t0x{:X}", cip);

        let publics = match reader.read_u32::<LittleEndian>() {
            Ok(p) => p,
            Err(_) => return Err("EOF on amx publics"),
        };
        trace!("publics:\t0x{:X}", publics);

        let natives = match reader.read_u32::<LittleEndian>() {
            Ok(n) => n,
            Err(_) => return Err("EOF on amx natives"),
        };
        trace!("natives:\t0x{:X}", natives);

        let libraries = match reader.read_u32::<LittleEndian>() {
            Ok(l) => l,
            Err(_) => return Err("EOF on amx libraries"),
        };
        trace!("libraries:\t0x{:X}", libraries);

        let pubvars = match reader.read_u32::<LittleEndian>() {
            Ok(p) => p,
            Err(_) => return Err("EOF on amx pubvars"),
        };
        trace!("pubvars:\t0x{:X}", pubvars);

        let tags = match reader.read_u32::<LittleEndian>() {
            Ok(t) => t,
            Err(_) => return Err("EOF on amx tags"),
        };
        trace!("tags:\t0x{:X}", tags);

        let nametable = match reader.read_u32::<LittleEndian>() {
            Ok(n) => n,
            Err(_) => return Err("EOF on amx nametable"),
        };
        trace!("nametable:\t0x{:X}", nametable);

        Ok(Plugin {
            flags: flags,
            defsize: defsize,
            cod: cod as usize,
            dat: dat as usize,
            hea: hea as usize,
            stp: stp as usize,
            cip: cip as usize,
            publics: publics as usize,
            natives: natives as usize,
            libraries: libraries as usize,
            pubvars: pubvars as usize,
            tags: tags as usize,
            nametable: nametable as usize,
            bin: bin.to_vec(),
        })
    }

    pub fn cod_slice(&self) -> &[u8] {
        // FIXME: Error handling when cod does not match
        // Calculate from start of next segment
        trace!("---- Slicing cod");
        trace!("cod starts at: {}", self.cod);
        trace!("dat starts at: {}", self.dat);
        let cod_size = self.dat - self.cod;
        trace!("cod size: {}", cod_size);
        trace!("bin size: {}", self.bin.len());
        trace!("final range: {}-{}", self.cod, self.cod + cod_size);
        &self.bin[self.cod..(self.cod + cod_size)]
    }

    pub fn opcodes(&self) -> Result<Vec<Opcode>, &str> {
        let mut cod_reader = Cursor::new(self.cod_slice());
        let mut opcodes: Vec<Opcode> = Vec::new();

        // FIXME: Error handling
        // Skip first two opcodes for some reason
        cod_reader.read_u32::<LittleEndian>().unwrap();
        cod_reader.read_u32::<LittleEndian>().unwrap();

        loop {
            match Opcode::read_from(&mut cod_reader) {
                // FIXME: Test all cases
                Ok(Some(o)) => opcodes.extend(o),
                Ok(None) => break,
                Err(e) => return Err(e),
            }
        }

        Ok(opcodes)
    }

    pub fn natives(&self) -> Vec<Native> {
        let slice = &self.bin[self.natives..self.libraries];
        slice.chunks(8) // Take natives by native struct
           .map(|x| &x[4..8] ) // Extrace name_offset from struct
           // TODO: Error handling?
           // Convert name offset to number
           .map(|mut x| x.read_u32::<LittleEndian>().unwrap() as usize )
           .map(|x| (x, self.bin[x..].read_string_zero().unwrap()) )
           .map(|(offset, name)| Native {name: name, address: offset})
           .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::fs::File;
    use std::ffi::CString;
    use super::Plugin;
    use super::Native;

    fn load_fixture(filename: &str) -> Vec<u8> {
        let mut file_bin: Vec<u8> = Vec::new();
        {
            let mut file = File::open(format!("test/fixtures/{}", filename)).unwrap();
            file.read_to_end(&mut file_bin).unwrap();
        }

        file_bin
    }

    #[test]
    fn it_load_plugins_when_it_is_correct() {
        let amxmod_bin = load_fixture("simple.amx183");
        let extracted_plugin = Plugin::from(&amxmod_bin).unwrap();
        let expected_plugin = Plugin {
            flags: 2,
            defsize: 8,
            cod: 116,
            dat: 192,
            hea: 296,
            stp: 16680,
            cip: 4294967295,
            publics: 56,
            natives: 64,
            libraries: 72,
            pubvars: 72,
            tags: 72,
            nametable: 80,
            bin: amxmod_bin.to_vec(),
        };
        assert_eq!(extracted_plugin, expected_plugin);
    }

    #[test]
    fn it_read_opcodes() {
        let amxmod_bin = load_fixture("simple.amx183");
        let amxmod_plugin = Plugin::from(&amxmod_bin).unwrap();
        amxmod_plugin.opcodes();
    }

    #[test]
    fn it_read_natives() {
        let amxmod_bin = load_fixture("two_natives.amx183");
        let amxmod_plugin = Plugin::from(&amxmod_bin).unwrap();
        let natives = amxmod_plugin.natives();
        let expected_natives = [
            Native {
                name: CString::new("native_one").unwrap(),
                address: 87,
            },
            Native {
                name: CString::new("native_two").unwrap(),
                address: 98,
            },
        ];

        assert_eq!(natives, expected_natives);
    }
}
