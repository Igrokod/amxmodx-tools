use std::io::Cursor;
use byteorder::{ReadBytesExt, LittleEndian};
use std::str;
use super::Opcode;

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
    bin: Vec<u8>,
}

impl Plugin {
    pub fn from<'a>(bin: &[u8]) -> Result<Plugin, &'a str> {
        let mut reader = Cursor::new(bin);

        let size = match reader.read_u32::<LittleEndian>() {
            Ok(s) => s,
            Err(_) => return Err("EOF on amx size"),
        };

        // FIXME: Test magic is correct
        let magic = match reader.read_u16::<LittleEndian>() {
            Ok(m) => m,
            Err(_) => return Err("EOF on amx magic"),
        };

        // FIXME: Test file_version is correct
        let file_version = match reader.read_u8() {
            Ok(v) => v,
            Err(_) => return Err("EOF on amx file version"),
        };

        // FIXME: Test amx_version is correct
        let amx_version = match reader.read_u8() {
            Ok(v) => v,
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

        let cod = match reader.read_u32::<LittleEndian>() {
            Ok(c) => c,
            Err(_) => return Err("EOF on amx cod"),
        };

        let dat = match reader.read_u32::<LittleEndian>() {
            Ok(d) => d,
            Err(_) => return Err("EOF on amx dat"),
        };

        let hea = match reader.read_u32::<LittleEndian>() {
            Ok(h) => h,
            Err(_) => return Err("EOF on amx hea"),
        };

        let stp = match reader.read_u32::<LittleEndian>() {
            Ok(s) => s,
            Err(_) => return Err("EOF on amx stp"),
        };

        let cip = match reader.read_u32::<LittleEndian>() {
            Ok(c) => c,
            Err(_) => return Err("EOF on amx cip"),
        };

        let publics = match reader.read_u32::<LittleEndian>() {
            Ok(p) => p,
            Err(_) => return Err("EOF on amx publics"),
        };

        let natives = match reader.read_u32::<LittleEndian>() {
            Ok(n) => n,
            Err(_) => return Err("EOF on amx natives"),
        };

        let libraries = match reader.read_u32::<LittleEndian>() {
            Ok(l) => l,
            Err(_) => return Err("EOF on amx libraries"),
        };

        let pubvars = match reader.read_u32::<LittleEndian>() {
            Ok(p) => p,
            Err(_) => return Err("EOF on amx pubvars"),
        };

        let tags = match reader.read_u32::<LittleEndian>() {
            Ok(t) => t,
            Err(_) => return Err("EOF on amx tags"),
        };

        let nametable = match reader.read_u32::<LittleEndian>() {
            Ok(n) => n,
            Err(_) => return Err("EOF on amx nametable"),
        };

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
        let cod_size = self.dat - self.cod;
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
                Ok(Some(o)) => opcodes.push(o),
                Ok(None) => break,
                Err(e) => return Err(e),
            }
        }

        Ok(opcodes)
    }
}

#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::fs::File;
    use super::Plugin;

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
}
