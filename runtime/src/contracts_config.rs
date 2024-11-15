use crate::{
	Balance, Balances, BalancesCall, Perbill, RandomnessCollectiveFlip, Runtime, RuntimeCall,
	RuntimeEvent, RuntimeHoldReason, Timestamp,
};
use frame_support::{
	parameter_types,
	traits::{ConstBool, ConstU32},
};
use frame_system::EnsureSigned;

pub enum AllowBalancesCall {}

impl frame_support::traits::Contains<RuntimeCall> for AllowBalancesCall {
	fn contains(call: &RuntimeCall) -> bool {
		matches!(call, RuntimeCall::Balances(BalancesCall::transfer_allow_death { .. }))
	}
}

// Unit = the base number of indivisible units for balances
const UNIT: Balance = 1_000_000_000_000;
const MILLIUNIT: Balance = 1_000_000_000;

const fn deposit(items: u32, bytes: u32) -> Balance {
	(items as Balance * UNIT + (bytes as Balance) * (5 * MILLIUNIT / 100)) / 10
}

fn schedule<T: pallet_contracts::Config>() -> pallet_contracts::Schedule<T> {
	pallet_contracts::Schedule {
		limits: pallet_contracts::Limits {
			runtime_memory: 1024 * 1024 * 1024,
			..Default::default()
		},
		..Default::default()
	}
}

parameter_types! {
	pub const DepositPerItem: Balance = deposit(1, 0);
	pub const DepositPerByte: Balance = deposit(0, 1);
	pub Schedule: pallet_contracts::Schedule<Runtime> = schedule::<Runtime>();
	pub const DefaultDepositLimit: Balance = deposit(1024, 1024 * 1024);
	pub const CodeHashLockupDepositPercent: Perbill = Perbill::from_percent(0);
	pub const MaxDelegateDependencies: u32 = 32;
}

impl pallet_contracts::Config for Runtime {
	type Time = Timestamp;
	type Randomness = RandomnessCollectiveFlip;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;

	/// The safest default is to allow no calls at all.
	///
	/// Runtimes should whitelist dispatchables that are allowed to be called from contracts
	/// and make sure they are stable. Dispatchables exposed to contracts are not allowed to
	/// change because that would break already deployed contracts. The `RuntimeCall` structure
	/// itself is not allowed to change the indices of existing pallets, too.
	type CallFilter = AllowBalancesCall;
	type DepositPerItem = DepositPerItem;
	type DepositPerByte = DepositPerByte;
	type CallStack = [pallet_contracts::Frame<Self>; 23];
	type WeightPrice = pallet_transaction_payment::Pallet<Self>;
	type WeightInfo = pallet_contracts::weights::SubstrateWeight<Self>;
	type ChainExtension = ();
	type Schedule = Schedule;
	type AddressGenerator = pallet_contracts::DefaultAddressGenerator;
	// This node is geared towards development and testing of contracts.
	// We decided to increase the default allowed contract size for this
	// reason (the default is `128 * 1024`).
	//
	// Our reasoning is that the error code `CodeTooLarge` is thrown
	// if a too-large contract is uploaded. We noticed that it poses
	// less friction during development when the requirement here is
	// just more lax.
	type MaxCodeLen = ConstU32<{ 256 * 1024 }>;
	type DefaultDepositLimit = DefaultDepositLimit;
	type MaxStorageKeyLen = ConstU32<128>;
	type MaxTransientStorageSize = ConstU32<{ 1024 * 1024 }>;
	type MaxDebugBufferLen = ConstU32<{ 2 * 1024 * 1024 }>;
	type UnsafeUnstableInterface = ConstBool<true>;
	type CodeHashLockupDepositPercent = CodeHashLockupDepositPercent;
	type MaxDelegateDependencies = MaxDelegateDependencies;
	type RuntimeHoldReason = RuntimeHoldReason;

	type Environment = ();
	type Debug = ();
	type ApiVersion = ();
	type Migrations = ();
	#[cfg(feature = "parachain")]
	type Xcm = pallet_xcm::Pallet<Self>;
	#[cfg(not(feature = "parachain"))]
	type Xcm = ();

	type UploadOrigin = EnsureSigned<Self::AccountId>;
	type InstantiateOrigin = EnsureSigned<Self::AccountId>;
}
