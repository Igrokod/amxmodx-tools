use super::super::amxmod::Opcode;
use super::function::Function;

#[derive(Debug, Clone)]
pub enum TreeElementType {
    OpcodeType(Opcode),
    FunctionType(Function),
}

pub trait TreeElement {
    fn to_string(&self) -> Result<String, &'static str>;
}

impl TreeElement for Opcode {
    fn to_string(&self) -> Result<String, &'static str> {
        let mut source = String::new();
        source.push_str(&format!("#emit {}", self.code));

        if let Some(p) = self.param {
            source.push_str(&format!("\t0x{:X}", p));
        }

        source.push('\n');
        Ok(source)
    }
}

impl TreeElement for TreeElementType {
    fn to_string(&self) -> Result<String, &'static str> {
        match *self {
            TreeElementType::OpcodeType(o) => o.to_string(),
            TreeElementType::FunctionType(ref f) => f.to_string(),
            _ => Ok(String::from("/* unknown tree element, internal error */")),
        }
    }
}
