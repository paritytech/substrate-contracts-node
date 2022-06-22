use ark_bls12_381::Fr as Bls381;
use ark_bn254::Fr as Bn254;
use ark_ff::{BigInteger, PrimeField};
use sp_std::{marker::PhantomData, vec::Vec};

pub trait IntoPrimeField<T> {
	fn into_field(value: T) -> Vec<u8>;
}

pub struct ArkworksIntoField<F: PrimeField>(PhantomData<F>);

impl<F: PrimeField> IntoPrimeField<i128> for ArkworksIntoField<F> {
	fn into_field(value: i128) -> Vec<u8> {
		let mut f = F::from(value.unsigned_abs());
		if value.is_negative() {
			f = -f;
		}
		f.into_repr().to_bytes_le()
	}
}

pub type ArkworksIntoFieldBn254 = ArkworksIntoField<Bn254>;
pub type ArkworksIntoFieldBls381 = ArkworksIntoField<Bls381>;
