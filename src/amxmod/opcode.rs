use byteorder::{ReadBytesExt, LittleEndian};
use std::str;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use enum_primitive::FromPrimitive;

enum_from_primitive! {
#[derive(Debug, PartialEq)]
#[allow(non_snake_case)]
pub enum OpcodeType {
    OP_NONE, // invalid opcode
    OP_LOAD_PRI,
    OP_LOAD_ALT,
    OP_LOAD_S_PRI,
    OP_LOAD_S_ALT,
    OP_LREF_PRI,
    OP_LREF_ALT,
    OP_LREF_S_PRI,
    OP_LREF_S_ALT,
    OP_LOAD_I,
    OP_LODB_I,
    OP_CONST_PRI,
    OP_CONST_ALT,
    OP_ADDR_PRI,
    OP_ADDR_ALT,
    OP_STOR_PRI,
    OP_STOR_ALT,
    OP_STOR_S_PRI,
    OP_STOR_S_ALT,
    OP_SREF_PRI,
    OP_SREF_ALT,
    OP_SREF_S_PRI,
    OP_SREF_S_ALT,
    OP_STOR_I,
    OP_STRB_I,
    OP_LIDX,
    OP_LIDX_B,
    OP_IDXADDR,
    OP_IDXADDR_B,
    OP_ALIGN_PRI,
    OP_ALIGN_ALT,
    OP_LCTRL,
    OP_SCTRL,
    OP_MOVE_PRI,
    OP_MOVE_ALT,
    OP_XCHG,
    OP_PUSH_PRI,
    OP_PUSH_ALT,
    OP_PUSH_R,
    OP_PUSH_C,
    OP_PUSH,
    OP_PUSH_S,
    OP_POP_PRI,
    OP_POP_ALT,
    OP_STACK,
    OP_HEAP,
    OP_PROC,
    OP_RET,
    OP_RETN,
    OP_CALL,
    OP_CALL_PRI,
    OP_JUMP,
    OP_JREL,
    OP_JZER,
    OP_JNZ,
    OP_JEQ,
    OP_JNEQ,
    OP_JLESS,
    OP_JLEQ,
    OP_JGRTR,
    OP_JGEQ,
    OP_JSLESS,
    OP_JSLEQ,
    OP_JSGRTR,
    OP_JSGEQ,
    OP_SHL,
    OP_SHR,
    OP_SSHR,
    OP_SHL_C_PRI,
    OP_SHL_C_ALT,
    OP_SHR_C_PRI,
    OP_SHR_C_ALT,
    OP_SMUL,
    OP_SDIV,
    OP_SDIV_ALT,
    OP_UMUL,
    OP_UDIV,
    OP_UDIV_ALT,
    OP_ADD,
    OP_SUB,
    OP_SUB_ALT,
    OP_AND,
    OP_OR,
    OP_XOR,
    OP_NOT,
    OP_NEG,
    OP_INVERT,
    OP_ADD_C,
    OP_SMUL_C,
    OP_ZERO_PRI,
    OP_ZERO_ALT,
    OP_ZERO,
    OP_ZERO_S,
    OP_SIGN_PRI,
    OP_SIGN_ALT,
    OP_EQ,
    OP_NEQ,
    OP_LESS,
    OP_LEQ,
    OP_GRTR,
    OP_GEQ,
    OP_SLESS,
    OP_SLEQ,
    OP_SGRTR,
    OP_SGEQ,
    OP_EQ_C_PRI,
    OP_EQ_C_ALT,
    OP_INC_PRI,
    OP_INC_ALT,
    OP_INC,
    OP_INC_S,
    OP_INC_I,
    OP_DEC_PRI,
    OP_DEC_ALT,
    OP_DEC,
    OP_DEC_S,
    OP_DEC_I,
    OP_MOVS,
    OP_CMPS,
    OP_FILL,
    OP_HALT,
    OP_BOUNDS,
    OP_SYSREQ_PRI,
    OP_SYSREQ_C,
    OP_FILE,    // obsolete
    OP_LINE,    // obsolete
    OP_SYMBOL,  // obsolete
    OP_SRANGE,  // obsolete
    OP_JUMP_PRI,
    OP_SWITCH,
    OP_CASETBL,
    OP_SWAP_PRI,
    OP_SWAP_ALT,
    OP_PUSHADDR,
    OP_NOP,
    OP_SYSREQ_D,
    OP_SYMTAG,  // obsolete
    OP_BREAK, // End of AMXX op codes
    // List of rxxma pseudo opcodes, careful!
    OP_CASENONE,
    OP_CASE,
    OP_CASEJMP
}}

