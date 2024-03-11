use super::flipper::*;
use ink_e2e::ContractsBackend;

type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

#[ink_e2e::test]
async fn instantiate_and_get<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
	// given
	let flipper_code = client
		.upload("flipper", &ink_e2e::alice())
		.submit()
		.await
		.expect("flipper upload failed");

	Ok(())
}
