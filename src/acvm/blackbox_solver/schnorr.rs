use super::{BlackboxSolver, Error};
use crate::bindings::schnorr;

pub(crate) trait SchnorrSig {
    fn construct_signature(
        &self,
        message: &[u8],
        private_key: [u8; 32],
    ) -> Result<([u8; 32], [u8; 32]), Error>;
    fn construct_public_key(&self, private_key: [u8; 32]) -> Result<[u8; 64], Error>;
    fn verify_signature(
        &self,
        pub_key: [u8; 64],
        sig_s: [u8; 32],
        sig_e: [u8; 32],
        message: &[u8],
    ) -> Result<bool, Error>;
}

impl SchnorrSig for BlackboxSolver {
    fn construct_signature(
        &self,
        message: &[u8],
        private_key: [u8; 32],
    ) -> Result<([u8; 32], [u8; 32]), Error> {
        let (sig_s, sig_e) =
            unsafe { schnorr::construct_signature(message, &private_key) }.unwrap();
        Ok((sig_s, sig_e))
    }

    #[allow(dead_code)]
    fn construct_public_key(&self, private_key: [u8; 32]) -> Result<[u8; 64], Error> {
        let pubkey = schnorr::compute_public_key(&private_key).unwrap();
        Ok(pubkey)
    }

    fn verify_signature(
        &self,
        pub_key: [u8; 64],
        sig_s: [u8; 32],
        sig_e: [u8; 32],
        message: &[u8],
    ) -> Result<bool, Error> {
        let verified =
            unsafe { schnorr::verify_signature(message, pub_key, sig_s, sig_e) }.unwrap();

        // Note, currently for Barretenberg plonk, if the signature fails
        // then the whole circuit fails.
        Ok(verified)
    }
}
