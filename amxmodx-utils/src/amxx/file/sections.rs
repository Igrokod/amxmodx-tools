// TODO: Find a way to remove pub(crate)
#[derive(Debug)]
pub struct SectionsIterator<'sections_bin> {
    pub(crate) sections_count: u8,
    pub(crate) bin: &'sections_bin [u8],
}

impl<'sections_bin> SectionsIterator<'sections_bin> {
    pub fn new(sections_count: u8, bin: &'sections_bin [u8]) -> SectionsIterator<'sections_bin> {
        SectionsIterator {
            sections_count,
            bin,
        }
    }
}
