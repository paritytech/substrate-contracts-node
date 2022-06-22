//! All the traits exposed to be used in other custom pallets
use frame_support::dispatch;

/// Tree trait definition to be used in other pallets
pub trait TreeInterface<AccountId, TreeId, Element> {
	// Creates a new tree
	fn create(creator: Option<AccountId>, depth: u8) -> Result<TreeId, dispatch::DispatchError>;
	/// Adds members/leaves to the tree
	fn insert_in_order(id: TreeId, leaf: Element) -> Result<Element, dispatch::DispatchError>;
	/// This returns the byte of zero root default hash
	fn zero_root(i: u8) -> Result<[u8; 32], dispatch::DispatchError>;
}

/// Tree trait for inspecting tree state
pub trait TreeInspector<AccountId, TreeId, Element> {
	/// Gets the merkle root for a tree or returns `TreeDoesntExist`
	fn get_root(id: TreeId) -> Result<Element, dispatch::DispatchError>;
	/// Checks if a merkle root is in a tree's cached history or returns
	/// `TreeDoesntExist
	fn is_known_root(id: TreeId, target: Element) -> Result<bool, dispatch::DispatchError>;
}
