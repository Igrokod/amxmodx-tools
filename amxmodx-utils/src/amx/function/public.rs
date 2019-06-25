use std::borrow::Cow;

#[derive(Debug, PartialEq)]
pub struct Public<'amx_file_bin> {
    name: Cow<'amx_file_bin, str>,
    address: u32,
}

impl<'amx_file_bin> Public<'amx_file_bin> {
    // TODO: Test
    pub fn new(name: Cow<'amx_file_bin, str>, address: u32) -> Self {
        Self { name, address }
    }

    // TODO: Test
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    // TODO: Test
    #[inline]
    pub fn address(&self) -> u32 {
        self.address
    }
}
