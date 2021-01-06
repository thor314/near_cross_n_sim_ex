#![allow(dead_code)]
use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  env, ext_contract, near_bindgen, wee_alloc, Promise, PromiseOrValue,
};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Default)]
pub struct Con1 {
  pub(crate) name: String,
  pub(crate) number: u32,
}

#[near_bindgen]
impl Con1 {
  #[init]
  pub fn new(name: String, number: u32) -> Self {
    Self { name, number }
  }
  pub fn get_name(&self) -> String {
    self.name.clone()
  }
  pub fn get_number(&self) -> u32 {
    self.number
  }
  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }
  pub fn set_number(&mut self, number: u32) {
    self.number = number;
  }
}

// Call methods on C2.
#[ext_contract(con2)]
pub trait Con2 {
  fn get_friend(&self) -> PromiseOrValue<String>;
  fn get_foe(&self) -> PromiseOrValue<String>;
  fn get_i_dunno(&self) -> PromiseOrValue<bool>;
  fn set_foe(&mut self, foe: String);
  fn set_friend(&mut self, friend: String);
  fn set_i_dunno(&mut self, i_dunno: bool);
}

const SINGLE_CALL_GAS: u64 = 100_000_000_000_000;
// Simple cross contract calls
#[near_bindgen]
impl Con1 {
  /// Deploy Con2 from within Con1 to a new address: c3.YOURADDRESS.testnet.
  pub fn deploy_con2_to(&self, subaddress: String) {
    Self::log_stuff();
    const C2_STORAGE_COSTS: u128 = 11_590_300_000_000_000_000_000_000;
    let c2wasm = include_bytes!("../../c2/res/c2.wasm").to_vec();
    Promise::new(subaddress)
      .create_account() // create address c3.YOURADDRESS.testnet
      .transfer(C2_STORAGE_COSTS) // cover storage costs
      .add_full_access_key(env::signer_account_pk()) // give the caller of this method (you) full access on that address's behalf
      .deploy_contract(c2wasm); // deploy c2 to that address
  }

  #[result_serializer(borsh)]
  pub fn get_friend(&self) -> PromiseOrValue<String> {
    let address: String = env::current_account_id()
      .split_terminator(".")
      .collect::<Vec<&str>>()[1]
      .to_string();
    let c2 = format!("c2.{}.testnet", address);
    // Note on this number: I don't have a good idea of how to estimate gas, but generally:
    // SINGLE_CALL_GAS /4 < N < SINGLE_CALL_GAS * 95/100
    // seems the right number. Higher causes `GasExceeded` errors, which can be solved with shoving more gas in, ie
    // add the arg `--gas 300000000000000`. This is an area I think NEAR is working on improving clarity.
    con2::get_friend(&c2, 0, SINGLE_CALL_GAS / 2).into()
  }

  pub fn set_friend(&self, friend: String) {
    let address: String = env::current_account_id()
      .split_terminator(".")
      .collect::<Vec<&str>>()[1]
      .to_string();
    let c2 = format!("c2.{}.testnet", address);
    con2::set_friend(friend, &c2, 0, SINGLE_CALL_GAS / 2);
  }

  pub fn get_foe(&self) -> PromiseOrValue<String> {
    let address: String = env::current_account_id()
      .split_terminator(".")
      .collect::<Vec<&str>>()[1]
      .to_string();
    let c2 = format!("c2.{}.testnet", address);
    con2::get_foe(&c2, 0, SINGLE_CALL_GAS / 2).into()
  }

  pub fn set_foe(&self, foe: String) {
    let address: String = env::current_account_id()
      .split_terminator(".")
      .collect::<Vec<&str>>()[1]
      .to_string();
    let c2 = format!("c2.{}.testnet", address);
    con2::set_foe(foe, &c2, 0, SINGLE_CALL_GAS / 2);
  }

