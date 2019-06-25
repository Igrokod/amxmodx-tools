use super::function::Function;
use super::function_call::FunctionCall;
use amxmodx_utils::amx::opcode::Opcode;

#[derive(Debug, Clone)]
pub enum TreeElementType {
    OpcodeType(Opcode),
    FunctionType(Function),
    FunctionCallType(FunctionCall),
}

pub trait TreeElement {
    fn to_string(&self, ident: usize) -> Result<String, &'static str>;
}

impl TreeElement for Opcode {
    fn to_string(&self, ident: usize) -> Result<String, &'static str> {
        let mut source = String::new();
        source.push_str(&format!(
            "{:>width$}#emit {}",
            "",
            self.code(),
            width = (2 * ident)
        ));

        if let Some(p) = self.argument() {
            source.push_str(&format!("\t0x{:X}", p));
        }

        source.push('\n');
        Ok(source)
    }
}

impl TreeElement for TreeElementType {
    fn to_string(&self, ident: usize) -> Result<String, &'static str> {
        match *self {
            TreeElementType::OpcodeType(o) => TreeElement::to_string(&o, ident),
            TreeElementType::FunctionType(ref f) => f.to_string(ident),
            TreeElementType::FunctionCallType(ref c) => c.to_string(ident),
        }
    }
}
