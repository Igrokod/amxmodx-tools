use super::TreeElement;
use crate::amxmod::plugin::ConstantParam;
use std::convert::From;
use std::ffi::CString;

#[derive(Debug, Clone)]
pub enum Argument {
    String(CString),
    Cell(u32),
}

impl From<ConstantParam> for Argument {
    fn from(constant: ConstantParam) -> Self {
        match constant {
            ConstantParam::Cell(v) => Argument::Cell(v),
            ConstantParam::String(v) => Argument::String(v),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub args: Option<Vec<Argument>>,
}

impl TreeElement for FunctionCall {
    fn to_string(&self, ident: usize) -> Result<String, &'static str> {
        let mut source = String::new();

        // Push ident
        source.push_str(&format!("{:>width$}", "", width = (2 * ident)));

        let args: String = self
            .args
            .clone()
            .unwrap_or_else(|| vec![])
            .into_iter()
            .map(|arg| match arg {
                Argument::String(s) => format!("{:?}", s),
                Argument::Cell(n) => format!("{}", n),
            })
            .collect::<Vec<String>>()
            .join(", ");

        source.push_str(&format!(
            "{native}({args});\n",
            native = self.name,
            args = args
        ));

        Ok(source)
    }
}
