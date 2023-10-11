use std::io::Read;

use base64::{engine::general_purpose, Engine};
use flate2::bufread::GzDecoder;

use crate::{
    acir_composer::AcirComposer,
    bindings::{
        circuit::circuit_size::get_circuit_sizes,
        srs::{netsrs::NetSrs, srs_init},
    },
};

const BYTECODE: &str = "H4sIAAAAAAAA/7WTMRLEIAhFMYkp9ywgGrHbq6yz5v5H2JkdCyaxC9LgWDw+H9gBwMM91p7fPeOzIKdYjEeMLYdGTB8MpUrCmOohJJQkfYMwN4mSSy0ZC0VudKbCZ4cthqzVrsc/yw28dMZeWmrWerfBexnsxD6hJ7jUufr4GvyZFp8xpG0C14Pd8s/q29vPCBXypvmpDx7sD8opnfqIfsM1RNtxBQAA";
const SOLVEDWITNESS: &str = "05000000000000000100000040000000000000003030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303302000000400000000000000030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303034030000004000000000000000333036343465373265313331613032396238353034356236383138313538356432383333653834383739623937303931343365316635393366303030303030300400000040000000000000003330363434653732653133316130323962383530343562363831383135383564323833336538343837396239373039313433653166353933663030303030303005000000400000000000000030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030";

#[test]
fn test_prove_verify() {
    let acir_buffer = general_purpose::STANDARD.decode(BYTECODE).unwrap();
    let mut decoder = GzDecoder::new(acir_buffer.as_slice());
    let mut acir_buffer_uncompressed = Vec::<u8>::new();
    decoder.read_to_end(&mut acir_buffer_uncompressed).unwrap();

    let circuit_size = get_circuit_sizes(&acir_buffer_uncompressed).unwrap();
    let log_value = (circuit_size.total as f64).log2().ceil() as u32;
    let subgroup_size = 2u32.pow(log_value);

    let srs = NetSrs::new(subgroup_size + 1);
    srs_init(&srs.data, srs.num_points, &srs.g2_data).unwrap();

    let acir_composer = AcirComposer::new(&subgroup_size).unwrap();
    let witness = hex::decode(SOLVEDWITNESS).unwrap();

    let proof = acir_composer
        .create_proof(&acir_buffer_uncompressed, &witness, false)
        .unwrap();
    let verdict = acir_composer.verify_proof(&proof, false).unwrap();
    assert!(verdict)
}
