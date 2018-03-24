use std::fmt;
use super::super::amxmod::{Public, Opcode};
use super::TreeElement;

#[derive(PartialEq)]
pub enum FunctionVisibility {
    Public,
    Stock,
}

impl fmt::Display for FunctionVisibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if *self == FunctionVisibility::Public {
            write!(f, "public ")
        } else {
            write!(f, "")
        }
    }
}

pub struct Function {
    pub name: String,
    pub opcodes: Vec<Opcode>,
    pub visibility: FunctionVisibility,
}

impl Function {
    pub fn from(opcode: &Opcode, public_list: &[Public]) -> Function {
        static mut STOCK_FUNCTION_COUNTER: u32 = 0;
        let opcode_public = public_list.iter().find(|x| x.address == opcode.address);

        let visibility = if opcode_public.is_some() {
            FunctionVisibility::Public
        } else {
            FunctionVisibility::Stock
        };

        let name = if let Some(p) = opcode_public {
            String::from(format!("{}", p.name.to_str().unwrap()))
        } else {
            // I like to live dangerously
            unsafe {
                STOCK_FUNCTION_COUNTER += 1;
                String::from(format!("sub_{:x}", STOCK_FUNCTION_COUNTER - 1))
            }
        };

        Function {
            name: name,
            opcodes: vec![],
            visibility: visibility,
        }
    }
}

impl TreeElement for Function {
    fn to_string(&self) -> Result<String, &'static str> {
        let mut source = String::new();

        source.push_str(&format!(
            "{visibility}{fname} () {{\n",
            visibility = self.visibility,
            fname = self.name
        ));

        for opcode in self.opcodes.iter() {
            source.push_str(&format!("\t#emit {}", opcode.code));

            if let Some(p) = opcode.param {
                source.push_str(&format!("\t0x{:X}", p));
            }

            source.push('\n');
        }

        source.push_str("}\n\n");
        Ok(source)
    }
}
