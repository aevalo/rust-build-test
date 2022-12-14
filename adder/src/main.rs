use std::sync::Arc;
use shaku::{Component, HasComponent, module};
use libadder::{AdderModule, AddOne, AddTwo, AddThree, AuthManager, AuthModule, MyComponent};
use libadder::add_one::AddOneModuleImpl;
use libadder::add_two::AddTwoModuleImpl;
use libadder::add_three::AddThreeModuleImpl;

/*
#[cfg(any(feature = "add_one", feature = "add_two"))]
fn adds(num: i32) {
  #[cfg(feature = "add_one")]
  println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
  #[cfg(feature = "add_two")]
  println!("Hello, world! {num} plus two is {}!", add_two::add_two(num));
}
*/

// Mutually exclusive features can be done.
// See notes here: https://doc.rust-lang.org/cargo/reference/features.html#mutually-exclusive-features
//#[cfg(all(feature = "add_one", feature = "add_two"))]
//compile_error!("feature \"add_one\" and feature \"add_two\" cannot be enabled at the same time");

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

fn main() {
  let module = logger::LoggingModule::builder()
    .with_component_parameters::<logger::datelogger::DateLoggerImpl>(logger::datelogger::DateLoggerImplParameters {
      today: "Jan 26".to_string(),
      year: 2020
    })
    .build();

  let date_logger: &dyn logger::datelogger::DateLogger = module.resolve_ref();
  date_logger.log_date();

  let auth_mod = Arc::new(AuthModuleImpl::builder().build());
  let add_one_mod = Arc::new(AddOneModuleImpl::builder().build());
  let add_two_mod = Arc::new(AddTwoModuleImpl::builder().build());
  let add_three_mod = Arc::new(AddThreeModuleImpl::builder().build());
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
