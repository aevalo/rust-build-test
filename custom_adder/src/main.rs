use std::sync::Arc;
use shaku::{Component, HasComponent, module};
use libadder::{AdderModule, AddOne, AddTwo, AddThree, AddThreeModule, AuthManager, AuthModule, MyComponent};
use libadder::add_one::AddOneModuleImpl;
use libadder::add_two::AddTwoModuleImpl;

#[derive(Component)]
#[shaku(interface = AuthManager)]
struct AuthManagerImpl;

impl AuthManager for AuthManagerImpl {
  fn do_auth(&self, _permission: &str) -> bool {
    true
  }
}

module! {
  AuthModuleImpl: AuthModule {
    components = [AuthManagerImpl],
    providers = []
  }
}

#[derive(Component)]
#[shaku(interface = AddThree)]
struct CustomAddThreeImpl;

impl AddThree for CustomAddThreeImpl {
  fn add(&self, x: i32) -> i32 {
    x - 3
  }
}

module! {
  CustomAddThreeModuleImpl: AddThreeModule {
    components = [CustomAddThreeImpl],
    providers = []
  }
}

fn main() {
  let auth_mod = Arc::new(AuthModuleImpl::builder().build());
  let add_one_mod = Arc::new(AddOneModuleImpl::builder().build());
  let add_two_mod = Arc::new(AddTwoModuleImpl::builder().build());
  let add_three_mod = Arc::new(CustomAddThreeModuleImpl::builder().build());
  let adder_mod = AdderModule::builder(auth_mod, add_one_mod, add_two_mod, add_three_mod).build();
  
  let my_component: &dyn MyComponent = adder_mod.resolve_ref();
  println!("My component can execute? {}", my_component.can_execute());
  let add_one: &dyn AddOne = adder_mod.resolve_ref();
  println!("Add one: {}", add_one.add(10));
  let add_two: &dyn AddTwo = adder_mod.resolve_ref();
  println!("Add two: {}", add_two.add(10));
  let add_three: &dyn AddThree = adder_mod.resolve_ref();
  println!("Add three: {}", add_three.add(10));
}
