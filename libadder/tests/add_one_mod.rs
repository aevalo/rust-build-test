use shaku::HasComponent;

use libadder::AddOne;
use libadder::add_one::AddOneModuleImpl;

#[test]
fn can_add_one() {
  let add_one_mod = AddOneModuleImpl::builder().build();
  let adder: &dyn AddOne = add_one_mod.resolve_ref();
  let result = adder.add(1);
  assert_eq!(result, 2);
}
