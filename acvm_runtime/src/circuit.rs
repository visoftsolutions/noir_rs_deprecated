use acvm::acir::circuit::Circuit;
use base64::Engine;
pub fn deserialize_circuit(bytecode_b64: &str) -> Circuit {
    let circuit_bytes = base64::engine::general_purpose::STANDARD
        .decode(bytecode_b64)
        .unwrap();
    Circuit::read(&*circuit_bytes).unwrap()
}
