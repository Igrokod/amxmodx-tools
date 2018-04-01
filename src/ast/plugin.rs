use super::super::amxmod::Plugin as AmxPlugin;
use super::super::amxmod::OpcodeType::*;

use super::TreeElement;
use super::TreeElementType;
use super::TreeElementType::*;
use super::Function as AstFunction;

pub struct Plugin<'a> {
    pub tree_elements: Vec<TreeElementType>,
    pub amx_plugin: &'a AmxPlugin,
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
