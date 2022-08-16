use shaku::HasComponent;

use libadder::{AuthManager, MyComponent};

mod common;

#[test]
fn should_not_authenticate() {
  let adder_mod = common::build_adder_module();
  let auth: &dyn AuthManager = adder_mod.resolve_ref();
  assert!(!auth.do_auth("perm"));
}

#[test]
fn should_not_execute() {
  let adder_mod = common::build_adder_module();
  let my_component: &dyn MyComponent = adder_mod.resolve_ref();
  assert!(!my_component.can_execute());
}
