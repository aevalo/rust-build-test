use shaku::HasComponent;

use libadder::Adder;

mod common;

#[test]
fn can_add_one() {
  let adder_module = common::build_module();
  let adder: &dyn Adder = adder_module.resolve_ref();
  let result = adder.add(1);
  assert_eq!(result, 2);
}
