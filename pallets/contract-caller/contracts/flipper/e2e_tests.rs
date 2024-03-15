use super::flipper::*;
use ink_e2e::{ChainBackend, ContractsBackend};

type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

#[ink_e2e::test]
async fn instantiate_and_get<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
	let mut constructor = FlipperRef::new(false);

	let contract = client
		.instantiate("flipper", &ink_e2e::alice(), &mut constructor)
		.submit()
		.await
		.expect("instantiate failed");

	// call pallet dispatchable
	client
		.runtime_call(
			&ink_e2e::alice(),
			"ContractCaller",
			"contract_call_flip",
			vec![
				scale_value::Value::from_bytes(contract.account_id),
				scale_value::serde::to_value(
					frame_support::weights::Weight::from_parts(1_000_000_000, 0),
				).unwrap(),
			],
		)
		.await
		.expect("runtime call failed");

	println!("contract: {:?}", contract.account_id);

	Ok(())
}
