use crate::bindings::acir::{
    create_proof, delete, get_solidity_verifier, get_verification_key, init_proving_key,
    init_verification_key, load_verification_key, new_acir_composer, serialize_proof_into_fields,
    serialize_verification_key_into_fields, verify_proof, AcirComposerPtr,
};

pub struct AcirComposer {
    composer_ptr: AcirComposerPtr,
}

impl AcirComposer {
    /// Creates a new ACIR composer.
    /// # Arguments
    /// * `size_hint` - Hint for the size of the composer.
    /// # Returns
    /// * `Result<AcirComposer, String>` - Returns an AcirComposer instance or an error message.
    pub fn new_acir_composer(size_hint: &u32) -> Result<Self, String> {
        new_acir_composer(size_hint).map(|ptr| Self { composer_ptr: ptr })
    }

    /// Initializes the proving key for the given composer.
    /// # Arguments
    /// * `constraint_system_buf` - Buffer representing the constraint system.
    /// # Returns
    /// * `Result<(), String>` - Returns an empty result or an error message.
    pub fn init_proving_key(&self, constraint_system_buf: &[u8]) -> Result<(), String> {
        init_proving_key(&self.composer_ptr, constraint_system_buf)
    }

    /// Creates a proof using the provided constraint system buffer and witness.
    /// # Arguments
    /// * `constraint_system_buf` - Buffer representing the constraint system.
    /// * `witness` - Buffer representing the witness.
    /// * `is_recursive` - Boolean indicating whether the proof is recursive.
    /// # Returns
    /// * `Result<Vec<u8>, String>` - Returns the created proof or an error message.
    pub fn create_proof(
        &self,
        constraint_system_buf: &[u8],
        witness: &[u8],
        is_recursive: bool,
    ) -> Result<Vec<u8>, String> {
        create_proof(
            &self.composer_ptr,
            constraint_system_buf,
            witness,
            is_recursive,
        )
    }

    /// Loads the verification key into the given composer.
    /// # Arguments
    /// * `verification_key` - Buffer representing the verification key.
    /// # Returns
    /// * `Result<(), String>` - Returns an empty result or an error message.
    pub fn load_verification_key(&self, verification_key: &[u8]) -> Result<(), String> {
        load_verification_key(&self.composer_ptr, verification_key)
    }

    /// Initializes the ACIR composer's verification key.
    /// # Returns
    /// * `Result<(), String>` - Returns an empty result or an error message if there's an issue with the initialization.
    pub fn init_verification_key(&self) -> Result<(), String> {
        init_verification_key(&self.composer_ptr)
    }

    /// Retrieves the verification key from the ACIR composer.
    /// # Arguments
    /// * `acir_composer` - Pointer to the ACIR composer.
    /// # Returns
    /// * `Result<Vec<u8>, String>` - Returns the verification key or an error message.
    pub fn get_verification_key(&self) -> Result<Vec<u8>, String> {
        get_verification_key(&self.composer_ptr)
    }

    /// Verifies the proof with the ACIR composer.
    /// # Arguments
    /// * `proof` - Buffer representing the proof.
    /// * `is_recursive` - Boolean indicating whether the proof is recursive.
    /// # Returns
    /// * `Result<bool, String>` - Returns `true` if the verification succeeds, `false` otherwise, or an error message.
    pub fn verify_proof(&self, proof: &[u8], is_recursive: bool) -> Result<bool, String> {
        verify_proof(&self.composer_ptr, proof, is_recursive)
    }

    /// Gets the Solidity verifier string representation from the ACIR composer.
    /// # Returns
    /// * `Result<String, String>` - Returns the Solidity verifier string or an error message.
    pub fn get_solidity_verifier(&self) -> Result<String, String> {
        get_solidity_verifier(&self.composer_ptr)
    }

    /// Serializes the provided proof into fields.
    /// # Arguments
    /// * `proof` - Buffer representing the proof.
    /// * `num_inner_public_inputs` - Number of inner public inputs.
    /// # Returns
    /// * `Result<Vec<u8>, String>` - Returns the serialized proof or an error message.
    pub fn serialize_proof_into_fields(
        &self,
        proof: &[u8],
        num_inner_public_inputs: u32,
    ) -> Result<Vec<u8>, String> {
        serialize_proof_into_fields(&self.composer_ptr, proof, num_inner_public_inputs)
    }

    /// Serializes the verification key into field elements.
    /// # Arguments
    /// * `acir_composer` - Pointer to the ACIR composer.
    /// # Returns
    /// * `Result<(Vec<u8>, Vec<u8>), String>` - Returns serialized verification key and its hash, or an error message.
    pub fn serialize_verification_key_into_fields(&self) -> Result<(Vec<u8>, Vec<u8>), String> {
        serialize_verification_key_into_fields(&self.composer_ptr)
    }
}

impl Drop for AcirComposer {
    fn drop(&mut self) {
        if let Err(e) = delete(self.composer_ptr) {
            eprintln!("Error when dropping AcirComposer: {}", e);
        }
    }
}
