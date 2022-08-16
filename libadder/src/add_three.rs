use shaku::{Component, module};

#[derive(Component)]
#[shaku(interface = super::AddThree)]
pub struct AddThreeImpl;

impl super::AddThree for AddThreeImpl {
  fn add(&self, x: i32) -> i32 {
    add_three(x)
  }
}

module! {
  pub AddThreeModuleImpl: super::AddThreeModule {
    components = [AddThreeImpl],
    providers = []
  }
}

fn add_three(x: i32) -> i32 {
  x + 3
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_three_works() {
    let result = add_three(1);
    assert_eq!(result, 4);
  }
}
