use super::{File, Flags};
use bytes::Buf;
use failure::Fail;
use std::convert::TryFrom;
use std::io::Cursor;
use std::mem::size_of;

const MAGIC: u16 = 0xF1E0;
const FILE_VERSION: u8 = 8;
const AMX_VERSION: u8 = 8;

#[derive(Debug, Fail)]
pub enum ParseError {
    #[fail(display = "Header is corrupted")]
    HeaderEOF,
    #[fail(display = "Amx magic mismatch, expected: 0x{:X}, got: 0x{:X}", _0, _1)]
    MagicMismatch(u16, u16),
    #[fail(display = "File version mismatch, expected: {}, got: {}", _0, _1)]
    FileVersionMismatch(u8, u8),
    #[fail(display = "Amx version mismatch, expected: {}, got: {}", _0, _1)]
    AmxVersionMismatch(u8, u8),
    #[fail(
        display = "Unexpected bit value for amx flags (contains unknown flags) {}",
        _0
    )]
    UnexpectedAmxFlags(u16),
}

// Struct used to calculate fields size for reading
#[repr(C)]
struct RawAmxHeader {
    size: u32,
    magic: u16,
    file_version: u8,
    amx_version: u8,
    flags: u16,
    defsize: u16,
    cod: u32,
    dat: u32,
    hea: u32,
    stp: u32,
    cip: u32,
    publics: u32,
    natives: u32,
    libraries: u32,
    pubvars: u32,
    tags: u32,
    nametable: u32,
}

impl TryFrom<&[u8]> for File {
    type Error = ParseError;

    fn try_from(bin: &[u8]) -> Result<Self, Self::Error> {
        // TODO: Test
        let header_bin = bin
            .get(0..size_of::<RawAmxHeader>())
            .ok_or_else(|| ParseError::HeaderEOF)?;

        let mut header_reader = Cursor::new(header_bin);

        let _size = header_reader.get_u32_le();
        let magic = header_reader.get_u16_le();
        let file_version = header_reader.get_u8();
        let amx_version = header_reader.get_u8();
        let flags = header_reader.get_u16_le();
        let defsize = header_reader.get_u16_le();
        let cod = header_reader.get_u32_le();
        let dat = header_reader.get_u32_le();
        let hea = header_reader.get_u32_le();
        let stp = header_reader.get_u32_le();
        let cip = header_reader.get_u32_le();
        let publics = header_reader.get_u32_le();
        let natives = header_reader.get_u32_le();
        let libraries = header_reader.get_u32_le();
        let pubvars = header_reader.get_u32_le();
        let tags = header_reader.get_u32_le();
        let nametable = header_reader.get_u32_le();

        // TODO: Test
        if magic != MAGIC {
            return Err(ParseError::MagicMismatch(MAGIC, magic));
        }

        // TODO: Test
        if file_version != FILE_VERSION {
            return Err(ParseError::FileVersionMismatch(FILE_VERSION, file_version));
        }

        // TODO: Test
        if amx_version != AMX_VERSION {
            return Err(ParseError::AmxVersionMismatch(AMX_VERSION, amx_version));
        }

        // TODO: Test
        let flags = Flags::from_bits(flags).ok_or_else(|| ParseError::UnexpectedAmxFlags(flags))?;

        let bin = bin.to_owned();

        Ok(File {
            bin,
            flags,
            defsize,
            cod,
            dat,
            hea,
            stp,
            cip,
            publics,
            natives,
            libraries,
            pubvars,
            tags,
            nametable,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{File as AmxFile, Flags};
    use std::convert::TryFrom;
    use std::fs::File as IoFile;
    use std::io::{self, Read};

    fn _read_file(path: &str) -> io::Result<Vec<u8>> {
        let mut file = IoFile::open(path)?;
        let mut plugin = vec![];
        file.read_to_end(&mut plugin)?;

        Ok(plugin)
    }

    fn read_file(path: &str) -> Vec<u8> {
        _read_file(path).expect(&format!("Could not read {} file", path))
    }

    #[test]
    fn it_parses_amx_file() {
        let unpacked_bin = read_file("test/fixtures/amxx/simple.cellsize4.amx183");
        let file = AmxFile::try_from(&unpacked_bin[..]).expect("Plugin should be correctly parsed");

        match file {
            AmxFile {
                bin: unpacked_bin,
                flags: Flags::DEBUG,
                defsize: 8,
                cod: 116,
                dat: 192,
                hea: 296,
                stp: 16680,
                cip: 4294967295, // FIXME: Is that correct?
                publics: 56,
                natives: 64,
                libraries: 72,
                pubvars: 72,
                tags: 72,
                nametable: 80,
            } => (),
            _ => panic!("Amxx file parsed invalid"),
        }
    }
}
