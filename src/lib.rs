#![cfg_attr(feature = "strict", deny(warnings))]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate failure;

pub mod amxmod;
pub mod amxmodx;
pub mod ast;
pub mod util;
