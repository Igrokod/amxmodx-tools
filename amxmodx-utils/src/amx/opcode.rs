use super::opcode_type::OpcodeType;
use super::Cell;

#[derive(Debug, Copy, Clone)]
pub struct Opcode {
    code: OpcodeType,
    argument: Option<Cell>,
}

impl Opcode {
    pub fn new(code: OpcodeType, argument: Option<Cell>) -> Opcode {
        Opcode { code, argument }
    }
}
