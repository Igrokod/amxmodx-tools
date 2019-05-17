use std::ffi::CString;

#[derive(Debug, PartialEq)]
pub struct Native {
    pub name: CString,
    pub address: usize,
}
