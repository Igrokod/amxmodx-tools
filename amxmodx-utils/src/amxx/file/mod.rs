mod error;
mod parser;

pub use error::ParseError;

#[derive(Debug)]
pub struct File {
    sections_count: u8,
}

#[cfg(test)]
mod tests {}
