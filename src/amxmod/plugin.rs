use std::io::Cursor;
use byteorder::{ReadBytesExt, LittleEndian};
use std::str;

#[derive(Debug, PartialEq)]
pub struct Plugin {}

impl Plugin {
    pub fn from<'a>(bin: &[u8]) -> Result<Plugin, &'a str> {
        let mut reader = Cursor::new(bin);
        Ok(Plugin {})
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
        assert!(Plugin::from(&amxmod_bin).is_ok());
    }
}
