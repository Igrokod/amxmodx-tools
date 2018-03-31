use super::super::amxmod::Plugin as AmxPlugin;
use super::super::amxmod::Opcode;
use super::super::amxmod::OpcodeType::*;

use super::TreeElement;
use super::TreeElementType;
use super::TreeElementType::*;
use super::Function as AstFunction;

pub struct Plugin<'a> {
    tree_elements: Vec<TreeElementType>,
    amx_plugin: &'a AmxPlugin,
}

impl<'a> Plugin<'a> {
    pub fn from(amx_plugin: &'a AmxPlugin) -> Result<Plugin<'a>, &'static str> {
        let mut tree_elements: Vec<TreeElementType> = vec![];

        // FIXME: Error handling
        let opcodes = amx_plugin.opcodes().unwrap();
        for opcode in opcodes.into_iter() {
            tree_elements.push(OpcodeType(opcode));
        }

        let plugin = Plugin {
            tree_elements: tree_elements,
            amx_plugin: amx_plugin,
        };
        Ok(plugin)
    }

    pub fn opcodes_into_functions(&mut self) {
        trace!("Pack opcodes into functions");
        let public_list = self.amx_plugin.publics();

        let mut new_tree: Vec<TreeElementType> = vec![];
        let mut current_function: Option<AstFunction> = None;

        for element in self.tree_elements.clone().into_iter() {
            let opcode = match element {
                OpcodeType(o) => o,
                _ => {
                    new_tree.push(element);
                    continue;
                }
            };

            // Open function
            if opcode.code == OP_PROC {
                // TODO: Check if func already exist
                current_function = Some(AstFunction::from(&opcode, &public_list));
                continue;
            }

            // Close function
            if opcode.code == OP_RETN && current_function.is_some() {
                new_tree.push(FunctionType(current_function.unwrap()));
                current_function = None;
                continue;
            }

            // Accumulate function opcodes
            // should be the last before top level opcodes accumulation
            if let Some(mut f) = current_function.as_mut() {
                f.tree_elements.push(OpcodeType(opcode));
                continue;
            }

            new_tree.push(OpcodeType(opcode));
        }

        self.tree_elements = new_tree;
    }
}

// TODO: Plugin is not a tree element
impl<'a> TreeElement for Plugin<'a> {
    fn to_string(&self, ident: usize) -> Result<String, &'static str> {
        let mut source = String::from("// Plugin source approximation starts here\n\n");

        for tree_element in self.tree_elements.iter() {
            let element_str = &tree_element.to_string(ident + 1)?;
            source.push_str(&element_str);
        }

        Ok(source)
    }
}
