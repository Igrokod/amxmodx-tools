use super::super::amxmod::Plugin as AmxPlugin;
use super::super::amxmod::Opcode;
use super::super::amxmod::OpcodeType::*;

use super::TreeElement;
use super::Function as AstFunction;

pub struct Plugin {
    tree_elements: Vec<Box<TreeElement>>,
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
                // FIXME: Handle when no functions were given yet
                let last_function = functions.last_mut().unwrap();
                last_function.opcodes.extend(&stack);
                stack.clear();
                continue;
            }

            stack.push(opcode);
        }

        let mut tree_elements: Vec<Box<TreeElement>> = vec![];
        for f in functions.into_iter() {
            tree_elements.push(Box::new(f));
        }

        let plugin = Plugin { tree_elements: tree_elements };
        Ok(plugin)
    }
}

impl TreeElement for Plugin {
    fn to_string(&self) -> Result<String, &'static str> {
        let mut source = String::from("// Plugin source approximation starts here\n\n");

        for tree_element in self.tree_elements.iter() {
            let element_str = &tree_element.to_string()?;
            source.push_str(&element_str);
        }

        Ok(source)
    }
}
