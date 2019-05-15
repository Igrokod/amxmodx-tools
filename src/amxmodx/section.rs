use std::convert::TryFrom;
use std::io::{Cursor, Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};
use failure::{Error, ResultExt};
use flate2::read::ZlibDecoder;
use log::trace;

use super::super::amxmod::Plugin;

#[derive(Debug, PartialEq)]
pub struct Section {
    pub cellsize: u8,
    pub disksize: u32,
    pub imagesize: u32,
    pub memsize: u32,
    pub offset: usize,
    pub bin: Vec<u8>,
}

#[derive(Debug, Fail)]
enum SectionParseError {
    #[fail(display = "Invalid section cellsize, must be 4 or 8, got: {}", _0)]
    InvalidCellSize(u8),
    #[fail(display = "unexpected EOF on reading sections contents, disksize does not match")]
    ContentsEof,
    #[fail(display = "imagesize does not match section unpacked contents")]
    ImageSizeMismatch,
}

impl Section {
    pub const SIZE: usize = 17; // Packed section size

    pub fn from(bin: &[u8], section_header_offset: usize) -> Result<Section, Error> {
        let mut reader = Cursor::new(bin);
        reader
            .seek(SeekFrom::Start(section_header_offset as u64))
            .context("EOF on offseting to section header (wtf?)")?;

        let cellsize = reader.read_u8().context("EOF on section cellsize")?;
        if !(cellsize == 4 || cellsize == 8) {
            Err(SectionParseError::InvalidCellSize(cellsize))?;
        }
        trace!("cellsize:\t{}", cellsize);

        let disksize = reader
            .read_u32::<LittleEndian>()
            .context("EOF on section disksize")?;
        trace!("disksize:\t{}", disksize);

        let imagesize = reader
            .read_u32::<LittleEndian>()
            .context("EOF on section imagesize")?;
        trace!("imagesize:\t{}", imagesize);

        let memsize = reader
            .read_u32::<LittleEndian>()
            .context("EOF on section memsize")?;
        trace!("memsize:\t{}", memsize);

        let offset = reader
            .read_u32::<LittleEndian>()
            .context("EOF on section offset")?;
        trace!("offset:\t{}", offset);

        let mut section_bin = vec![0; disksize as usize];
        reader
            .seek(SeekFrom::Start(offset as u64))
            .context("EOF on offseting to section contents")?;
        reader
            .read_exact(&mut section_bin)
            .context(SectionParseError::ContentsEof)?;
        trace!("section contents size match disksize");

        Ok(Section {
            cellsize: cellsize,
            disksize: disksize,
            imagesize: imagesize,
            memsize: memsize,
            offset: offset as usize,
            bin: section_bin,
        })
    }

    pub fn unpack_section(&self) -> Result<Plugin, Error> {
        let imagesize = self.imagesize as usize;
        let mut amx_bin: Vec<u8> = Vec::with_capacity(imagesize);
        let reader = Cursor::new(&self.bin);
        // TODO: test
        ZlibDecoder::new(reader).read_to_end(&mut amx_bin)?;

        // TODO: test
        if amx_bin.len() != imagesize {
            Err(SectionParseError::ImageSizeMismatch)?;
        }

        // TODO: test
        let plugin = Plugin::try_from(amx_bin);
        plugin.map_err(|e| format_err!("{}", e))
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
        let mut file = File::open(format!("test/fixtures/{}", filename)).unwrap();
        file.read_to_end(&mut file_bin).unwrap();
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
            Section::from(&section_bin, 0)
                .err()
                .unwrap()
                .cause()
                .to_string(),
            "EOF on section cellsize"
        );
    }

    #[test]
    fn it_err_on_invalid_cellsize() {
        // invalid cellsize
        let section_bin = vec![0];
        assert_eq!(
            Section::from(&section_bin, 0)
                .err()
                .unwrap()
                .cause()
                .to_string(),
            "Invalid section cellsize, must be 4 or 8, got: 0"
        );
    }

    #[test]
    fn it_err_on_disksize_eof() {
        // 1 cellsize
        // empty disksize
        let section_bin = vec![4];
        assert_eq!(
            Section::from(&section_bin, 0)
                .err()
                .unwrap()
                .cause()
                .to_string(),
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
            Section::from(&section_bin, 0)
                .err()
                .unwrap()
                .cause()
                .to_string(),
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
            Section::from(&section_bin, 0)
                .err()
                .unwrap()
                .cause()
                .to_string(),
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
            Section::from(&section_bin, 0)
                .err()
                .unwrap()
                .cause()
                .to_string(),
            "EOF on section offset"
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
