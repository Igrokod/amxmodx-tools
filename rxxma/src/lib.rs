#![cfg_attr(feature = "strict", deny(warnings))]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate failure;

pub mod amx;
pub mod amxx;
pub mod ast;
pub mod util;
