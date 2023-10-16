// Suppress the flurry of warnings caused by using "C" naming conventions
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod acir_composer;
pub mod acvm;
pub mod bindings;

// This matches bindgen::Builder output
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {}
