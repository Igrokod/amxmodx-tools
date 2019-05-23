pub mod parser;

#[derive(Debug, PartialEq)]
pub struct File {
    bin: Vec<u8>,
    flags: Flags,
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

bitflags! {
    pub struct Flags: u16 {
        // const AMX_FLAG_CHAR16 = 0x01; // no longer used
        const DEBUG = 0x02; // symbolic info available
        const COMPACT = 0x04; // compact encoding
        const BYTEOPC = 0x08; // opcode is a byte (not a cell)
        const NOCHECKS = 0x10; // no array bounds checking; no STMT opcode
        const NTVREG = 0x1000; // all native functions are registered
        const JITC = 0x2000; // abstract machine is JIT compiled
        const BROWSE = 0x4000; // busy browsing
        const RELOC = 0x8000; // jump/call addresses relocated
    }
}
