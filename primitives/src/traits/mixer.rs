//! All the traits exposed to be used in other custom pallets
use frame_support::dispatch;

/// Mixer trait definition to be used in other pallets
pub trait MixerInterface<AccountId, Balance, CurrencyId, TreeId, Element> {
	// Creates a new mixer
	fn create(
		creator: Option<AccountId>,
		deposit_size: Balance,
		depth: u8,
		asset: CurrencyId,
	) -> Result<TreeId, dispatch::DispatchError>;
	/// Deposit into the mixer
	fn deposit(
		account: AccountId,
		id: TreeId,
		leaf: Element,
	) -> Result<(), dispatch::DispatchError>;
	/// Withdraw from the mixer
	#[allow(clippy::too_many_arguments)]
	fn withdraw(
		id: TreeId,
		proof_bytes: &[u8],
		root: Element,
		nullifier_hash: Element,
		recipient: AccountId,
		relayer: AccountId,
		fee: Balance,
		refund: Balance,
	) -> Result<(), dispatch::DispatchError>;
	// Stores nullifier hash from a spend tx
	fn add_nullifier_hash(
		id: TreeId,
		nullifier_hash: Element,
	) -> Result<(), dispatch::DispatchError>;
}

/// Mixer trait for inspecting mixer state
pub trait MixerInspector<AccountId, CurrencyId, TreeId, Element> {
	/// Gets the merkle root for a tree or returns `TreeDoesntExist`
	fn get_root(id: TreeId) -> Result<Element, dispatch::DispatchError>;
	/// Checks if a merkle root is in a tree's cached history or returns
	/// `TreeDoesntExist
	fn is_known_root(id: TreeId, target: Element) -> Result<bool, dispatch::DispatchError>;

	fn ensure_known_root(id: TreeId, target: Element) -> Result<(), dispatch::DispatchError>;
	/// Check if a nullifier has been used in a tree or returns
	/// `InvalidNullifier`
	fn is_nullifier_used(id: TreeId, nullifier: Element) -> bool;

	fn ensure_nullifier_unused(
		id: TreeId,
		nullifier: Element,
	) -> Result<(), dispatch::DispatchError>;
}
