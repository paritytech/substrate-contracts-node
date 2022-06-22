use ark_crypto_primitives::Error;
use frame_support::pallet_prelude::DispatchError;
use sp_std::vec::Vec;

// A trait meant to be implemented over a hash function instance
pub trait InstanceHasher {
	fn hash(data: &[u8], params: &[u8]) -> Result<Vec<u8>, Error>;
}

// A trait meant to be implemented by a pallet
pub trait HasherModule {
	/// hash arbitrary slice
	fn hash(data: &[u8]) -> Result<Vec<u8>, DispatchError>;
	/// hash two elements
	fn hash_two(left: &[u8], right: &[u8]) -> Result<Vec<u8>, DispatchError>;
}
