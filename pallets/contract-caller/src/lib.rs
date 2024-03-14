//! # Contract Caller
//!
//! todo: [AJ] docs

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	type AccountIdOf<Runtime> = <Runtime as frame_system::Config>::AccountId;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_contracts::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {
		NoOp, // ContractCallError(pallet_contracts::Error<T>),
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Call the flip method on the contract at the given `contract` account.
		#[pallet::call_index(0)]
		#[pallet::weight(<T::WeightInfo as pallet_contracts::WeightInfo>::call().saturating_add(*gas_limit))]
		pub fn contract_call_flip(
			origin: OriginFor<T>,
			contract: AccountIdOf<T>,
			gas_limit: Weight,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// todo: call the contract..

			Ok(())
		}
	}
}
