use codec::{Decode, Encode};
use scale_info::prelude::fmt::Debug;
use sp_runtime::traits::AtLeast32Bit;
use sp_std::vec::Vec;
use webb_proposals::{ResourceId, TargetSystem, TypedChainId};

/// Takes an (ideally u32) ChainIdentifier and a chain type and re-computes
/// an updated chain id with the chain type prepended to it. The resulting
/// chain id is 6 bytes long and so requires a u64 to represent it.
///
/// ```rust
/// use webb_primitives::utils::compute_chain_id_type;
/// pub type ChainId = u64;
/// let chain_id: u32 = 5;
/// let chain_type: [u8; 2] = [2, 0];
///
/// let chain_id_type: ChainId = compute_chain_id_type::<ChainId>(chain_id.into(), chain_type);
/// ```
pub fn compute_chain_id_type<ChainId>(chain_id: ChainId, chain_type: [u8; 2]) -> u64
where
	ChainId: AtLeast32Bit,
{
	let mut chain_id_value: u32 = chain_id.try_into().unwrap_or_default();
	let mut buf = [0u8; 8];
	buf[2..4].copy_from_slice(&chain_type);
	buf[4..8].copy_from_slice(&chain_id_value.to_be_bytes());
	u64::from_be_bytes(buf)
}
/// Truncate and pad 256 bit slice
pub fn truncate_and_pad(t: &[u8]) -> Vec<u8> {
	let mut truncated_bytes = t[..20].to_vec();
	truncated_bytes.extend_from_slice(&[0u8; 12]);
	truncated_bytes
}

pub fn element_encoder(v: &[u8]) -> [u8; 32] {
	let mut output = [0u8; 32];
	output.iter_mut().zip(v).for_each(|(b1, b2)| *b1 = *b2);
	output
}

/// gets the chain id and tree id to derive resource id
pub fn derive_resource_id(chain_id: u32, tree_id: u32) -> ResourceId {
	let target_system = TargetSystem::TreeId(tree_id);
	let typed_chain_id = TypedChainId::Substrate(chain_id);

	let resource_id = webb_proposals::ResourceId::new(target_system, typed_chain_id);
	resource_id
}

/// Gets the resource id and parses it to tree id and chain id
pub fn parse_resource_id<TreeId, ChainId>(
	resource_id: webb_proposals::ResourceId,
) -> (TreeId, ChainId)
where
	TreeId: Encode + Decode + AtLeast32Bit + Default + Copy + Debug,
	ChainId: Encode + Decode + AtLeast32Bit + Default + Copy + Debug,
{
	let typed_chain_id = resource_id.typed_chain_id();

	let tree_id_u32: u32 = match resource_id.target_system() {
		TargetSystem::TreeId(tree_id) => tree_id,
		_ => 0,
	};
	let tree_id = TreeId::try_from(tree_id_u32).unwrap_or_default();

	// returns the underlying chain id
	// the underlying chain id will be used to get the actual chain id(Substrate)
	let chain_id = ChainId::try_from(typed_chain_id.underlying_chain_id()).unwrap_or_default();

	(tree_id, chain_id)
}

/// takes an input of the underlying chain id and
/// gets the actual chain id(Substrate) which is in u64
pub fn get_typed_chain_id<ChainId>(chain_id: ChainId) -> ChainId
where
	ChainId: Encode + Decode + AtLeast32Bit + Default + Copy + Debug,
{
	let typed_chain_id =
		TypedChainId::Substrate(chain_id.try_into().unwrap_or_default()).chain_id();

	ChainId::try_from(typed_chain_id).unwrap_or_default()
}

/// takes an input of the underlying chain id(u32) and
/// gets the actual chain id(Substrate) which is in u64
pub fn get_typed_chain_id_in_u64(chain_id: u32) -> u64 {
	let typed_chain_id = TypedChainId::Substrate(chain_id).chain_id();

	typed_chain_id
}

/// gets the underlying chain id(u32) from the
/// actual chain id(Substrate) which is in u64
pub fn get_underlying_chain_id(typed_chain_id: u64) -> u32 {
	let bytes = typed_chain_id.to_be_bytes();
	let mut underlying_chain_id_bytes = [0u8; 4];
	underlying_chain_id_bytes.copy_from_slice(&bytes[4..]);
	let underlying_chain_id = u32::from_be_bytes(underlying_chain_id_bytes);

	underlying_chain_id
}

#[cfg(test)]
mod tests {
	use super::*;
	type TreeId = u32;
	type ChainId = u64;

	#[test]
	fn derive_parse_resource_ids() {
		let tree_id = 0u32;
		let chain_id = 2000u32;
		let updated_chain_id: u64 = compute_chain_id_type(chain_id, [2, 0]);
		let resource_id = derive_resource_id(chain_id, tree_id);
		let (tree_id2, chain_id2): (u32, u64) = parse_resource_id(resource_id);
		assert_eq!(chain_id as u64, chain_id2);
		assert_eq!(tree_id, tree_id2);
	}
}
