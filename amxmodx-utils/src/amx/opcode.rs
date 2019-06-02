use super::opcode_type::OpcodeType;
use super::UCell;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Opcode {
    address: u32,
    code: OpcodeType,
    argument: Option<UCell>,
}

impl Opcode {
    pub fn new(code: OpcodeType, argument: Option<UCell>) -> Opcode {
        // FIXME: Save actual address
        let address = 0;
        Opcode {
            address,
            code,
            argument,
        }
    }

    // TODO: Test
    #[inline]
    pub fn code(&self) -> OpcodeType {
        self.code
    }

    #[inline]
    pub fn argument(&self) -> Option<UCell> {
        self.argument
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.argument {
            // Align + Pad as Cell
            Some(v) => write!(f, "{} 0x{:0>8X}", self.code, v),
            None => write!(f, "{}", self.code),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Opcode, OpcodeType};

    #[test]
    fn has_fmt_with_argument() {
        let opcode = Opcode::new(OpcodeType::OpPushPri, Some(1000));
        assert_eq!("push.pri 0x000003E8", format!("{}", opcode));
    }
}
