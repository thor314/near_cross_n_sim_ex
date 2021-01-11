use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  near_bindgen,
};

#[cfg(target = "wasm32")]
#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Con2 {
  friend: String,
  foe: String,
  i_dunno: bool,
}

impl Default for Con2 {
  fn default() -> Self {
    Con2 {
      friend: "Todd".to_string(),
      foe: "clowns are in my head".to_string(),
      i_dunno: true,
    }
  }
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
