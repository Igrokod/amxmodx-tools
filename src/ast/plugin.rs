use super::super::amxmod::Plugin as AmxPlugin;
use super::super::amxmod::Opcode;
use super::super::amxmod::OpcodeType::*;

use super::Function as AstFunction;

pub struct Plugin {
    functions: Vec<AstFunction>,
}

impl Plugin {
    pub fn from_amxmod_plugin(amx_plugin: &AmxPlugin) -> Result<Plugin, &'static str> {
        let public_list = amx_plugin.publics();
        // let native_list = amx_plugin.natives();

        let mut functions: Vec<AstFunction> = vec![];
        let mut stack: Vec<Opcode> = vec![];
        // FIXME: Error handling
        let opcodes = amx_plugin.opcodes().unwrap();

        for opcode in opcodes.into_iter() {
            if opcode.code == OP_PROC {
                let function = AstFunction::from(&opcode, &public_list);
                functions.push(function);
                continue;
            }

            if opcode.code == OP_BREAK || opcode.code == OP_RETN {
                let last_function = functions.last_mut().unwrap();
                // FIXME: Handle when no functions were given yet
                last_function.opcodes.extend(&stack);
                stack.clear();
                continue;
            }

            stack.push(opcode);
        }

        let plugin = Plugin { functions: functions };
        Ok(plugin)
    }

    pub fn to_string(&self) -> String {
        let mut source = String::from("// Plugin source approximation starts here\n\n");

        for function in self.functions.iter() {
            source.push_str(&function.to_string())
        }

        source
    }
}
