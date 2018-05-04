use std::fs::File;
use std::io::prelude::*;

pub fn load_fixture(filename: &str) -> Vec<u8> {
    let mut file_bin: Vec<u8> = Vec::new();
    let mut file = File::open(format!("test/fixtures/{}", filename)).unwrap();
    file.read_to_end(&mut file_bin).unwrap();
    file_bin
}
