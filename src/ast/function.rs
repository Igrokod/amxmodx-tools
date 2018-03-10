use super::super::amxmod::Opcode;

pub struct Function<'a> {
    pub name: String,
    pub opcodes: Vec<&'a Opcode>,
}

impl<'a> Function<'a> {
    pub fn to_string(&self) -> String {
        let mut source = String::new();

        source.push_str(&format!("public {fname} () {{\n", fname = self.name));

        for opcode in self.opcodes.iter() {
            source.push_str(&format!("\t#emit {}", opcode.code));

            if let Some(p) = opcode.param {
                source.push_str(&format!("\t0x{:X}", p));
            }

            source.push('\n');
        }

        source.push_str("}");
        source
    }
}
