pub trait TreeElement {
    fn to_string(&self) -> Result<String, &'static str>;
}
