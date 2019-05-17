// TODO: Make it sum of mem::size_of calls
pub(crate) const HEADER_SIZE: usize = 17;

#[derive(Copy, Clone, Debug, PartialEq)]
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
}
