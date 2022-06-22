//! All the traits exposed to be used in other custom pallets
use crate::types::vanchor::{ExtData, ProofData};
use codec::Encode;
use frame_support::dispatch;

pub trait VAnchorConfig {
	type LeafIndex;
	type AccountId: Encode;
	type Balance: Encode;
	type Amount: Encode;
	type CurrencyId;
	type ChainId;
	type TreeId;
	type Element: Encode;
}

/// Anchor trait definition to be used in other pallets
pub trait VAnchorInterface<C: VAnchorConfig> {
	// Creates a new anchor
	fn create(
		creator: Option<C::AccountId>,
		depth: u8,
		max_edges: u32,
		asset: C::CurrencyId,
	) -> Result<C::TreeId, dispatch::DispatchError>;
	/// Transaction
	fn transact(
		transactor: C::AccountId,
		id: C::TreeId,
		proof_data: ProofData<C::Element>,
		ext_data: ExtData<C::AccountId, C::Amount, C::Balance, C::Element>,
	) -> Result<(), dispatch::DispatchError>;
	// Stores nullifier hash from a spend tx
	fn add_nullifier_hash(
		id: C::TreeId,
		nullifier_hash: C::Element,
	) -> Result<(), dispatch::DispatchError>;
	/// Add an edge to this tree
	fn add_edge(
		id: C::TreeId,
		src_chain_id: C::ChainId,
		root: C::Element,
		latest_leaf_index: C::LeafIndex,
		target: C::Element,
	) -> Result<(), dispatch::DispatchError>;
	/// Update an edge for this tree
	fn update_edge(
		id: C::TreeId,
		src_chain_id: C::ChainId,
		root: C::Element,
		latest_leaf_index: C::LeafIndex,
		target: C::Element,
	) -> Result<(), dispatch::DispatchError>;

	fn set_max_deposit_amount(
		max_deposit_amount: C::Balance,
	) -> Result<(), dispatch::DispatchError>;

	fn set_min_withdraw_amount(
		min_withdraw_amount: C::Balance,
	) -> Result<(), dispatch::DispatchError>;
}

/// Anchor trait for inspecting tree state
pub trait VAnchorInspector<C: VAnchorConfig> {
	/// Check if a nullifier has been used in a tree or returns
	/// `InvalidNullifier`
	fn is_nullifier_used(id: C::TreeId, nullifier: C::Element) -> bool;
	/// Check if a nullifier has been used in a tree and throws if not
	fn ensure_nullifier_unused(
		id: C::TreeId,
		nullifier: C::Element,
	) -> Result<(), dispatch::DispatchError>;
	/// Check if this linked tree has this edge (for backwards compatability)
	fn has_edge(id: C::TreeId, src_chain_id: C::ChainId) -> bool;
}
