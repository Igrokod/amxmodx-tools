use super::super::amxmod::Opcode as AmxOpcode;
use super::super::amxmod::OpcodeType;
use super::TreeElement;

pub struct Opcode {
    inner: AmxOpcode,
}

impl Opcode {
    pub fn from(opcode: AmxOpcode) -> Opcode {
        Opcode { inner: opcode }
    }

    pub fn address(&self) -> usize {
        self.inner.address
    }

    pub fn code(&self) -> OpcodeType {
        self.inner.code
    }

    pub fn param(&self) -> Option<u32> {
        self.inner.param
    }
}

impl TreeElement for Opcode {
    fn to_string(&self) -> Result<String, &'static str> {
        let mut source = String::new();
        source.push_str(&format!("\t#emit {}", self.code()));

        if let Some(p) = self.param() {
            source.push_str(&format!("\t0x{:X}", p));
        }

        source.push('\n');
        Ok(source)
    }
}
