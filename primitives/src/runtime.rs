use frame_support::sp_runtime::{
	generic,
	traits::{BlakeTwo256, IdentifyAccount, Verify},
	MultiSignature,
};

/// An index to a block.
pub type BlockNumber = u64;

/// Alias to 512-bit hash when used in the context of a transaction
/// signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make
/// it equivalent to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of
/// them, but you never know...
pub type AccountIndex = u32;

pub type AssetId = u32;

/// Balance of an account.
pub type Balance = u128;

pub type Amount = i128;

/// Index of a transaction in the chain.
pub type Index = u32;

pub type ChainId = u64;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Digest item type.
pub type DigestItem = generic::DigestItem;

/// Opaque types. These are used by the CLI to instantiate machinery that
/// don't need to know the specifics of the runtime. They can then be made
/// to be agnostic over specific formats of data like extrinsics, allowing
/// for them to continue syncing the network through upgrades to even the
/// core data structures.
pub mod opaque {
	use super::*;

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
}
