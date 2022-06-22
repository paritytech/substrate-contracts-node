use super::{ElementTrait, IntoAbiToken, Token};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::vec::Vec;

#[derive(Clone, Encode, Decode, TypeInfo)]
pub struct VAnchorMetadata<AccountId, AssetId> {
	/// Creator account
	pub creator: Option<AccountId>,
	/// Option of specifying a fungible asset. When None, the asset is the
	/// native currency.
	pub asset: AssetId,
}

#[derive(Clone, Encode, Decode, Debug, Eq, PartialEq, TypeInfo)]
pub struct ProofData<E> {
	pub proof: Vec<u8>,
	pub public_amount: E,
	pub roots: Vec<E>,
	pub input_nullifiers: Vec<E>,
	pub output_commitments: Vec<E>,
	pub ext_data_hash: E,
}

impl<E: ElementTrait> ProofData<E> {
	pub fn new(
		proof: Vec<u8>,
		public_amount: E,
		roots: Vec<E>,
		input_nullifiers: Vec<E>,
		output_commitments: Vec<E>,
		ext_data_hash: E,
	) -> Self {
		Self { proof, public_amount, roots, input_nullifiers, output_commitments, ext_data_hash }
	}
}

#[derive(Encode, Decode, Default, Debug, Clone, Eq, PartialEq, Copy, TypeInfo)]
pub struct ExtData<AccountId: Encode, Amount: Encode, Balance: Encode, Element: Encode> {
	pub recipient: AccountId,
	pub relayer: AccountId,
	pub ext_amount: Amount,
	pub fee: Balance,
	pub encrypted_output1: Element,
	pub encrypted_output2: Element,
}

impl<I: Encode, A: Encode, B: Encode, E: Encode> ExtData<I, A, B, E> {
	pub fn new(
		recipient: I,
		relayer: I,
		ext_amount: A,
		fee: B,
		encrypted_output1: E,
		encrypted_output2: E,
	) -> Self {
		Self { recipient, relayer, ext_amount, fee, encrypted_output1, encrypted_output2 }
	}
}

impl<I: Encode, A: Encode, B: Encode, E: Encode> IntoAbiToken for ExtData<I, A, B, E> {
	fn into_abi(&self) -> Token {
		// TODO: Make sure the encodings match the solidity side
		let recipient = Token::Bytes(self.recipient.encode());
		let ext_amount = Token::Bytes(self.ext_amount.encode());
		let relayer = Token::Bytes(self.relayer.encode());
		let fee = Token::Bytes(self.fee.encode());
		let encrypted_output1 = Token::Bytes(self.encrypted_output1.encode());
		let encrypted_output2 = Token::Bytes(self.encrypted_output2.encode());
		let mut ext_data_args = Vec::new();
		ext_data_args.push(recipient);
		ext_data_args.push(relayer);
		ext_data_args.push(ext_amount);
		ext_data_args.push(fee);
		ext_data_args.push(encrypted_output1);
		ext_data_args.push(encrypted_output2);
		Token::Tuple(ext_data_args)
	}
}
