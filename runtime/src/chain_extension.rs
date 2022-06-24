use crate::Randomness;
use codec::{Decode, Encode};
use frame_support::log::{debug, error, trace};
use pallet_contracts::{
	chain_extension::{
		ChainExtension, Environment, Ext, InitState, RetVal, SysConfig, UncheckedFrom,
	},
	Config,
};
use sp_runtime::DispatchError;
use webb_primitives::verifier::*;
use sp_std::vec::Vec;

/// Contract extension for `FetchRandom`
pub struct VerifyProofExtension;

impl<C: Config> ChainExtension<C> for VerifyProofExtension {
	fn call<E: Ext>(func_id: u32, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
	where
		<E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
	{
		match func_id {
			1101 => {
				let mut env = env.buf_in_buf_out();
				//let arg: [u8; 32] = env.read_as()?;
				let (arg1, arg2): (Vec<u8>, Vec<u8>) =
					env.read_as_unbounded(env.in_len())?;

				debug!(target: "runtime", "arg1 value is: {:?}", arg1);
				debug!(target: "runtime", "arg2 value is: {:?}", arg2);
				//let random_seed = crate::RandomnessCollectiveFlip::random(&arg).0;
				//let random_slice = random_seed.encode();
				let random_slice = true.encode();
				trace!(
					target: "runtime",
					"[ChainExtension]|call|func_id:{:}",
					func_id
				);
				env.write(&random_slice, false, None)
					.map_err(|_| DispatchError::Other("ChainExtension failed to call random"))?;
			},

			1102 => {
				let mut env = env.buf_in_buf_out();
				let mut buf = Vec::new();
				env.read_into(&mut &mut*buf)?;
				debug!(target: "runtime", "buffer is: {:?}", buf);
				let address = env.ext().address(); // contract
				debug!(target: "runtime", "contract address: {:?}", address);
				//let (pub_inp, pub_proof): (Vec<u8>, Vec<u8>) = codec::Decode::decode(&mut &*buf).unwrap();
				let (public_inputs, proof_input): (Vec<u8>, Vec<u8>) =
					env.read_as_unbounded(env.in_len())?;
				//let in_len = env.in_len(); // Number of passed parameters
				//debug!(target: "runtime", "in_len: {}", inputs);
				//let public_inputs = env.read(1)?; // Read n incoming parameters
				debug!(target: "runtime", "public input with len: {:?}", public_inputs);
				//let proof = env.read(2)?; // Read n incoming parameters
				debug!(target: "runtime", "proof input with len: {:?}", &proof_input);
				let result = crate::MixerVerifierBn254::verify(&public_inputs, &proof_input);
				debug!(target: "runtime", "result of verification is: {:?}", result.unwrap());
				let result_slice = result.unwrap().encode();
				trace!(
					target: "runtime",
					"[ChainExtension]|call|func_id:{:}",
					func_id
				);
				env.write(&result_slice, false, None)
					.map_err(|_| DispatchError::Other("ChainExtension failed to call verify"))?;
			},

			_ => {
				error!("Called an unregistered `func_id`: {:}", func_id);
				return Err(DispatchError::Other("Unimplemented func_id"))
			},
		}
		Ok(RetVal::Converging(0))
	}

	fn enabled() -> bool {
		true
	}
}
