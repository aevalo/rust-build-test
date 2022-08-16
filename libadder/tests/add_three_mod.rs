use shaku::HasComponent;

use libadder::AddThree;
use libadder::add_three::AddThreeModuleImpl;

#[test]
fn can_add_three() {
  let add_three_mod = AddThreeModuleImpl::builder().build();
  let adder: &dyn AddThree = add_three_mod.resolve_ref();
  let result = adder.add(1);
  assert_eq!(result, 4);
}
