#![cfg_attr(feature = "strict", deny(warnings))]

extern crate byteorder;
extern crate flate2;
#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;
pub mod util;
pub mod amxmodx;
pub mod amxmod;
pub mod ast;
