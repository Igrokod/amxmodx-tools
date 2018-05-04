use super::super::amxmod::Plugin;
use byteorder::{LittleEndian, ReadBytesExt};
use flate2::read::ZlibDecoder;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::str;

#[derive(Debug, PartialEq)]
pub struct Section {
    pub cellsize: u8,
    pub disksize: u32,
    pub imagesize: u32,
    pub memsize: u32,
    pub offset: usize,
    pub bin: Vec<u8>,
}

impl Section {
    pub const SIZE: usize = 17; // Packed section size

    pub fn from<'a>(bin: &'a [u8], section_header_offset: usize) -> Result<Section, &str> {
        let mut reader = Cursor::new(bin);
        // TODO: Error handling
        reader
            .seek(SeekFrom::Start(section_header_offset as u64))
            .unwrap();

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

        trace!("------");
        trace!("bin size: {}", bin.len());
        trace!("disk size: {}", disksize);
        trace!("image size: {}", imagesize);
        // TODO: Check readed size
        let mut section_bin = vec![0; disksize as usize];
        // let mut section_bin = Vec::new();
        // TODO: Error handler
        reader.seek(SeekFrom::Start(offset as u64)).unwrap();
        // TODO: Error handler
        // TODO: Correct size read
        reader.read_exact(&mut section_bin).unwrap();

        // TODO: Check if necessary
        // if let Err(_) = reader.read_exact(&mut section_bin) {
        //     return Err("EOF on reading section contents");
        // };
        trace!("section contents saved");

        Ok(Section {
            cellsize: cellsize,
            disksize: disksize,
            imagesize: imagesize,
            memsize: memsize,
            offset: offset as usize,
            bin: section_bin,
        })
    }

    pub fn unpack_section<'a>(&self) -> Result<Plugin, &'a str> {
        let mut amx_bin: Vec<u8> = vec![0; self.imagesize as usize];
        // let mut amx_bin: Vec<u8> = Vec::new();
        let reader = Cursor::new(&self.bin);

        match ZlibDecoder::new(reader).read_exact(&mut amx_bin) {
            Ok(_) => (),
            Err(_) => return Err("amx gz unpack error"),
        };

        // TODO: Check if check is really necessary
        // if amx_bin.len() != self.imagesize as usize {
        //     return Err("amx bin size does not match section imagesize");
        // }

        let plugin = Plugin::from(&amx_bin);
        plugin
    }
}

#[cfg(test)]
mod tests {
    use super::Section;
    use std::fs::File;
    use std::io::prelude::*;

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
        assert!(Section::from(&amxmodx_bin, AMXX_HEADER_SIZE).is_ok());
    }

    #[test]
    fn it_unpack_section_without_errors() {
        // File with single section.
        let amxmodx_bin = load_fixture("simple.amxx183");
        let section = Section::from(&amxmodx_bin, AMXX_HEADER_SIZE).unwrap();
        section.unpack_section().unwrap();
    }

    #[test]
    fn it_err_on_cellsize_eof() {
        // empty section header
        let section_bin = vec![];
        assert_eq!(
            Section::from(&section_bin, 0).err().unwrap(),
            "EOF on section cellsize"
        );
    }

    #[test]
    fn it_err_on_invalid_cellsize() {
        // invalid cellsize
        let section_bin = vec![0];
        assert_eq!(
            Section::from(&section_bin, 0).err().unwrap(),
            "Invalid section cellsize, must be 4 or 8"
        );
    }

    #[test]
    fn it_err_on_disksize_eof() {
        // 1 cellsize
        // empty disksize
        let section_bin = vec![4];
        assert_eq!(
            Section::from(&section_bin, 0).err().unwrap(),
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
            Section::from(&section_bin, 0).err().unwrap(),
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
            Section::from(&section_bin, 0).err().unwrap(),
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
            Section::from(&section_bin, 0).err().unwrap(),
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
        let extracted_section = Section::from(&section_bin, 0).unwrap();
        let expected_section = Section {
            cellsize: 4,
            disksize: 0,
            imagesize: 0,
            memsize: 0,
            offset: 0,
            bin: vec![],
        };
        assert_eq!(extracted_section, expected_section);
    }
}
