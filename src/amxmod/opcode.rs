use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::str;

use byteorder::{LittleEndian, ReadBytesExt};
use enum_primitive::FromPrimitive;
use log::trace;

use super::opcode_type::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Opcode {
    pub code: OpcodeType,
    pub address: usize,
    pub param: Option<u32>,
}

impl Opcode {
    pub fn read_from<T: Read + Seek>(
        cod_reader: &mut T,
    ) -> Result<Option<Vec<Opcode>>, &'static str> {
        // In case we return multiple
        let mut opcodes: Vec<Opcode> = vec![];

        let address = Opcode::read_addr(cod_reader)?;

        // FIXME: Check for invalid opcode
        let code = match cod_reader.read_u32::<LittleEndian>() {
            Ok(c) => c,
            Err(_) => return Ok(None), // Return no opcode, end of cod section
        };
        // for debugging purposes
        trace!("0x{:X}\tOpcode: {}", address, code);

        let enum_code = match OpcodeType::from_u32(code) {
            Some(c) => c,
            None => return Err("invalid opcode found"),
        };
        // for debugging purposes
        trace!("As enum: {:?}", enum_code);

        // TODO: Test param
        let param = if SINGLE_PARAM_OPCODES.contains(&code) {
            trace!("Reading param");
            match Opcode::read_param(cod_reader) {
                Ok(p) => Some(p),
                Err(_) => return Err("opcode declared to have param but it's .COD EOF instead"),
            }
        } else {
            None
        };

        let opcode = Opcode {
            code: enum_code,
            address: address as usize,
            param: param,
        };

        if opcode.code == OP_SHL || opcode.code == OP_SSHR {
            // FIXME: Check compiler for
            // CONST.pri  0x1
            // LOAD.alt  0x2528C     ; weaponid
            // SHL  0xC
            // UNKNOWN OP CODE: 0x4030C02
            match Opcode::read_param(cod_reader) {
                Ok(p) => p,
                Err(_) => return Err("EOF on SHL Hack"),
            };
        }

        let is_casetbl = opcode.code == OP_CASETBL;
        opcodes.push(opcode);

        if is_casetbl {
            match Opcode::read_case_table(cod_reader) {
                Ok(v) => opcodes.extend(v),
                Err(e) => return Err(e),
            };
        };

        Ok(Some(opcodes))
    }

    fn read_case_table<T: Read + Seek>(cod_reader: &mut T) -> Result<Vec<Opcode>, &'static str> {
        trace!("Process case table");
        let mut opcodes: Vec<Opcode> = vec![];

        let number_of_jumps = match Opcode::read_param(cod_reader) {
            Ok(p) => p,
            Err(_) => return Err("casetbl number of jumps unexpected EOF"),
        };
        trace!("Case table number of jumps: {}", number_of_jumps);

        let address = Opcode::read_addr(cod_reader)?;
        let none_found_param = match Opcode::read_param(cod_reader) {
            Ok(p) => p,
            Err(_) => return Err("casetbl 'none found' param: unexpected EOF"),
        };

        // for debugging purposes
        trace!("CASENONE case, param: {0}/0x{0:X}", none_found_param);

        let none_found_opcode = Opcode {
            code: OP_CASENONE,
            address: address,
            param: Some(none_found_param),
        };
        opcodes.push(none_found_opcode);

        for i in 0..number_of_jumps {
            trace!("Process casetbl case #{}", i);
            let address = Opcode::read_addr(cod_reader)?;
            let case_param = match Opcode::read_param(cod_reader) {
                Ok(p) => p,
                Err(_) => return Err("casetbl 'case' param: unexpected EOF"),
            };
            trace!("CASE {}", case_param);
            let case_op = Opcode {
                code: OP_CASE,
                address: address,
                param: Some(case_param),
            };
            opcodes.push(case_op);

            let address = Opcode::read_addr(cod_reader)?;
            let case_jmp_param = match Opcode::read_param(cod_reader) {
                Ok(p) => p,
                Err(_) => return Err("casetbl 'case' param: unexpected EOF"),
            };
            trace!("CASEJMP {}", case_jmp_param);
            let case_jmp = Opcode {
                code: OP_CASENONE,
                address: address,
                param: Some(case_param),
            };
            opcodes.push(case_jmp);
        }

        Ok(opcodes)
    }

    fn read_param<T: Read + Seek>(cod_reader: &mut T) -> Result<u32, io::Error> {
        cod_reader.read_u32::<LittleEndian>()
    }

    fn read_addr<T: Read + Seek>(cod_reader: &mut T) -> Result<usize, &'static str> {
        let address = match cod_reader.seek(SeekFrom::Current(0)) {
            Ok(c) => c,
            Err(_) => return Err("wtf: cannot seek on reader"),
        };

        Ok(address as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::Opcode;
    use super::OpcodeType::*;
    use std::io::Cursor;

    #[test]
    fn it_read_opcode() {
        let mut cursor = Cursor::new([0, 0, 0, 0]);
        let opcodes = Opcode::read_from(&mut cursor).unwrap().unwrap();
        assert_eq!(opcodes[0].code, OP_NONE);
    }

    #[test]
    fn it_do_not_err_on_eof() {
        let mut cursor = Cursor::new([]);
        assert!(Opcode::read_from(&mut cursor).unwrap().is_none());
    }
}
