use std::io::Cursor;
use byteorder::{ReadBytesExt, LittleEndian};
use std::str;

pub struct File<'a> {
    pub bin: &'a [u8],
}

impl<'a> File<'a> {
    const MAGIC: u32 = 0x414d5858;
    const COMPATIBLE_VERSION: u16 = 768;

    pub fn from(bin: &'a [u8]) -> Result<File, &str> {
        let mut reader = Cursor::new(bin);

        // magic
        match reader.read_u32::<LittleEndian>() {
            Ok(magick) => {
                if magick != File::MAGIC {
                    return Result::Err("Invalid file magic");
                }
            }
            Err(_) => return Result::Err("Magic EOF"),
        };

        // version
        match reader.read_u16::<LittleEndian>() {
            Ok(version) => {
                if version != File::COMPATIBLE_VERSION {
                    return Result::Err("Incompatible file version");
                }
            }
            Err(_) => return Result::Err("Version EOF"),
        };

        // sections count
        let sections = match reader.read_u8() {
            Ok(s) => s,
            Err(_) => return Result::Err("Sections EOF"),
        };

        println!("sections: {}", sections);

        Ok(File { bin: bin })
    }
}

#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::fs::File;
    use super::File as AmxmodxFile;

    #[test]
    fn it_load_file_when_binary_is_correct() {
        let mut amxmodx_bin: Vec<u8> = Vec::new();
        {
            let mut file = File::open("test/fixtures/simple.amxx183").unwrap();
            file.read_to_end(&mut amxmodx_bin).unwrap();
        }

        assert!(AmxmodxFile::from(&amxmodx_bin).is_ok());
    }

    #[test]
    fn it_err_on_empty_file() {
        let amxmodx_bin = vec![];
        let result = AmxmodxFile::from(&amxmodx_bin);
        assert!(result.is_err());
    }

    #[test]
    fn it_err_on_magic_eof() {
        let amxmodx_bin = vec![0, 0, 0];
        let result = AmxmodxFile::from(&amxmodx_bin);
        assert!(result.is_err());
    }

    #[test]
    fn it_err_on_invalid_magic() {
        let amxmodx_bin = vec![0, 0, 0, 0];
        let result = AmxmodxFile::from(&amxmodx_bin);
        assert!(result.is_err());
    }

    #[test]
    fn it_err_on_version_eof() {
        // Correct magic, incorrect version
        let amxmodx_bin = vec![88, 88, 77, 65, 0];
        let result = AmxmodxFile::from(&amxmodx_bin);
        assert!(result.is_err());
    }

    #[test]
    fn it_err_on_incompatible_version() {
        // Correct magic, incorrect version
        let amxmodx_bin = vec![88, 88, 77, 65, 0, 4];
        let result = AmxmodxFile::from(&amxmodx_bin);
        assert!(result.is_err());
    }
}
