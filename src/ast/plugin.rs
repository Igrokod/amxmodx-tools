use super::super::amxmod::Opcode;
use super::super::amxmod::OpcodeType::*;

use super::Function;

pub struct Plugin {
    functions: Vec<Function>,
}

impl Plugin {
    pub fn from_opcodes(bin: &[u8], opcodes: &[Opcode]) -> Result<Plugin, &'static str> {
        let mut function_counter = 0;
        let mut functions: Vec<Function> = vec![];

        for opcode in opcodes.iter() {
            if opcode.code == OP_PROC {
                let function_name = Plugin::name_for_function(function_counter);
                let function = Function { name: function_name };
                functions.push(function);
                function_counter += 1;
            }
        }

        let mut plugin = Plugin { functions: functions };
        Ok(plugin)
    }

    fn name_for_function(function_counter: u32) -> String {
        String::from(format!("sub_{:x}", function_counter))
    }

    pub fn to_string(&self) -> String {
        let mut source = String::from("// Plugin source approximation starts here\n\n");

        for function in self.functions.iter() {
            source.push_str(&function.to_string())
        }

        source
    }
}