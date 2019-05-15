use std::convert::TryFrom;
use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};
use failure::Error;
use log::trace;

use super::{File, COMPATIBLE_VERSION, MAGIC};

impl TryFrom<Vec<u8>> for File {
    type Error = Error;

    fn try_from(bin: Vec<u8>) -> Result<Self, Self::Error> {
        let sections = {
            let mut reader = Cursor::new(&bin);

            // magic
            let magic = match reader.read_u32::<LittleEndian>() {
                Ok(magic) => {
                    if magic != MAGIC {
                        return Err(format_err!(
                            "Invalid file magic, expected: 0x{:X}, got: 0x{:X}",
                            MAGIC,
                            magic
                        ));
                    }
                    magic
                }
                Err(_) => return Err(format_err!("Magic EOF")),
            };
            trace!("File magic is 0x{:X}", magic);

            // version
            let version = match reader.read_u16::<LittleEndian>() {
                Ok(version) => {
                    if version != COMPATIBLE_VERSION {
                        return Err(format_err!(
                            "Incompatible file version, expected: {}, got: {}",
                            COMPATIBLE_VERSION,
                            version
                        ));
                    }
                    version
                }
                Err(_) => return Err(format_err!("Version EOF")),
            };
            trace!("Version is 0x{:X}", version);

            // sections count
            let sections = match reader.read_u8() {
                Ok(s) => {
                    if s < 1 {
                        return Err(format_err!("Zero sections amount"));
                    }

                    if s > 2 {
                        return Err(format_err!("More than two sections (malicious file?)"));
                    }

                    s
                }
                Err(_) => return Err(format_err!("Sections EOF")),
            };
            trace!("File has {} sections", sections);
            sections
        };

        Ok(File { bin, sections })
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use std::fs::File;
    use std::io::prelude::*;

    use super::File as AmxmodxFile;

    fn load_fixture(filename: &str) -> Vec<u8> {
        let mut file_bin: Vec<u8> = Vec::new();
        {
            let mut file = File::open(format!("test/fixtures/{}", filename)).unwrap();
            file.read_to_end(&mut file_bin).unwrap();
        }

        file_bin
    }

    #[test]
    fn it_load_file_when_binary_is_correct() {
        let amxmodx_bin = load_fixture("simple.amxx183");
        assert!(AmxmodxFile::try_from(amxmodx_bin).is_ok());
    }

    #[test]
    fn it_err_on_empty_file() {
        let amxmodx_bin = vec![];
        let result = AmxmodxFile::try_from(amxmodx_bin).err().unwrap();
        assert_eq!(result.cause().to_string(), "Magic EOF");
    }

    #[test]
    fn it_err_on_magic_eof() {
        let amxmodx_bin = vec![0, 0, 0];
        let result = AmxmodxFile::try_from(amxmodx_bin).err().unwrap();
        assert_eq!(result.cause().to_string(), "Magic EOF");
    }

    #[test]
    fn it_err_on_invalid_magic() {
        let amxmodx_bin = vec![0, 0, 0, 0];
        let result = AmxmodxFile::try_from(amxmodx_bin).err().unwrap();
        assert_eq!(
            result.cause().to_string(),
            "Invalid file magic, expected: 0x414D5858, got: 0x0"
        );
    }

    #[test]
    fn it_err_on_version_eof() {
        // Correct magic, incorrect version
        let amxmodx_bin = vec![88, 88, 77, 65, 0];
        let result = AmxmodxFile::try_from(amxmodx_bin).err().unwrap();
        assert_eq!(result.cause().to_string(), "Version EOF");
    }

    #[test]
    fn it_err_on_incompatible_version() {
        // Correct magic, incorrect version
        let amxmodx_bin = vec![88, 88, 77, 65, 0, 4];
        let result = AmxmodxFile::try_from(amxmodx_bin).err().unwrap();
        assert_eq!(
            result.cause().to_string(),
            "Incompatible file version, expected: 768, got: 1024"
        );
    }

    #[test]
    fn it_err_on_sections_eof() {
        // Correct magic, correct version, no section byte
        let amxmodx_bin = vec![88, 88, 77, 65, 0, 3];
        let result = AmxmodxFile::try_from(amxmodx_bin).err().unwrap();
        assert_eq!(result.cause().to_string(), "Sections EOF");
    }

    #[test]
    fn it_err_on_zero_sections() {
        // Correct magic, correct version, zero sections
        let amxmodx_bin = vec![88, 88, 77, 65, 0, 3, 0];
        let result = AmxmodxFile::try_from(amxmodx_bin).err().unwrap();
        assert_eq!(result.cause().to_string(), "Zero sections amount");
    }

    #[test]
    fn it_err_on_more_than_two_sections() {
        // Correct magic, correct version, 3 sections
        let amxmodx_bin = vec![88, 88, 77, 65, 0, 3, 3];
        let result = AmxmodxFile::try_from(amxmodx_bin).err().unwrap();
        assert_eq!(
            result.cause().to_string(),
            "More than two sections (malicious file?)"
        );
    }
}
