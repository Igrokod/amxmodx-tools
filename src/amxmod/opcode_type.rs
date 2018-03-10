use std::fmt;

enum_from_primitive! {
#[derive(Copy, Clone, Debug, PartialEq)]
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
    OP_CASEJMP,
    OP_FAKE // Used only in tests, never used in real code
}}

pub use self::OpcodeType::*;

pub const SINGLE_PARAM_OPCODES: [u32; 74] = [
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

const OPCODE_FMT_NAMES: [&str; 138] = [
	"INVALID",		    // invalid opcode
	"LOAD.pri",		    // Load address into PRI.
	"LOAD.alt",		    // Load address into ALT.
	"LOAD.S.pri",		// Load stack offset into PRI.
	"LOAD.S.alt",		// Load stack offset into ALT.
	"LREF.pri",		    // Load address ref into PRI.
	"LREF.alt",		    // Load address ref into ALT.
	"LREF.S.pri",		// Load stack offset ref into PRI.
	"LREF.S.alt",		// Load stack offset ref into ALT.
	"LOAD.I",		    // PRI = [PRI]
	"LOBD.I",		    // PRI = [PRI + Param bytes]
	"CONST.pri",		// PRI = value TODO: Better const reading
	"CONST.alt",		// ALT = value
	"ADDR.pri",		    // PRI = FRM + offs
	"ADDR.alt",		    // ALT = FRM + offs
	"STOR.pri",		    // [ Param ] = PRI
	"STOR.alt",		    // [ Param ] = ALT
	"STOR.S.pri",		// [ Stack + Param ] = PRI
	"STOR.S.alt",		// [ Stack + Param ] = ALT
	"SREF.pri",		    // [ [ Param ] ] = PRI
	"SREF.alt",		    // [ [ Param ] ] = ALT
	"SREF.S.pri",		// [ [ Stack + Param ] ] = PRI
	"SREF.S.alt",		// [ [ Stack + Param ] ] = ALT
	"STOR.I",		    // [ ALT ] = PRI
	"STRB.I",		    // [ ALT ] = PRI (Param = number of bytes written)
	"LIDX",		        // PRI = [ ALT + (PRI * sizeof(cell)) ]
	"LIDX.B",		    // PRI = [ ALT + (PRI << param)]
	"IDXADDR",		    // PRI = ALT + (PRI * sizeof(cell))
	"IDXADDR.B",		// PRI = ALT + (PRI << param)
	"ALIGN.pri",		// PRI ^= cellsize - param
	"ALIGN.pri",		// ALT ^= cellsize - param
	"LCTRL",		    // PRI is set to special register value.
	"SCTRL",		    // the special register is set to PRI
	"MOVE.pri",		    // PRI = ALT
	"MOVE.alt",		    // ALT = PRI
	"XCHG",		        // Exchange alt and pri
	"PUSH.pri",		    // [STK] = PRI; STK -= sizeof(cell)
	"PUSH.alt",		    // [STK] = ALT; STK -= sizeof(cell)
	"PUSH.R",		    // obsolete
	"PUSH.C",		    // TODO: Better handling. [STK] = param; STK -= sizeof(cell)
	"PUSH",		        // [STK] = [PARAM]; STK -= sizeof(cell)
	"PUSH.S",		    // [STK] = [FRM + param]; STK -= sizeof(cell)
	"POP.pri",		    // STK += sizeof(cell) ; PRI = [STK]
	"POP.alt",		    // STK += sizeof(cell) ; ALT = [STK]
	"STACK",		    // ALT = STK; STK += param
	"HEAP",		        // ALT = HEA; HEA += param
	"PROC",		        // [STK] = FRM; STK -= sizeof(cell); FRM = [STK]
	"RET",		        // STK += cellsize; FRM = [STK]; STK += cellsize; CIP = [STK]
	"RETN",		        // STK += cellsize; FRM = [STK]; STK += cellsize; CIP = [STK]; STK += [STK]
	"CALL",		        // [STK] = CIP + 5; STK = STK - cellsize; CIP = param
	"CALL.pri",		    // [STK] = CIP + 1; STK -= cellsize; CIP = pri
	"JUMP",		        // CIP = param
	"JREL",		        // CIP += param
	"JZER",		        // if (PRI==0) CIP = [CIP + 1]
	"JNZ",		        // if (PRI!=0) CIP = [ CIP + 1 ]
	"JEQ",		        // if PRI==ALT CIP = [ CIP + 1 ]
	"JNEQ",		        // if PRI!=ALT CIP = [ CIP + 1 ]
	"JLESS",		    // if PRI<ALT CIP = [ CIP + 1 ]
	"JLEQ",		        // if PRI<=ALT CIP = [ CIP + 1 ]
	"JGRTR",		    // if PRI>ALT CIP = [ CIP + 1 ]
	"JGEQ",		        // if PRI>=ALT CIP = [ CIP + 1 ]
	"JSLESS",		    // if (SIGNED) PRI<ALT CIP = [ CIP + 1 ]
	"JSLEQ",		    // if SIGNED PRI<=ALT CIP = [ CIP + 1 ]
	"JSGRTR",		    // if SIGNED PRI>ALT CIP = [ CIP + 1 ]
	"JSGEQ",		    // if SIGNED PRI>=ALT CIP = [ CIP + 1 ]
	"SHL",		        // PRI = PRI << ALT
	"SHR",		        // PRI = PRI >> ALT
	"SSHR",		        // PRI = PRI >> ALT SIGNED
	"SHL.C.pri",		// PRI = PRI << param
	"SHL.C.alt",		// ALT = ALT << param
	"SHR.C.pri",		// PRI = PRI >> param
	"SHR.C.alt",		// ALT = ALT >> param
	"SMUL",		        // PRI *= ALT SIGNED
	"SDIV",		        // PRI = PRI / ALT SIGNED (ALT = PRI mod ALT)
	"SDIV.alt",		    // PRI = ALT / PRI SIGNED (ALT = PRI mod ALT)
	"UMUL",		        // PRI *= ALT UNSIGNED
	"UDIV",		        // PRI = PRI / ALT UNSIGNED (ALT = PRI mod ALT)
	"UDIV.alt",		    // PRI = ALT / PRI UNSIGNED (ALT = PRI mod ALT)
	"ADD",		        // PRI += ALT
	"SUB",		        // PRI -= ALT
	"SUB.alt",		    // PRI = ALT - PRI
	"AND",		        // PRI &= ALT
	"OR",		        // PRI |= ALT
	"XOR",		        // PRI ^= ALT
	"NOT",		        // PRI = !ALT
	"NEG",		        // PRI = -PRI
	"INVERT",		    // PRI = ~PRI
	"ADD.C",		    // PRI += param
	"SMUL.C",		    // PRI *= param
	"ZERO.pri",		    // PRI=0
	"ZERO.alt",		    // ALT=0
	"ZERO",		        // [ param ] = 0
	"ZERO.S",		    // [ FRM + param ] = 0
	"SIGN.pri",		    // sign extent the byte in PRI or ALT to a cell
	"SIGN.alt",		    // sign extent the byte in PRI or ALT to a cell
	"EQ",		        // PRI = PRI == ALT ? 1 : 0
	"NEQ",		        // PRI = PRI != ALT ? 1 : 0
	"LESS",		        // PRI = PRI < ALT ? 1 : 0
	"LEQ",		        // PRI = PRI <= ALT ? 1 : 0
	"GRTR",		        // PRI = PRI > ALT ? 1 : 0
	"GEQ",		        // PRI = PRI >= ALT ? 1 : 0
	"SLESS",		    // PRI = PRI < ALT ? 1 : 0
	"SLEQ",		        // PRI = PRI <= ALT ? 1 : 0
	"SGRTR",		    // PRI = PRI > ALT ? 1 : 0
	"SGEQ",		        // PRI = PRI >= ALT ? 1 : 0
	"EQ.C.pri",		    // PRI = PRI == param ? 1 : 0
	"EQ.C.alt",		    // PRI = ALT == param ? 1 : 0
	"INC.pri",		    // PRI++
	"INC.alt",		    // ALT++
	"INC",		        // [ param ] ++
	"INC.S",		    // [ FRM + param ] ++
	"INC.I",		    // [PRI]++
	"DEC.pri",		    // PRI--
	"DEC.alt",		    // ALT--
	"DEC",		        // [ param ] --
	"DEC.S",		    // [ FRM + param ] --
	"DEC.I",		    // [PRI]--
	"MOVS",		        // [ALT] = [PRI] (param is # of bytes)
	"CMPS",		        // compare [ALT] to [PRI] (param is # of bytes)
	"FILL",		        // Fill memory at [ALT] with value at [PRI], param is # of bytes
	"HALT",		        // Halt operation.
	"BOUNDS",		    // Aborts if PRI > param or PRI < 0
	"SYSREQ.pri",		// native, native id is in PRI
	"SYSREQ.C",		    // native, id is param.
	"OP_FILE",		    // obsolete | !WARNING! No fmt value for OP_FILE
	"OP_LINE",		    // obsolete | !WARNING! No fmt value for OP_LINE
	"OP_SYMBOL",		// obsolete | !WARNING! No fmt value for OP_SYMBOL
	"OP_SRANGE",		// obsolete | !WARNING! No fmt value for OP_SRANGE
	"JUMP.pri",		    // CIP = pri
	"SWITCH",		    // Compare PRI to the value of the passed casetbl, jump accordingly.
	"CASETBL",		    // TODO: Multiple params
	"SWAP.pri",		    // [STK] = PRI; PRI = old [STK]
	"SWAP.alt",		    // [STK] = ALT; ALT = old [STK]
	"PUSH.ADR",		    // [STK] = FRM + param; STK-=sizeofcell;
	"NOP",		        // No Operation
	"SYSREQ.D",
	"OP_SYMTAG",		// obsolete | !WARNING! No fmt value for OP_SYMTAG
	"BREAK",		    // Breakpoint
    // End of AMXX op codes
];

impl fmt::Display for OpcodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = *self as usize;

        if id > OPCODE_FMT_NAMES.len() {
            panic!("OPCODE \"{}\" FMT IS OUT OF BOUNDS", self);
        }

        let fmt_name = OPCODE_FMT_NAMES[id];
        write!(f, "{}", fmt_name)
    }
}

#[cfg(test)]
mod tests {
    use super::OpcodeType::*;

    #[test]
    fn has_fmt() {
        assert_eq!("LOAD.pri", format!("{}", OP_LOAD_PRI));
    }

    // FIXME: Handle panic
    // #[test]
    // #[should_panic]
    // fn fmt_panics_when_opcode_has_no_fmt_value() {
    //     println!("{}", OP_FAKE);
    // }
}
