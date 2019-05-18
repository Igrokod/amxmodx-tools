mod sections_iterator;

pub use sections_iterator::SectionsIterator;

use flate2::read::ZlibDecoder;
use std::io::{self, Read};

// TODO: Calculate it using C style header struct
pub(crate) const HEADER_SIZE: usize = 17;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct SectionMetadata {
    pub cellsize: u8,
    pub disksize: u32,
    pub imagesize: u32,
    pub memsize: u32,
}

impl SectionMetadata {
    pub fn new(cellsize: u8, disksize: u32, imagesize: u32, memsize: u32) -> Self {
        SectionMetadata {
            cellsize,
            disksize,
            imagesize,
            memsize,
        }
    }
}

#[derive(Debug)]
pub struct Section<'compressed_body> {
    metadata: SectionMetadata,
    compressed_body: &'compressed_body [u8],
}

impl<'compressed_body> Section<'compressed_body> {
    pub fn new(metadata: SectionMetadata, compressed_body: &'compressed_body [u8]) -> Self {
        Section {
            metadata,
            compressed_body,
        }
    }

    pub fn metadata(&self) -> SectionMetadata {
        self.metadata
    }

    pub fn compressed_body(&self) -> &'compressed_body [u8] {
        self.compressed_body
    }

    pub fn unpack_body(&self) -> io::Result<Vec<u8>> {
        let mut decoder = ZlibDecoder::new(self.compressed_body);
        let mut unpacked_body = vec![];
        decoder.read_to_end(&mut unpacked_body)?;

        //        // TODO
        //        if unpacked_body.len() != self.metadata.imagesize {
        //            Err()?;
        //        }

        Ok(unpacked_body)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{self, Read};

    use super::{Section, SectionMetadata};

    fn _read_file(path: &str) -> io::Result<Vec<u8>> {
        let mut file = File::open(path)?;
        let mut plugin = vec![];
        file.read_to_end(&mut plugin)?;

        Ok(plugin)
    }

    fn read_file(path: &str) -> Vec<u8> {
        _read_file(path).expect(&format!("Could not read {} file", path))
    }

    #[test]
    fn it_returns_body() {
        const BIN: &[u8] = b"hello";
        let meta = SectionMetadata::new(0, 0, 0, 0);
        let section = Section::new(meta, BIN);

        assert_eq!(section.compressed_body(), BIN);
    }

    #[test]
    fn it_unpacks_body() {
        let bin = read_file("test/fixtures/amxx/simple.cellsize4_section.amxx183");
        let meta = SectionMetadata::default();
        let section = Section::new(meta, &bin);
        let unpacked_body = section
            .unpack_body()
            .expect("Section should have correct zlib stream");

        let expected_unpacked_body =
            read_file("test/fixtures/amxx/simple.cellsize4_section.unpacked.amxx183");

        assert_eq!(unpacked_body, expected_unpacked_body);
    }
}
