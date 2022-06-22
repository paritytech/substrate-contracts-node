// This file is part of Webb.

// Copyright (C) 2021 Webb Technologies Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Verifier Module
//!
//! A simple module for abstracting over arbitrary zero-knowledge verifiers
//! for arbitrary zero-knowledge gadgets. This pallet should store verifying
//! keys and any other verification specific parameters for different backends
//! that we support in Webb's ecosystem of runtime modules.
//!
//! ## Overview
//!
//! The Verifier module provides functionality for zero-knowledge verifier
//! management including:
//!
//! * Setting parameters for zero-knowledge verifier
//! * Setting the maintainer of the parameters
//!
//! To use it in your runtime, you need to implement the verifier [`Config`].
//! Additionally, you will want to implement the verifier traits defined in the
//! webb_primitives::verifier module.
//!
//! The supported dispatchable functions are documented in the [`Call`] enum.
//!
//! ### Terminology
//!
//! ### Goals
//!
//! The verifier system in Webb is designed to make the following possible:
//!
//! * Define.
//!
//! ## Interface
//!
//! ## Related Modules
//!
//! * [`System`](../frame_system/index.html)
//! * [`Support`](../frame_support/index.html)

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;

use frame_support::pallet_prelude::{ensure, DispatchError};
use webb_primitives::verifier::*;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use core::convert::TryInto;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T, I = ()>(_);

	#[pallet::config]
	/// The module configuration trait.
	pub trait Config<I: 'static = ()>: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self, I>> + IsType<<Self as frame_system::Config>::Event>;

		/// The verifier instance trait
		type Verifier: InstanceVerifier;

		/// The origin which may forcibly reset parameters or otherwise alter
		/// privileged attributes.
		type ForceOrigin: EnsureOrigin<Self::Origin>;
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config<I>, I: 'static = ()> {
		pub phantom: (PhantomData<T>, PhantomData<I>),
		pub parameters: Option<Vec<u8>>,
	}

	#[cfg(feature = "std")]
	impl<T: Config<I>, I: 'static> Default for GenesisConfig<T, I> {
		fn default() -> Self {
			Self { phantom: Default::default(), parameters: None }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config<I>, I: 'static> GenesisBuild<T, I> for GenesisConfig<T, I> {
		fn build(&self) {
			if let Some(params) = &self.parameters {
				Parameters::<T, I>::put(params);
			}
		}
	}

	#[pallet::storage]
	#[pallet::getter(fn parameters)]
	/// Details of the module's parameters
	pub(super) type Parameters<T: Config<I>, I: 'static = ()> =
		StorageValue<_, Vec<u8>, ValueQuery>;

	#[pallet::event]
	pub enum Event<T: Config<I>, I: 'static = ()> {}

	#[pallet::error]
	pub enum Error<T, I = ()> {
		/// Parameters haven't been initialized
		ParametersNotInitialized,
		/// Error during verification
		VerifyError,
	}

	#[pallet::hooks]
	impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {}

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		#[pallet::weight(50_000_000)]
		pub fn force_set_parameters(
			origin: OriginFor<T>,
			parameters: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			T::ForceOrigin::ensure_origin(origin)?;
			Parameters::<T, I>::try_mutate(|params| {
				*params = parameters.clone();
				Ok(().into())
			})
		}
	}
}

impl<T: Config<I>, I: 'static> VerifierModule for Pallet<T, I> {
	fn verify(public_inp_bytes: &[u8], proof: &[u8]) -> Result<bool, DispatchError> {
		let params = Self::parameters();
		ensure!(!params.is_empty(), Error::<T, I>::ParametersNotInitialized);
		match T::Verifier::verify(public_inp_bytes, proof, &params) {
			Ok(verified) => Ok(verified),
			Err(e) => {
				log::info!("{:?}", e);
				// println!("{:?}", e);
				// TODO: Handle properly
				ensure!(false, Error::<T, I>::VerifyError);
				Ok(false)
			},
		}
	}
}
