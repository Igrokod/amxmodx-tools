use num_derive::FromPrimitive;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
pub enum OpcodeType {
    OpNone, // invalid opcode
    OpLoadPri,
    OpLoadAlt,
    OpLoadSPri,
    OpLoadSAlt,
    OpLrefPri,
    OpLrefAlt,
    OpLrefSPri,
    OpLrefSAlt,
    OpLoadI,
    OpLodbI,
    OpConstPri,
    OpConstAlt,
    OpAddrPri,
    OpAddrAlt,
    OpStorPri,
    OpStorAlt,
    OpStorSPri,
    OpStorSAlt,
    OpSrefPri,
    OpSrefAlt,
    OpSrefSPri,
    OpSrefSAlt,
    OpStorI,
    OpStrbI,
    OpLidx,
    OpLidxB,
    OpIdxaddr,
    OpIdxaddrB,
    OpAlignPri,
    OpAlignAlt,
    OpLctrl,
    OpSctrl,
    OpMovePri,
    OpMoveAlt,
    OpXchg,
    OpPushPri,
    OpPushAlt,
    OpPushR,
    OpPushC,
    OpPush,
    OpPushS,
    OpPopPri,
    OpPopAlt,
    OpStack,
    OpHeap,
    OpProc,
    OpRet,
    OpRetn,
    OpCall,
    OpCallPri,
    OpJump,
    OpJrel,
    OpJzer,
    OpJnz,
    OpJeq,
    OpJneq,
    OpJless,
    OpJleq,
    OpJgrtr,
    OpJgeq,
    OpJsless,
    OpJsleq,
    OpJsgrtr,
    OpJsgeq,
    OpShl,
    OpShr,
    OpSshr,
    OpShlCPri,
    OpShlCAlt,
    OpShrCPri,
    OpShrCAlt,
    OpSmul,
    OpSdiv,
    OpSdivAlt,
    OpUmul,
    OpUdiv,
    OpUdivAlt,
    OpAdd,
    OpSub,
    OpSubAlt,
    OpAnd,
    OpOr,
    OpXor,
    OpNot,
    OpNeg,
    OpInvert,
    OpAddC,
    OpSmulC,
    OpZeroPri,
    OpZeroAlt,
    OpZero,
    OpZeroS,
    OpSignPri,
    OpSignAlt,
    OpEq,
    OpNeq,
    OpLess,
    OpLeq,
    OpGrtr,
    OpGeq,
    OpSless,
    OpSleq,
    OpSgrtr,
    OpSgeq,
    OpEqCPri,
    OpEqCAlt,
    OpIncPri,
    OpIncAlt,
    OpInc,
    OpIncS,
    OpIncI,
    OpDecPri,
    OpDecAlt,
    OpDec,
    OpDecS,
    OpDecI,
    OpMovs,
    OpCmps,
    OpFill,
    OpHalt,
    OpBounds,
    OpSysreqPri,
    OpSysreqC,
    OpFile,   // obsolete
    OpLine,   // obsolete
    OpSymbol, // obsolete
    OpSrange, // obsolete
    OpJumpPri,
    OpSwitch,
    OpCasetbl,
    OpSwapPri,
    OpSwapAlt,
    OpPushaddr,
    OpNop,
    OpSysreqD,
    OpSymtag, // obsolete
    OpBreak,  // End of AMXX op codes
}

use self::OpcodeType::*;

pub const SINGLE_PARAM_OPCODES: &[OpcodeType] = &[
    OpLoadPri, OpLoadAlt, OpLoadSPri, OpLoadSAlt, OpLrefPri, OpLrefAlt, OpLrefSPri, OpLrefSAlt,
    OpLodbI, OpConstPri, OpConstAlt, OpAddrPri, OpAddrAlt, OpStorPri, OpStorAlt, OpStorSPri,
    OpStorSAlt, OpSrefPri, OpSrefAlt, OpSrefSPri, OpSrefSAlt, OpStrbI, OpLidxB, OpIdxaddrB,
    OpAlignPri, OpAlignAlt, OpLctrl, OpSctrl, OpPushR, OpPushC, OpPush, OpPushS, OpStack, OpHeap,
    OpCall, OpJump, OpJrel, OpJzer, OpJnz, OpJeq, OpJneq, OpJless, OpJleq, OpJgrtr, OpJgeq,
    OpJsless, OpJsleq, OpJsgrtr, OpJsgeq, OpShl, OpShr, OpSshr, OpShlCPri, OpShlCAlt, OpShrCPri,
    OpShrCAlt, OpAddC, OpSmulC, OpZero, OpZeroS, OpEqCPri, OpEqCAlt, OpInc, OpIncS, OpDec, OpDecS,
    OpMovs, OpCmps, OpFill, OpHalt, OpBounds, OpSysreqC, OpSwitch, OpPushaddr,
];