use self::OpcodeType::*;

const SINGLE_PARAM_OPCODES: [u32; 74] = [
    OP_LOAD_PRI as u32,
    OP_LOAD_ALT as u32,
    OP_LOAD_S_PRI as u32,
    OP_LOAD_S_ALT as u32,
    OP_LREF_PRI as u32,
    OP_LREF_ALT as u32,
    OP_LREF_S_PRI as u32,
    OP_LREF_S_ALT as u32,
    OP_LODB_I as u32,
    OP_CONST_PRI as u32,
    OP_CONST_ALT as u32,
    OP_ADDR_PRI as u32,
    OP_ADDR_ALT as u32,
    OP_STOR_PRI as u32,
    OP_STOR_ALT as u32,
    OP_STOR_S_PRI as u32,
    OP_STOR_S_ALT as u32,
    OP_SREF_PRI as u32,
    OP_SREF_ALT as u32,
    OP_SREF_S_PRI as u32,
    OP_SREF_S_ALT as u32,
    OP_STRB_I as u32,
    OP_LIDX_B as u32,
    OP_IDXADDR_B as u32,
    OP_ALIGN_PRI as u32,
    OP_ALIGN_ALT as u32,
    OP_LCTRL as u32,
    OP_SCTRL as u32,
    OP_PUSH_R as u32,
    OP_PUSH_C as u32,
    OP_PUSH as u32,
    OP_PUSH_S as u32,
    OP_STACK as u32,
    OP_HEAP as u32,
    OP_CALL as u32,
    OP_JUMP as u32,
    OP_JREL as u32,
    OP_JZER as u32,
    OP_JNZ as u32,
    OP_JEQ as u32,
    OP_JNEQ as u32,
    OP_JLESS as u32,
    OP_JLEQ as u32,
    OP_JGRTR as u32,
    OP_JGEQ as u32,
    OP_JSLESS as u32,
    OP_JSLEQ as u32,
    OP_JSGRTR as u32,
    OP_JSGEQ as u32,
    OP_SHL as u32,
    OP_SHR as u32,
    OP_SSHR as u32,
    OP_SHL_C_PRI as u32,
    OP_SHL_C_ALT as u32,
    OP_SHR_C_PRI as u32,
    OP_SHR_C_ALT as u32,
    OP_ADD_C as u32,
    OP_SMUL_C as u32,
    OP_ZERO as u32,
    OP_ZERO_S as u32,
    OP_EQ_C_PRI as u32,
    OP_EQ_C_ALT as u32,
    OP_INC as u32,
    OP_INC_S as u32,
    OP_DEC as u32,
    OP_DEC_S as u32,
    OP_MOVS as u32,
    OP_CMPS as u32,
    OP_FILL as u32,
    OP_HALT as u32,
    OP_BOUNDS as u32,
    OP_SYSREQ_C as u32,
    OP_SWITCH as u32,
    OP_PUSHADDR as u32,
];

#[derive(Debug, PartialEq)]
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
            Err(e) => return Ok(None), // Return no opcode, end of cod section
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

        if opcode.code == OP_SHL {
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
    use std::io::Cursor;
    use super::Opcode;
    use super::OpcodeType::*;

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
