use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  ext_contract, near_bindgen, wee_alloc,
};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Default)]
pub struct Con2 {
  friend: String,
  foe: String,
  i_dunno: bool,
}

#[near_bindgen]
impl Con2 {
  #[init]
  pub fn new(friend: String, foe: String, i_dunno: bool) -> Self {
    Self {
      friend,
      foe,
      i_dunno,
    }
  }
  pub fn get_friend(&self) -> String {
    self.friend.clone()
  }
  pub fn get_foe(&self) -> String {
    self.foe.clone()
  }
  pub fn get_i_dunno(&self) -> bool {
    self.i_dunno
  }
  pub fn set_foe(&mut self, foe: String) {
    self.foe = foe;
  }
  pub fn set_friend(&mut self, friend: String) {
    self.friend = friend;
  }
  pub fn set_i_dunno(&mut self, i_dunno: bool) {
    self.i_dunno = i_dunno;
  }
}

// Call methods on C1.
#[ext_contract(con1)]
pub trait Con1 {
  fn get_name(&self) -> String;
  fn get_number(&self) -> u32;
  fn set_name(&mut self, name: String);
  fn set_number(&mut self, number: u32);
}

// #[near_bindgen]
// impl Con2{
// }
