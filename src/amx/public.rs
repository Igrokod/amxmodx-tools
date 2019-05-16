use std::ffi::CString;

#[derive(Debug, PartialEq)]
pub struct Public {
    pub name: CString,
    pub address: usize,
}