  pub fn get_i_dunno(&self) -> PromiseOrValue<bool> {
    let address: String = env::current_account_id()
      .split_terminator(".")
      .collect::<Vec<&str>>()[1]
      .to_string();
    let c2 = format!("c2.{}.testnet", address);
    con2::get_i_dunno(&c2, 0, SINGLE_CALL_GAS / 2).into()
  }

  pub fn set_i_dunno(&self, i_dunno: bool) {
    let address: String = env::current_account_id()
      .split_terminator(".")
      .collect::<Vec<&str>>()[1]
      .to_string();
    let c2 = format!("c2.{}.testnet", address);
    con2::set_i_dunno(i_dunno, &c2, 0, SINGLE_CALL_GAS / 2);
  }

  pub fn log_stuff() {
    // Logging messages like these can be super useful.
    env::log(format!("prep gas: {}", env::prepaid_gas()).as_bytes());
    env::log(format!("current_account_id: {}", env::current_account_id()).as_bytes());
    env::log(format!("signer_account_id: {}", env::signer_account_id()).as_bytes());
    env::log(format!("predecessor_account_id: {}", env::predecessor_account_id()).as_bytes());
  }
}

// methods for callbacks
#[ext_contract(c1cb)]
pub trait Con1Callbacks {
  fn cb_set_name(
    &self,
    #[callback]
    #[serializer(borsh)]
    name: String,
  );
  fn cb_increment_number_if_true(
    &self,
    #[callback]
    #[serializer(borsh)]
    b: bool,
  );
}

// Methods that do stuff after callbacks
#[near_bindgen]
// impl Con1Callbacks for Con1 // Nope, normal is not how this works. That would be nice though.
impl Con1 {
  #[result_serializer(borsh)]
  fn cb_set_name(
    &mut self,
    #[callback]
    #[serializer(borsh)]
    name: String,
  ) {
    self.set_name(name);
  }

  #[result_serializer(borsh)]
  fn cb_increment_number_if_true(
    &mut self,
    #[callback]
    #[serializer(borsh)]
    b: bool,
  ) {
    if b {
      self.set_number(self.number + 1);
    }
  }
}

// Methods that generate Callbacks
#[near_bindgen]
impl Con1 {
  /// Call `get_friend` and use it to call `set_name` locally, using `cb_set_name` as an intermediary.
  pub fn cb_get_friend_then_set_name(&mut self) {
    // returns PromiseOrValue<String>, where the String will be taken as a callback argument
    con2::get_friend(&env::current_account_id(), 0, SINGLE_CALL_GAS / 2)
      // self.get_friend() // This (better) syntax fails. Sad face for no code reus.
      // Take the string as a callback argument.
      .then(c1cb::cb_set_name(
        &env::current_account_id(),
        0,
        SINGLE_CALL_GAS / 2,
      ));
  }


	  /// Call `get_friend` and use it to call `set_name` locally, using `cb_set_name` as an intermediary.
	/// Then call set_foe on C2 with the old `name` value.
  pub fn cb_get_friend_then_set_name_then_set_foe(&mut self) {
		let temp_foe = &self.name;
    con2::get_friend(&env::current_account_id(), 0, SINGLE_CALL_GAS / 2)
      .then(c1cb::cb_set_name(
        &env::current_account_id(),
        0,
        SINGLE_CALL_GAS / 2,
      ))
    .then(con2::set_foe( // not a callback, just a followup then
      temp_foe.to_string(),
      &env::current_account_id(),
      0,
      SINGLE_CALL_GAS / 2,
    ));
  }

  /// Call `get_i_dunno`, and if it's true, increment number
  pub fn cb_get_i_dunno_incr_number(&mut self) {
    con2::get_i_dunno(&env::current_account_id(), 0, SINGLE_CALL_GAS / 2) // returns PromiseOrValue<bool>, where the bool will be taken as a callback argument
      .then(c1cb::cb_increment_number_if_true(
        &env::current_account_id(),
        0,
        SINGLE_CALL_GAS / 2,
      ));
  }
}
