use std::convert::TryFrom;
use std::io::Cursor;
use std::mem::size_of;

use super::{File, ParseError};

// magic:u32 + version:u16 + sections:u8
const AMXX_HEADER_SIZE: usize = size_of::<u32>() + size_of::<u16>() + size_of::<u8>();

impl TryFrom<&[u8]> for File {
    type Error = ParseError;

    fn try_from(source: &[u8]) -> Result<File, Self::Error> {
        Ok(File {})
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use std::fs::File as IoFile;
    use std::io::{self, Read};

    use super::File as AmxxFile;

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
    fn it_parses_correct_file() {
        let plugin_bin = read_file("test/fixtures/amxx/simple.amxx183");
        let plugin = AmxxFile::try_from(&plugin_bin[..]).expect("Could not parse amxx file");
    }
}
