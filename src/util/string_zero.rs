use std::ffi::CString;

pub trait ReadByteString {
    fn read_string_zero(&self) -> Option<CString>;
}

impl ReadByteString for [u8] {
    fn read_string_zero(&self) -> Option<CString> {
        let pos = self.iter().position(|&x| x == 0)?;
        CString::new(&self[..pos]).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::CString;
    use super::ReadByteString;

    #[test]
    fn it_read_zero_terminated_string_to_zerobyte() {
        assert_eq!(
            Some(CString::new("hello").unwrap()),
            b"hello\0hehe"[..].read_string_zero()
        );
    }
}
