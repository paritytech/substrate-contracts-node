//! All the traits exposed to be used in other custom pallets
use frame_support::dispatch;
use sp_std::vec::Vec;

pub trait AnchorConfig {
	type LeafIndex;
	type AccountId;
	type Balance;
	type CurrencyId;
	type ChainId;
	type TreeId;
	type Element;
}

/// Anchor trait definition to be used in other pallets
pub trait AnchorInterface<C: AnchorConfig> {
	// Creates a new anchor
	fn create(
		creator: Option<C::AccountId>,
		deposit_size: C::Balance,
		depth: u8,
		max_edges: u32,
		asset: C::CurrencyId,
	) -> Result<C::TreeId, dispatch::DispatchError>;
	/// Deposit into the anchor
	fn deposit(
		account: C::AccountId,
		id: C::TreeId,
		leaf: C::Element,
	) -> Result<(), dispatch::DispatchError>;
	/// Withdraw from the anchor
	#[allow(clippy::too_many_arguments)]
	fn withdraw(
		id: C::TreeId,
		proof_bytes: &[u8],
		roots: Vec<C::Element>,
		nullifier_hash: C::Element,
		recipient: C::AccountId,
		relayer: C::AccountId,
		fee: C::Balance,
		refund: C::Balance,
		commitment: C::Element,
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
}

/// Anchor trait for inspecting tree state
pub trait AnchorInspector<C: AnchorConfig> {
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
	/// Compute the updated chain id type for this chain
	fn get_chain_id_type() -> C::ChainId;
	fn get_chain_type() -> [u8; 2];
}
