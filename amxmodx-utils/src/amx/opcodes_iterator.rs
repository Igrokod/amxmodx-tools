use super::opcode::Opcode;
use super::opcode_type::{OpcodeType, SINGLE_PARAM_OPCODES};
use byteorder::{LittleEndian, ReadBytesExt};
use failure::Fail;
use num_traits::FromPrimitive;
use std::io::Cursor;

#[derive(Debug, Fail)]
pub enum ParseError {
    #[fail(display = "Invalid opcode code: {}", _0)]
    InvalidOpcodeCode(u32),
    #[fail(display = "Unexpected end of cod, missing opcode argument")]
    MissingOpcodeArgument,
}

#[derive(Debug)]
pub struct OpcodesIterator<'amx_bin> {
    stop_iteration: bool,
    cod_reader: Cursor<&'amx_bin [u8]>,
}

impl<'amx_bin> OpcodesIterator<'amx_bin> {
    pub fn new(cod_bin: &'amx_bin [u8]) -> OpcodesIterator<'amx_bin> {
        let stop_iteration = false;
        let cod_reader = Cursor::new(cod_bin);

        OpcodesIterator {
            stop_iteration,
            cod_reader,
        }
    }
}

impl<'amx_bin> Iterator for OpcodesIterator<'amx_bin> {
    type Item = Result<Opcode, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop_iteration {
            return None;
        }

        let opcode_code = self.cod_reader.read_u32::<LittleEndian>().ok()?;
        let opcode_type = match OpcodeType::from_u32(opcode_code) {
            Some(opcode_type) => opcode_type,
            None => {
                self.stop_iteration = true;
                // TODO: test
                return Some(Err(ParseError::InvalidOpcodeCode(opcode_code)));
            }
        };

        let has_arguments = SINGLE_PARAM_OPCODES.iter().any(|e| e == &opcode_type);

        let opcode_argument = if has_arguments {
            match self.cod_reader.read_u32::<LittleEndian>().ok() {
                Some(value) => Some(value),
                None => {
                    self.stop_iteration = true;
                    // TODO: Test
                    return Some(Err(ParseError::MissingOpcodeArgument));
                }
            }
        } else {
            None
        };

        Some(Ok(Opcode::new(opcode_type, opcode_argument)))
    }
}

#[cfg(test)]
mod tests {
    use super::{Opcode, ParseError};
    use crate::amx::File as AmxFile;
    use std::convert::TryFrom;
    use std::fs::File;
    use std::io::{self, Read};

    fn _read_file(path: &str) -> io::Result<Vec<u8>> {
        let mut file = File::open(path)?;
        let mut plugin = vec![];
        file.read_to_end(&mut plugin)?;

        Ok(plugin)
    }

    fn read_file(path: &str) -> Vec<u8> {
        _read_file(path).expect(&format!("Could not read {} file", path))
    }

    #[test]
    fn it_iterates_opcodes_on_correct_file() {
        let amx_bin = read_file("test/fixtures/amxx/simple.cellsize4.amx183");
        let amx_file = AmxFile::try_from(&amx_bin[..]).unwrap();
        let mut opcodes_iterator = amx_file.opcodes().expect("Should return opcodes iterator");
        let opcodes: Vec<Result<Opcode, ParseError>> = opcodes_iterator.collect();

        // TODO: Test parsing correctness
    }

    // TODO: Failing test cases
}
