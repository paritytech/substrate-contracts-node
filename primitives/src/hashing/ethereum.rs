use crate::hasher::InstanceHasher;
use ark_crypto_primitives::Error;
use ark_ff::{BigInteger, PrimeField};
pub use sp_io::hashing::keccak_256;
use sp_std::{marker::PhantomData, vec::Vec};

pub struct Keccak256Hasher<F: PrimeField>(PhantomData<F>);

impl<F: PrimeField> InstanceHasher for Keccak256Hasher<F> {
	fn hash(data: &[u8], _: &[u8]) -> Result<Vec<u8>, Error> {
		let res = keccak_256(data);
		let field_res = F::from_le_bytes_mod_order(&res);
		let value = field_res.into_repr().to_bytes_le();
		Ok(value)
	}
}

use ark_bn254::Fr as Bn254;
pub type Keccak256HasherBn254 = Keccak256Hasher<Bn254>;

use ark_bls12_381::Fr as Bls381;
pub type Keccak256HasherBls381 = Keccak256Hasher<Bls381>;
