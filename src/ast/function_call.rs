use super::TreeElement;

#[derive(Debug, Clone)]
pub enum Argument {
    String(String),
    Cell(u32),
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub args: Option<Vec<Argument>>,
}

impl TreeElement for FunctionCall {
    fn to_string(&self, ident: usize) -> Result<String, &'static str> {
        let mut source = String::new();

        // Push ident
        source.push_str(&format!("{:>width$}", "", width = (2 * ident)));

        let args: String = self.args
            .clone()
            .unwrap_or(vec![])
            .into_iter()
            .map(|arg| match arg {
                Argument::String(d) => format!("\"{}\"", d),
                _ => String::from("unknown argument type formatting"),
            })
            .collect::<Vec<String>>()
            .join(", ");

        source.push_str(&format!(
            "{native}({args});\n",
            native = self.name,
            args = args
        ));

        Ok(source)
    }
}
