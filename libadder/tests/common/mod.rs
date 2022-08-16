use shaku::{Component, module};
use std::sync::Arc;

use libadder::{AdderModule, AuthManager, AuthModule};
use libadder::add_one::AddOneModuleImpl;
use libadder::add_two::AddTwoModuleImpl;
use libadder::add_three::AddThreeModuleImpl;

#[derive(Component)]
#[shaku(interface = AuthManager)]
pub struct FakeAuthManagerImpl;

impl AuthManager for FakeAuthManagerImpl {
  fn do_auth(&self, _permission: &str) -> bool {
    false
  }
}

module! {
  AuthModuleImpl: AuthModule {
    components = [FakeAuthManagerImpl],
    providers = []
  }
}

pub fn build_adder_module() -> AdderModule {
  let auth_mod = Arc::new(AuthModuleImpl::builder().build());
  let add_one_mod = Arc::new(AddOneModuleImpl::builder().build());
  let add_two_mod = Arc::new(AddTwoModuleImpl::builder().build());
  let add_three_mod = Arc::new(AddThreeModuleImpl::builder().build());
  AdderModule::builder(auth_mod, add_one_mod, add_two_mod, add_three_mod).build()
}
