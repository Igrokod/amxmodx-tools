use super::super::amxmod::OpcodeType::*;

use super::TreeElementType;
use super::TreeElementType::*;
use super::Function as AstFunction;
use super::Plugin as AstPlugin;

pub struct Decompiler<'a> {
    pub inner: AstPlugin<'a>,
}

impl<'a> Decompiler<'a> {
    pub fn from(ast_plugin: AstPlugin<'a>) -> Decompiler<'a> {
        Decompiler { inner: ast_plugin }
    }

    pub fn into_inner(self) -> AstPlugin<'a> {
        self.inner
    }

    pub fn opcodes_into_functions(&mut self) {
        trace!("Pack opcodes into functions");
        let public_list = self.inner.amx_plugin.publics();

        let mut new_tree: Vec<TreeElementType> = vec![];
        let mut current_function: Option<AstFunction> = None;

        for element in self.inner.tree_elements.clone().into_iter() {
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

        self.inner.tree_elements = new_tree;
    }
}
