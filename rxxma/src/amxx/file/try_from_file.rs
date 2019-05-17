use std::convert::TryFrom;
use std::fs::File as IoFile;
use std::io::Read;
use std::path::PathBuf;

use failure::Error;

use super::File;

impl TryFrom<PathBuf> for File {
    type Error = Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let mut open_result = IoFile::open(path)?;
        let mut file_contents: Vec<u8> = Vec::new();
        open_result.read_to_end(&mut file_contents)?;

        Self::try_from(file_contents).map_err(|e| format_err!("{}", e))
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use std::path::PathBuf;

    use super::File as AmxmodxFile;

    #[test]
    fn it_try_from_file() {
        let path = PathBuf::from("test/fixtures/simple.amxx183");
        assert!(AmxmodxFile::try_from(path).is_ok());

        let path = PathBuf::from("test/fixtures/unexistent");
        assert!(AmxmodxFile::try_from(path).is_err());
    }
}
