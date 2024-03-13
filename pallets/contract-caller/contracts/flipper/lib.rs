#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod flipper {
	use contract_traits::Flip;

	/// Defines the storage of your contract.
	/// Add new fields to the below struct in order
	/// to add new static storage fields to your contract.
	#[ink(storage)]
	pub struct Flipper {
		/// Stores a single `bool` value on the storage.
		value: bool,
	}

	impl Flipper {
		/// Constructor that initializes the `bool` value to the given `init_value`.
		#[ink(constructor)]
		pub fn new(init_value: bool) -> Self {
			Self { value: init_value }
		}
	}

	impl Flip for Flipper {
		/// Flip the value of the stored `bool` from `true`
		/// to `false` and vice versa.
		#[ink(message)]
		fn flip(&mut self) {
			self.value = !self.value;
		}

		/// Returns the current value of our `bool`.
		#[ink(message)]
		fn get(&self) -> bool {
			self.value
		}
	}
}

#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests;
