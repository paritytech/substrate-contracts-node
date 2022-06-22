use crate::Randomness;
use codec::Encode;
use frame_support::log::{debug, error, trace};
use pallet_contracts::{
	chain_extension::{
		ChainExtension, Environment, Ext, InitState, RetVal, SysConfig, UncheckedFrom,
	},
	Config,
};
use sp_runtime::DispatchError;
use webb_primitives::verifier::*;

/// Contract extension for `FetchRandom`
pub struct FetchRandomExtension;

impl<C: Config> ChainExtension<C> for FetchRandomExtension {
	fn call<E: Ext>(func_id: u32, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
	where
		<E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
	{
		match func_id {
			1101 => {
				let mut env = env.buf_in_buf_out();
				let arg: [u8; 32] = env.read_as()?;
				crate::MixerVerifierBn254::verify(&arg, &arg);
				let random_seed = crate::RandomnessCollectiveFlip::random(&arg).0;
				let random_slice = random_seed.encode();
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
				let in_len = env.in_len(); // Number of passed parameters
				debug!(target: "runtime", "in_len: {}", in_len);
				let public_inputs = env.read(1)?; // Read n incoming parameters
				debug!(target: "runtime", "input with len: {}", public_inputs[0]);
				let proof = env.read(2)?; // Read n incoming parameters
				debug!(target: "runtime", "input with len: {}", proof[0]);
				let result = crate::MixerVerifierBn254::verify(&public_inputs, &proof);
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
