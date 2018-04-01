use super::super::amxmod::Opcode;

use super::TreeElement;
use super::TreeElementType;
use super::TreeElementType::*;

pub struct Plugin {
    pub tree_elements: Vec<TreeElementType>,
}

impl Plugin {
    pub fn from(opcodes: Vec<Opcode>) -> Result<Plugin, &'static str> {
        let mut tree_elements: Vec<TreeElementType> = vec![];

        for opcode in opcodes.into_iter() {
            tree_elements.push(OpcodeType(opcode));
        }

        Ok(Plugin { tree_elements: tree_elements })
    }
}

// TODO: Plugin is not a tree element
impl TreeElement for Plugin {
    fn to_string(&self, ident: usize) -> Result<String, &'static str> {
        let mut source = String::from("// Plugin source approximation starts here\n\n");

        for tree_element in self.tree_elements.iter() {
            let element_str = &tree_element.to_string(ident + 1)?;
            source.push_str(&element_str);
        }

        Ok(source)
    }
}
