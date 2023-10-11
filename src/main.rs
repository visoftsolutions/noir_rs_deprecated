use crate::bindings::examples::simple_create_and_verify_proof;

pub mod bindings;

// This matches bindgen::Builder output
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    println!("Hello, world!");
    println!("{}", simple_create_and_verify_proof().unwrap());
}