const OPCODE_FMT_NAMES: &[&str] = &[
    "invalid",    // invalid opcode
    "load.pri",   // Load address into PRI.
    "load.alt",   // Load address into ALT.
    "load.s.pri", // Load stack offset into PRI.
    "load.s.alt", // Load stack offset into ALT.
    "lref.pri",   // Load address ref into PRI.
    "lref.alt",   // Load address ref into ALT.
    "lref.s.pri", // Load stack offset ref into PRI.
    "lref.s.alt", // Load stack offset ref into ALT.
    "load.i",     // PRI = [PRI]
    "lobd.i",     // PRI = [PRI + Param bytes]
    "const.pri",  // PRI = value
    "const.alt",  // ALT = value
    "addr.pri",   // PRI = FRM + offs
    "addr.alt",   // ALT = FRM + offs
    "stor.pri",   // [ Param ] = PRI
    "stor.alt",   // [ Param ] = ALT
    "stor.s.pri", // [ Stack + Param ] = PRI
    "stor.s.alt", // [ Stack + Param ] = ALT
    "sref.pri",   // [ [ Param ] ] = PRI
    "sref.alt",   // [ [ Param ] ] = ALT
    "sref.s.pri", // [ [ Stack + Param ] ] = PRI
    "sref.s.alt", // [ [ Stack + Param ] ] = ALT
    "stor.i",     // [ ALT ] = PRI
    "strb.i",     // [ ALT ] = PRI (Param = number of bytes written)
    "lidx",       // PRI = [ ALT + (PRI * sizeof(cell)) ]
    "lidx.b",     // PRI = [ ALT + (PRI << param)]
    "idxaddr",    // PRI = ALT + (PRI * sizeof(cell))
    "idxaddr.b",  // PRI = ALT + (PRI << param)
    "align.pri",  // PRI ^= cellsize - param
    "align.pri",  // ALT ^= cellsize - param
    "lctrl",      // PRI is set to special register value.
    "sctrl",      // the special register is set to PRI
    "move.pri",   // PRI = ALT
    "move.alt",   // ALT = PRI
    "xchg",       // Exchange alt and pri
    "push.pri",   // [STK] = PRI; STK -= sizeof(cell)
    "push.alt",   // [STK] = ALT; STK -= sizeof(cell)
    "push.r",     // obsolete
    "push.c",     // [STK] = param; STK -= sizeof(cell)
    "push",       // [STK] = [PARAM]; STK -= sizeof(cell)
    "push.s",     // [STK] = [FRM + param]; STK -= sizeof(cell)
    "pop.pri",    // STK += sizeof(cell) ; PRI = [STK]
    "pop.alt",    // STK += sizeof(cell) ; ALT = [STK]
    "stack",      // ALT = STK; STK += param
    "heap",       // ALT = HEA; HEA += param
    "proc",       // [STK] = FRM; STK -= sizeof(cell); FRM = [STK]
    "ret",        // STK += cellsize; FRM = [STK]; STK += cellsize; CIP = [STK]
    "retn",       // STK += cellsize; FRM = [STK]; STK += cellsize; CIP = [STK]; STK += [STK]
    "call",       // [STK] = CIP + 5; STK = STK - cellsize; CIP = param
    "call.pri",   // [STK] = CIP + 1; STK -= cellsize; CIP = pri
    "jump",       // CIP = param
    "jrel",       // CIP += param
    "jzer",       // if (PRI==0) CIP = [CIP + 1]
    "jnz",        // if (PRI!=0) CIP = [ CIP + 1 ]
    "jeq",        // if PRI==ALT CIP = [ CIP + 1 ]
    "jneq",       // if PRI!=ALT CIP = [ CIP + 1 ]
    "jless",      // if PRI<ALT CIP = [ CIP + 1 ]
    "jleq",       // if PRI<=ALT CIP = [ CIP + 1 ]
    "jgrtr",      // if PRI>ALT CIP = [ CIP + 1 ]
    "jgeq",       // if PRI>=ALT CIP = [ CIP + 1 ]
    "jsless",     // if (SIGNED) PRI<ALT CIP = [ CIP + 1 ]
    "jsleq",      // if SIGNED PRI<=ALT CIP = [ CIP + 1 ]
    "jsgrtr",     // if SIGNED PRI>ALT CIP = [ CIP + 1 ]
    "jsgeq",      // if SIGNED PRI>=ALT CIP = [ CIP + 1 ]
    "shl",        // PRI = PRI << ALT
    "shr",        // PRI = PRI >> ALT
    "sshr",       // PRI = PRI >> ALT SIGNED
    "shl.c.pri",  // PRI = PRI << param
    "shl.c.alt",  // ALT = ALT << param
    "shr.c.pri",  // PRI = PRI >> param
    "shr.c.alt",  // ALT = ALT >> param
    "smul",       // PRI *= ALT SIGNED
    "sdiv",       // PRI = PRI / ALT SIGNED (ALT = PRI mod ALT)
    "sdiv.alt",   // PRI = ALT / PRI SIGNED (ALT = PRI mod ALT)
    "umul",       // PRI *= ALT UNSIGNED
    "udiv",       // PRI = PRI / ALT UNSIGNED (ALT = PRI mod ALT)
    "udiv.alt",   // PRI = ALT / PRI UNSIGNED (ALT = PRI mod ALT)
    "add",        // PRI += ALT
    "sub",        // PRI -= ALT
    "sub.alt",    // PRI = ALT - PRI
    "and",        // PRI &= ALT
    "or",         // PRI |= ALT
    "xor",        // PRI ^= ALT
    "not",        // PRI = !ALT
    "neg",        // PRI = -PRI
    "invert",     // PRI = ~PRI
    "add.c",      // PRI += param
    "smul.c",     // PRI *= param
    "zero.pri",   // PRI=0
    "zero.alt",   // ALT=0
    "zero",       // [ param ] = 0
    "zero.s",     // [ FRM + param ] = 0
    "sign.pri",   // sign extent the byte in PRI or ALT to a cell
    "sign.alt",   // sign extent the byte in PRI or ALT to a cell
    "eq",         // PRI = PRI == ALT ? 1 : 0
    "neq",        // PRI = PRI != ALT ? 1 : 0
    "less",       // PRI = PRI < ALT ? 1 : 0
    "leq",        // PRI = PRI <= ALT ? 1 : 0
    "grtr",       // PRI = PRI > ALT ? 1 : 0
    "geq",        // PRI = PRI >= ALT ? 1 : 0
    "sless",      // PRI = PRI < ALT ? 1 : 0
    "sleq",       // PRI = PRI <= ALT ? 1 : 0
    "sgrtr",      // PRI = PRI > ALT ? 1 : 0
    "sgeq",       // PRI = PRI >= ALT ? 1 : 0
    "eq.c.pri",   // PRI = PRI == param ? 1 : 0
    "eq.c.alt",   // PRI = ALT == param ? 1 : 0
    "inc.pri",    // PRI++
    "inc.alt",    // ALT++
    "inc",        // [ param ] ++
    "inc.s",      // [ FRM + param ] ++
    "inc.i",      // [PRI]++
    "dec.pri",    // PRI--
    "dec.alt",    // ALT--
    "dec",        // [ param ] --
    "dec.s",      // [ FRM + param ] --
    "dec.i",      // [PRI]--
    "movs",       // [ALT] = [PRI] (param is # of bytes)
    "cmps",       // compare [ALT] to [PRI] (param is # of bytes)
    "fill",       // Fill memory at [ALT] with value at [PRI], param is # of bytes
    "halt",       // Halt operation.
    "bounds",     // Aborts if PRI > param or PRI < 0
    "sysreq.pri", // native, native id is in PRI
    "sysreq.c",   // native, id is param.
    "file",       // obsolete
    "line",       // obsolete
    "symbol",     // obsolete
    "srange",     // obsolete
    "jump.pri",   // CIP = pri
    "switch",     // Compare PRI to the value of the passed casetbl, jump accordingly.
    "casetbl",    // Table of case values
    "swap.pri",   // [STK] = PRI; PRI = old [STK]
    "swap.alt",   // [STK] = ALT; ALT = old [STK]
    "push.adr",   // [STK] = FRM + param; STK -= size_of_cell;
    "nop",        // No Operation
    "sysreq.d",   // TODO: check in amxx sources
    "symtag",     // obsolete
    "break",      // Breakpoint
];

impl fmt::Display for OpcodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = *self as usize;
        // TODO: Return original name when out of bounds
        let fmt_name = OPCODE_FMT_NAMES[id];
        write!(f, "{}", fmt_name)
    }
}

#[cfg(test)]
mod tests {
    use super::OpcodeType::*;

    #[test]
    fn has_fmt() {
        assert_eq!("load.pri", format!("{}", OpLoadPri));
    }
}
