use std::io::Read;

use acir_composer::AcirComposer;
use acvm::{
    acir::native_types::{Witness, WitnessMap},
    pwg::ACVM,
    FieldElement,
};
use acvm_runtime::circuit::deserialize_circuit;
use barretenberg::{
    circuit::circuit_size::get_circuit_sizes,
    srs::{netsrs::NetSrs, srs_init},
};
use base64::{engine::general_purpose, Engine};
use blackbox_solver::BlackboxSolver;
use flate2::read::GzDecoder;

const BYTECODE: &str = "H4sIAAAAAAAA/7WTMRLEIAhFMYkp9ywgGrHbq6yz5v5H2JkdCyaxC9LgWDw+H9gBwMM91p7fPeOzIKdYjEeMLYdGTB8MpUrCmOohJJQkfYMwN4mSSy0ZC0VudKbCZ4cthqzVrsc/yw28dMZeWmrWerfBexnsxD6hJ7jUufr4GvyZFp8xpG0C14Pd8s/q29vPCBXypvmpDx7sD8opnfqIfsM1RNtxBQAA";

pub fn main() {
    let circuit = deserialize_circuit(BYTECODE);
    let blackbox_solver = BlackboxSolver::new();
    let mut initial_witness = WitnessMap::new();
    initial_witness.insert(Witness(1), FieldElement::zero());
    initial_witness.insert(Witness(2), FieldElement::one());
    let mut solver = ACVM::new(&blackbox_solver, circuit.opcodes, initial_witness);
    let status = solver.solve();
    println!("{}", status);
    let finalize = solver.finalize();
    println!("{:?}", finalize);
    let serialized_solved_witness = bincode::serialize(&finalize).unwrap();

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
    let proof = acir_composer
        .create_proof(&acir_buffer_uncompressed, &serialized_solved_witness, false)
        .unwrap();
    let verdict = acir_composer.verify_proof(&proof, false).unwrap();
    println!("{}", verdict);
}
