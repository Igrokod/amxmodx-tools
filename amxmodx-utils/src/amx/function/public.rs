use std::borrow::Cow;

#[derive(Debug, PartialEq)]
pub struct Public<'amx_file_bin> {
    pub name: Cow<'amx_file_bin, str>,
    pub address: u32,
}

impl<'amx_file_bin> Public<'amx_file_bin> {
    pub fn new(name: Cow<'amx_file_bin, str>, address: u32) -> Self {
        Self { name, address }
    }
}
