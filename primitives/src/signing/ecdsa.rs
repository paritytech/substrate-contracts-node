pub use sp_io::{hashing::keccak_256, EcdsaVerifyError};
use sp_std::vec::Vec;

pub const SIGNATURE_LENGTH: usize = 65;

pub fn validate_ecdsa_signature(data: &[u8], signature: &[u8]) -> bool {
	if signature.len() == SIGNATURE_LENGTH {
		let mut sig = [0u8; SIGNATURE_LENGTH];
		sig[..SIGNATURE_LENGTH].copy_from_slice(&signature);

		let hash = keccak_256(&data);

		return sp_io::crypto::secp256k1_ecdsa_recover(&sig, &hash).is_ok()
	} else {
		return false
	}
}

pub fn recover_ecdsa_pub_key(data: &[u8], signature: &[u8]) -> Result<Vec<u8>, EcdsaVerifyError> {
	if signature.len() == SIGNATURE_LENGTH {
		let mut sig = [0u8; SIGNATURE_LENGTH];
		sig[..SIGNATURE_LENGTH].copy_from_slice(&signature);

		let hash = keccak_256(&data);
		let pub_key = sp_io::crypto::secp256k1_ecdsa_recover(&sig, &hash)?;
		return Ok(pub_key.to_vec())
	}
	Err(EcdsaVerifyError::BadSignature)
}
