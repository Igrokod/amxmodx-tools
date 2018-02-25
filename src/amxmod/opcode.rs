use byteorder::{ReadBytesExt, LittleEndian};
use std::str;
use std::io::{Read, Seek, SeekFrom};
use enum_primitive::FromPrimitive;

enum_from_primitive! {
#[derive(Debug, PartialEq)]
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
    OP_BREAK // End of AMXX op codes
}}

#[derive(Debug, PartialEq)]
pub struct Opcode {
    pub code: OpcodeType,
    pub address: usize,
}

impl Opcode {
    pub fn read_from<T: Read + Seek>(cod_reader: &mut T) -> Result<Option<Opcode>, &'static str> {
        // TODO: Error handling?
        let address = cod_reader.seek(SeekFrom::Current(0)).unwrap();
        let code = match cod_reader.read_u32::<LittleEndian>() {
            Ok(c) => c,
            Err(e) => return Ok(None), // Return no opcode, end of cod section
        };

        let enum_code = match OpcodeType::from_u32(code) {
            Some(c) => c,
            None => return Err("invalid opcode found"),
        };

        // FIXME: Check for invalid opcode
        Ok(Some(Opcode {
            code: enum_code,
            address: address as usize,
        }))
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
        let opcode = Opcode::read_from(&mut cursor).unwrap().unwrap();
        assert_eq!(opcode.code, OP_NONE);
    }

    #[test]
    fn it_do_not_err_on_eof() {
        let mut cursor = Cursor::new([]);
        assert!(Opcode::read_from(&mut cursor).unwrap().is_none());
    }
}
