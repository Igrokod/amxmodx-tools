use super::super::amxmod::Opcode;
use super::super::amxmod::OpcodeType::*;

use super::Function;

pub struct Plugin<'a> {
    functions: Vec<Function<'a>>,
}

impl<'a> Plugin<'a> {
    pub fn from_opcodes(_bin: &[u8], opcodes: &'a [Opcode]) -> Result<Plugin<'a>, &'static str> {
        let mut function_counter = 0;
        let mut functions: Vec<Function> = vec![];
        let mut stack: Vec<&Opcode> = vec![];

        for opcode in opcodes.iter() {
            if opcode.code == OP_PROC {
                let function_name = Plugin::name_for_function(function_counter);
                let function = Function {
                    name: function_name,
                    opcodes: vec![],
                };
                functions.push(function);
                function_counter += 1;
                continue;
            }

            if opcode.code == OP_BREAK || opcode.code == OP_RETN {
                let last_function = functions.last_mut().unwrap();
                // FIXME: Handle when no functions were given yet
                last_function.opcodes.extend(&stack);
                stack.clear();
                continue;
            }

            stack.push(&opcode);
        }

        let plugin = Plugin { functions: functions };
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
