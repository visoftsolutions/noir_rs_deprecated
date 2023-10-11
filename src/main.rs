use std::ops::Shl;

use crate::bindings::{
    examples::simple_create_and_verify_proof,
    srs::{netsrs::NetSrs, srs_init},
};

pub mod acir_composer;
pub mod bindings;

// This matches bindgen::Builder output
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    println!("Hello, world!");

    let srs = NetSrs::new(1_u32.shl(19) + 1_u32);
    srs_init(&srs.data, srs.num_points, &srs.g2_data).unwrap();

    println!("{}", simple_create_and_verify_proof().unwrap());
}
