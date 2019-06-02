use std::ffi::CStr;

pub trait ByteStringExt {
    fn read_cstr(&self) -> Option<&CStr>;
}

impl ByteStringExt for [u8] {
    fn read_cstr(&self) -> Option<&CStr> {
        let pos = self.iter().position(|&x| x == 0)?;
        let c_str = CStr::from_bytes_with_nul(&self[..=pos])
            .expect("We already checked it does not have null bytes");
        Some(c_str)
    }
}

#[cfg(test)]
mod tests {
    use super::ByteStringExt;
    use std::ffi::{CStr, CString};

    const BYTE_STRING: &[u8] = b"hello\0world";

    #[test]
    fn it_read_zero_terminated_string_up_to_zero_byte() {
        let c_string: &CStr = &CString::new("hello").unwrap();

        assert_eq!(Some(c_string), BYTE_STRING.read_cstr());
    }

    #[test]
    fn it_returns_nothing_on_non_terminated_slice() {
        assert!(b"Hello".read_cstr().is_none());
    }
}
