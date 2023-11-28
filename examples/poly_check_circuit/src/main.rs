use std::fs;
use serde_json::Value;
use noir_rs::{native_types::{Witness, WitnessMap}, FieldElement};

static PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    let data = fs::read_to_string(format!("target/{}.json", PACKAGE_NAME)).expect("Unable to read file");
    let json: Value = serde_json::from_str(&data).expect("Unable to parse JSON");
    let bytecode: &str = json["bytecode"].as_str().expect("Unable to extract bytecode");

    let mut initial_witness = WitnessMap::new();
    initial_witness.insert(Witness(1), FieldElement::from(47_i128));
    initial_witness.insert(Witness(2), FieldElement::from(2_i128));
    let (proof, vk) = noir_rs::prove(String::from(bytecode), initial_witness).unwrap();
    let verdict = noir_rs::verify(String::from(bytecode), proof, vk).unwrap();
    assert!(verdict);
}
