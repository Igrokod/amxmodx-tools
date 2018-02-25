use std::io::Cursor;
use byteorder::{ReadBytesExt, LittleEndian};
use std::str;
use std::io::Read;
use std::io;

#[derive(Debug, PartialEq)]
pub struct Opcode {
    code: u32,
}

impl Opcode {
    pub fn read_from<T: Read>(cod_reader: &mut T) -> Option<Opcode> {
        let code = match cod_reader.read_u32::<LittleEndian>() {
            Ok(c) => c,
            Err(e) => return None,
        };

        Some(Opcode { code: code })
    }
}

#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::fs::File;
    use std::io::Cursor;
    use super::Opcode;

    #[test]
    fn it_read_opcode() {
        let mut cursor = Cursor::new([0, 0, 0, 0]);
        let opcode = Opcode::read_from(&mut cursor).unwrap();
        assert_eq!(opcode.code, 0);
    }

    #[test]
    fn it_do_not_err_on_eof() {
        let mut cursor = Cursor::new([]);
        assert!(Opcode::read_from(&mut cursor).is_none());
    }
}
