use super::AMXX_HEADER_SIZE;
use super::File;
use super::super::Section;
use failure::Error;

impl File {
    pub fn sections(&self) -> Result<Vec<Section>, Error> {
        let mut sections: Vec<Section> = vec![];

        for i in 0..self.sections {
            trace!("---------------");
            trace!("Reading section {}", i + 1);
            let section_offset = AMXX_HEADER_SIZE + (Section::SIZE * i as usize);
            // TODO: Fix panic
            // let section_bin = &self.bin[section_offset..];
            let section = match Section::from(&self.bin, section_offset) {
                Ok(s) => s,
                Err(e) => return Err(e),
            };
            sections.push(section);
        }

        Ok(sections)
    }
}

#[cfg(test)]
mod tests {
    use super::File as AmxmodxFile;
    use super::super::super::Section;
    use std::fs::File;
    use std::io::prelude::*;
    use util::try_from::TryFrom;

    fn load_fixture(filename: &str) -> Vec<u8> {
        let mut file_bin: Vec<u8> = Vec::new();
        {
            let mut file = File::open(format!("test/fixtures/{}", filename)).unwrap();
            file.read_to_end(&mut file_bin).unwrap();
        }

        file_bin
    }

    #[test]
    fn it_return_multiple_sections() {
        let amxmodx_bin = load_fixture("simple.amxx181");
        let amxmodx_file = AmxmodxFile::try_from(amxmodx_bin).unwrap();
        let extracted_sections = amxmodx_file.sections().unwrap();
        let expected_sections = [
            Section {
                cellsize: 4,
                disksize: 161,
                imagesize: 288,
                memsize: 16672,
                offset: 41,
                bin: load_fixture("raw_sections/simple.amxx181_cell4.gz"),
            },
            Section {
                cellsize: 8,
                disksize: 177,
                imagesize: 488,
                memsize: 33256,
                offset: 202,
                bin: load_fixture("raw_sections/simple.amxx181_cell8.gz"),
            },
        ];
        assert_eq!(extracted_sections, expected_sections);
    }

    #[test]
    fn it_err_on_sections_parsing_eof() {
        // Correct magic, correct version, 2 sections, zero section headers
        let amxmodx_bin = vec![88, 88, 77, 65, 0, 3, 2];
        let amxmodx_file = AmxmodxFile::try_from(amxmodx_bin).unwrap();
        let result = amxmodx_file.sections();
        assert!(result.is_err());
    }
}
