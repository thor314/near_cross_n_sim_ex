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
  ContractAccount, UserAccount, STORAGE_AMOUNT,
};

// Bring contract crate into namespace
extern crate c1;
use c1::*;
// use c2::*;

// Load contracts' bytes.
near_sdk_sim::lazy_static! {
  static ref C1_BYTES: &'static [u8] = include_bytes!("./c1.wasm").as_ref();
  static ref C2_BYTES: &'static [u8] = include_bytes!("./c2.wasm").as_ref();
}

/// Deploy the contract and create some dummy accounts.
fn init_c1_and_c2(
  initial_balance: u128,
) -> (UserAccount, ContractAccount<Con1Contract>, UserAccount) {
  let main_account = init_simulator(None);

  // Create Con1.
  let deploy_market = deploy!(
      contract: Con1Contract,
      contract_id: "c1",
      bytes: &C1_BYTES,
      // User deploying the contract,
      signer_account: main_account,
      // init method
      init_method: new("Alex".to_string(), 0)
  );
  let alice = main_account.create_user("alice".to_string(), to_yocto("100"));

  // Now create Con2. Panics on panics! But there's no better example of doing this.
	// https://github.com/near/near-sdk-rs/blob/master/examples/fungible-token/tests/general.rs
  const C2_STORAGE_COSTS: u128 = 11_590_300_000_000_000_000_000_000;
  let c2 = alice.deploy(&C2_BYTES.to_vec(), "c2".to_string(), C2_STORAGE_COSTS);

  // let contract_two = deploy! (
  //   contract: Con2Contract,
    
  // )
  
  // Promise::new("c2".to_string())
  //   .create_account()
  //   .transfer(C2_STORAGE_COSTS)
  //   .add_full_access_key(env::signer_account_pk())
  //   .deploy_contract(C2_BYTES.to_vec());

  (main_account, deploy_market, alice)
}

// some tests
#[test]
fn test_get_friend() {
  let (master_account, contract, alice) = init_c1_and_c2(to_yocto("1000"));
  let res = call!(
    master_account,
    contract.get_friend(),
    deposit = STORAGE_AMOUNT
  );
}
