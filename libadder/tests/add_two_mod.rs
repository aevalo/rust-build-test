use shaku::HasComponent;

use libadder::AddTwo;
use libadder::add_two::AddTwoModuleImpl;

#[test]
fn can_add_two() {
  let add_two_mod = AddTwoModuleImpl::builder().build();
  let adder: &dyn AddTwo = add_two_mod.resolve_ref();
  let result = adder.add(1);
  assert_eq!(result, 3);
}
