use std::io::Cursor;
use byteorder::{ReadBytesExt, LittleEndian};
use std::str;
use flate2::read::ZlibDecoder;
use std::io::Read;
use super::super::amxmod::Plugin;

#[derive(Debug, PartialEq)]
pub struct Section {
    pub cellsize: u8,
    pub disksize: u32,
    pub imagesize: u32,
    pub memsize: u32,
    pub offset: usize,
}

impl Section {
    pub const SIZE: usize = 17; // Packed section size

    pub fn from(bin: &[u8]) -> Result<Section, &str> {
        let mut reader = Cursor::new(bin);

        let cellsize = match reader.read_u8() {
            Ok(cs) => {
                if !(cs == 4 || cs == 8) {
                    return Err("Invalid section cellsize, must be 4 or 8");
                }

                cs
            }
            Err(_) => return Err("EOF on section cellsize"),
        };
        trace!("cellsize:\t{}", cellsize);

        let disksize = match reader.read_u32::<LittleEndian>() {
            Ok(ds) => ds,
            Err(_) => return Err("EOF on section disksize"),
        };
        trace!("disksize:\t{}", disksize);

        let imagesize = match reader.read_u32::<LittleEndian>() {
            Ok(is) => is,
            Err(_) => return Err("EOF on section imagesize"),
        };
        trace!("imagesize:\t{}", imagesize);

        let memsize = match reader.read_u32::<LittleEndian>() {
            Ok(ms) => ms,
            Err(_) => return Err("EOF on section memsize"),
        };
        trace!("memsize:\t{}", memsize);

        let offset = match reader.read_u32::<LittleEndian>() {
            Ok(offs) => offs,
            Err(_) => return Err("EOF on section amx gz offset"),
        };
        trace!("offset:\t{}", offset);

        Ok(Section {
            cellsize: cellsize,
            disksize: disksize,
            imagesize: imagesize,
            memsize: memsize,
            offset: offset as usize,
        })
    }

    pub fn unpack_section<'a>(&self, bin: &[u8]) -> Result<Plugin, &'a str> {
        let gzip_slice = &bin[self.offset..];
        let mut amx_bin: Vec<u8> = Vec::new();

        match ZlibDecoder::new(gzip_slice).read_to_end(&mut amx_bin) {
            Ok(_) => (),
            Err(_) => return Err("amx gz unpack error"),
        };

        if amx_bin.len() != self.imagesize as usize {
            return Err("amx bin size does not match section imagesize");
        }

        let plugin = Plugin::from(&amx_bin);
        plugin
    }
}

#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::fs::File;
    use super::Section;

    const AMXX_HEADER_SIZE: usize = 7;

    fn load_fixture(filename: &str) -> Vec<u8> {
        let mut file_bin: Vec<u8> = Vec::new();
        {
            let mut file = File::open(format!("test/fixtures/{}", filename)).unwrap();
            file.read_to_end(&mut file_bin).unwrap();
        }

        file_bin
    }

    #[test]
    fn it_load_section_from_real_file_when_it_is_correct() {
        // File with single section.
        let amxmodx_bin = load_fixture("simple.amxx183");
        // Skip amxmodx header, leaving only section headers
        let section_bin = &amxmodx_bin[AMXX_HEADER_SIZE..];
        assert!(Section::from(section_bin).is_ok());
    }

    #[test]
    fn it_unpack_section_without_errors() {
        // File with single section.
        let amxmodx_bin = load_fixture("simple.amxx183");
        let section_bin = &amxmodx_bin[AMXX_HEADER_SIZE..];
        let section = Section::from(section_bin).unwrap();
        section.unpack_section(&amxmodx_bin).unwrap();
    }

    #[test]
    fn it_err_on_cellsize_eof() {
        // empty section header
        let section_bin = vec![];
        assert_eq!(
            Section::from(&section_bin).err().unwrap(),
            "EOF on section cellsize"
        );
    }

    #[test]
    fn it_err_on_invalid_cellsize() {
        // invalid cellsize
        let section_bin = vec![0];
        assert_eq!(
            Section::from(&section_bin).err().unwrap(),
            "Invalid section cellsize, must be 4 or 8"
        );
    }

    #[test]
    fn it_err_on_disksize_eof() {
        // 1 cellsize
        // empty disksize
        let section_bin = vec![4];
        assert_eq!(
            Section::from(&section_bin).err().unwrap(),
            "EOF on section disksize"
        );
    }

    #[test]
    fn it_err_on_imagesize_eof() {
        // 1 cellsize
        // 4 disksize
        // empty imagesize
        let mut section_bin = vec![4, 0, 0, 0, 0];
        section_bin[0] = 4;
        assert_eq!(
            Section::from(&section_bin).err().unwrap(),
            "EOF on section imagesize"
        );
    }

    #[test]
    fn it_err_on_memsize_eof() {
        // 1 cellsize
        // 4 disksize
        // 4 imagesize
        // empty memsize
        let mut section_bin = vec![0; 9];
        section_bin[0] = 4;
        assert_eq!(
            Section::from(&section_bin).err().unwrap(),
            "EOF on section memsize"
        );
    }

    #[test]
    fn it_err_on_offset_eof() {
        // 1 cellsize
        // 4 disksize
        // 4 imagesize
        // 4 memsize
        // empty offset
        let mut section_bin = vec![0; 13];
        section_bin[0] = 4;
        assert_eq!(
            Section::from(&section_bin).err().unwrap(),
            "EOF on section amx gz offset"
        );
    }

    #[test]
    fn it_load_section_when_it_is_correct() {
        // 1 cellsize
        // 4 disksize
        // 4 imagesize
        // 4 memsize
        // 4 offset
        let mut section_bin = vec![0; 17];
        section_bin[0] = 4;
        let extracted_section = Section::from(&section_bin);
        assert!(extracted_section.is_ok());
        let expected_section = Section {
            cellsize: 4,
            disksize: 0,
            imagesize: 0,
            memsize: 0,
            offset: 0,
        };
        assert_eq!(extracted_section.unwrap(), expected_section);
    }
}
