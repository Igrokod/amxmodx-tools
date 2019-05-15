mod sections;
mod try_from_file;
mod try_from_vec_u8;

// TODO: `core::num::<impl u32>::from_be_bytes` is not yet stable as a const fn
// const MAGIC: u32 = u32::from_be_bytes(*b"XXMA");
#[allow(clippy::unreadable_literal)]
const MAGIC: u32 = 0x414d5858;
const COMPATIBLE_VERSION: u16 = 768;
const AMXX_HEADER_SIZE: usize = 7;

#[derive(Debug)]
pub struct File {
    pub bin: Vec<u8>,
    pub sections: u8,
}
