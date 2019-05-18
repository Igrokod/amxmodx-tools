use std::io::Cursor;

use bytes::Buf;

use super::super::parser::HEADER_SIZE as AMXX_HEADER_SIZE;
use super::super::ParseError;
use super::{Section, SectionMetadata, HEADER_SIZE as SECTION_HEADER_SIZE};

// TODO: Find a way to remove pub(crate)
#[derive(Debug)]
pub struct SectionsIterator<'sections_bin> {
    current_section: u8,
    stop_iteration: bool,
    pub(crate) sections_count: u8,
    pub(crate) bin: &'sections_bin [u8],
}

impl<'sections_bin> SectionsIterator<'sections_bin> {
    pub fn new(sections_count: u8, bin: &'sections_bin [u8]) -> SectionsIterator<'sections_bin> {
        let current_section = 0;
        let stop_iteration = false;

        SectionsIterator {
            current_section,
            stop_iteration,
            sections_count,
            bin,
        }
    }
}

impl<'sections_bin> Iterator for SectionsIterator<'sections_bin> {
    type Item = Result<Section<'sections_bin>, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop_iteration || self.current_section == self.sections_count {
            return None;
        }

        let current_position: usize = SECTION_HEADER_SIZE * usize::from(self.current_section);
        let section_range = current_position..(current_position + SECTION_HEADER_SIZE);

        let section_header = match self.bin.get(section_range) {
            Some(slice) => slice,
            None => {
                self.stop_iteration = true;
                // TODO: Custom error messages for section
                return Some(Err(ParseError::InvalidSection));
            }
        };

        let mut header_reader = Cursor::new(section_header);

        let cellsize = header_reader.get_u8();
        let disksize = header_reader.get_u32_le();
        let imagesize = header_reader.get_u32_le();
        let memsize = header_reader.get_u32_le();
        let metadata = SectionMetadata::new(cellsize, disksize, imagesize, memsize);

        let offset = (header_reader.get_u32_le() as usize) - AMXX_HEADER_SIZE;
        let section_compressed_body_range = offset..(offset + (disksize as usize));
        let compressed_body = match self.bin.get(section_compressed_body_range) {
            Some(slice) => slice,
            None => {
                self.stop_iteration = true;
                return Some(Err(ParseError::InvalidSection));
            }
        };

        self.current_section += 1;

        Some(Ok(Section::new(metadata, compressed_body)))
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{self, Read};

    use super::{SectionMetadata, SectionsIterator};

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
    fn it_iterates_sections_on_correct_file() {
        let sections_bin = read_file("test/fixtures/amxx/simple.amxx183.only_sections.amxx");
        let mut sections_iterator = SectionsIterator::new(1, &sections_bin);

        let section = sections_iterator
            .next()
            .expect("File have exactly one section")
            .expect("Section should be correctly parsed");

        assert!(sections_iterator.next().is_none());
        assert_eq!(
            section.metadata(),
            SectionMetadata {
                cellsize: 4,
                disksize: 330,
                imagesize: 578,
                memsize: 16680,
            }
        );
    }

    // TODO: Failing test cases
    // TODO: Test correct body is extracted
}
