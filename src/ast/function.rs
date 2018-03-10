pub struct Function {
    pub name: String,
}

impl Function {
    pub fn to_string(&self) -> String {
        String::from(format!(
            "public {fname}() {{\n\t// No code yet\n}}",
            fname = self.name
        ))
    }
}
