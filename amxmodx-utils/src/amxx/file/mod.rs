mod errors;
mod parser;
pub mod section;

pub use errors::ParseError;
pub use section::{Section, SectionsIterator};

#[derive(Debug)]
pub struct File {
    sections_count: u8,
    sections_bin: Vec<u8>,
}

impl File {
    pub fn sections(&self) -> SectionsIterator {
        SectionsIterator::new(self.sections_count, &self.sections_bin)
    }

    // TODO: test
    pub fn sections_count(&self) -> u8 {
        self.sections_count
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::{File as AmxxFile, SectionsIterator};

    #[test]
    fn it_returns_sections_iterator() {
        const HEADER: &[u8] = b"XXMA\0\x03\x01\x01";
        const EXPECTED_SECTIONS_BINARY: &[u8] = b"\x01";

        match AmxxFile::try_from(HEADER).unwrap().sections() {
            SectionsIterator {
                sections_count: 1,
                bin: EXPECTED_SECTIONS_BINARY,
                ..
            } => (),
            SectionsIterator {
                sections_count,
                bin,
                ..
            } => panic!(
                "sections() must return sections iterator with inherited values, actual: {:?}",
                SectionsIterator::new(sections_count, bin)
            ),
        }
    }
}
