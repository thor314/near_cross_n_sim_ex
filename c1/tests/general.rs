#![allow(unused_imports)]
#![allow(unused_variables)]
use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  json_types::U128,
  serde::{Deserialize, Serialize},
  serde_json::json,
  Promise, *,
};
use near_sdk_sim::{
  account::AccessKey, call, deploy, init_simulator, near_crypto::Signer, to_yocto, view,
  ContractAccount, UserAccount, STORAGE_AMOUNT, DEFAULT_GAS
};

// Bring contract crate into namespace
extern crate c1;
use c1::*;
use c2::*;

// Load contracts' bytes.
near_sdk_sim::lazy_static! {
  static ref C1_BYTES: &'static [u8] = include_bytes!("./c1.wasm").as_ref();
  static ref C2_BYTES: &'static [u8] = include_bytes!("./c2.wasm").as_ref();
}

/// Deploy the contract(s) and create some dummy accounts. Returns:
/// - Contract1
/// - Contract2
/// - Root Account
/// - Testnet Account (utility suffix for building other addresses)
/// - The account Dingu, who deploys contracts 1 and 2 to its subaccounts
fn init_c1_and_c2(
  initial_balance: u128,
) -> (
  ContractAccount<Con1Contract>,
  ContractAccount<Con2Contract>,
  UserAccount, // root
  UserAccount, // testnet suffix
  UserAccount, // Dingu, who deploys contracts 1 and 2 to its subaccounts
) {
	// Root account has address: "root"
  let root_account = init_simulator(None);

	// Other accounts may be created from the root account
	// Note: address naming is fully expressive: we may create any suffix we desire, ie testnet, near, etc.
	// but only those two (.testnet, .near) will be used in practice.
  let testnet = root_account.create_user("testnet".to_string(), to_yocto("1000000"));

	// We need an account to deploy the contracts from. We may create subaccounts as follows:
  let dingu = testnet.create_user("dingu.testnet".to_string(), to_yocto("10000"));

  // Create Con1 with the deploy macro.
  let c1 = deploy!(
      contract: Con1Contract,
      contract_id: "c1.dingu.testnet",
      bytes: &C1_BYTES,
      // User deploying the contract,
      signer_account: dingu,
      // init method
      init_method: new("mah name".to_string(), 0)
  );

	// Create Con2 either with the same macro (its code and wasm blob should BOTH be in this workspace), or see below
	const C2_STORAGE_COSTS: u128 = 50000000000000000000000010;
  let c2 =
    deploy!(
      contract: Con2Contract,
      contract_id: "c2.dingu.testnet",
      bytes: &C2_BYTES,
      signer_account: dingu,
      deposit: C2_STORAGE_COSTS,
      gas: DEFAULT_GAS,
      init_method: new("Todd".to_string(), "clowns in my hair".to_string(), true)
    );

	// The other way to deploy a contract, use an existing account:
  // dingu.deploy(
  //   &C2_BYTES.to_vec(),
  //   "c2.dingu.testnet".to_string(),
  //   C2_STORAGE_COSTS,
  // );
	// This method is less preferable because the following call macro only succeeds with the first method.
	// let res = call!(
	// 	dingu,
	// 	c2.new("Todd".to_string(), "clowns".to_string(), true),
	// 	deposit = 0
	// );

	// Also note, this deployment method will fail because we can't call Promises from sim tests.
  // Promise::new("c2".to_string())
  //   .create_account()
  //   .transfer(C2_STORAGE_COSTS)
  //   .add_full_access_key(env::signer_account_pk())
  //   .deploy_contract(C2_BYTES.to_vec());

  (c1, c2, root_account, testnet, dingu)
}

// some tests
#[test]
fn test_get_friend() {
  let (root_account, c1, testnet, dingu, c2) = init_c1_and_c2(to_yocto("100000000000"));
  // let res = call!(
  //   // access local state on root_account contract c1.
  //   dingu,
  //   c1.get_name(),
  //   deposit = STORAGE_AMOUNT // WHY DID THIS PASS?!?!
  // );
  // println!("hello friend: {:#?}", res.get_receipt_results());
  // println!("hello friend: {:#?}", res);
  // assert!(res.is_ok(), "1"); // minimal panic achieved

  // // Can't create new account "" because it already exists
  // let res = call!(
  //   dingu, // access local state on "alice" c1 c2.
  //   c1.get_friend(),
  //   deposit = STORAGE_AMOUNT
  // );
  // println!("hello friend: {:#?}", res.get_receipt_results());
  // println!("hello friend: {:#?}", res);
  // assert!(res.is_ok(), "2");

  let res = call!(
    dingu, // access cross c1 state on "alice" c1 c2 from c1.
    c1.cb_get_friend_then_set_name(),
    deposit = 0
  );
  // In this case we want to see all of the promises there were generated since we
  // no longer have the receipts as part of the result
  println!("Promise results: {:#?}", res.promise_results());
  println!("Result: {:#?}", res);
  assert!(res.is_ok(), "3");

  let res = call!(
    dingu, // access cross c1 state on "alice" c1 c2 from c1.
    c1.get_name(),
    deposit = 0
  );
  // In this case we want to see all of the promises there were generated since we
  // no longer have the receipts as part of the result println!("Result: {:#?}", res);
}
