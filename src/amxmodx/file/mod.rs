mod sections;
mod try_from_file;
mod try_from_vec_u8;

const MAGIC: u32 = 0x414d5858;
const COMPATIBLE_VERSION: u16 = 768;
const AMXX_HEADER_SIZE: usize = 7;

pub struct File {
    pub bin: Vec<u8>,
    pub sections: u8,
}
