use shaku::{Component, module};

#[derive(Component)]
#[shaku(interface = super::AddTwo)]
pub struct AddTwoImpl;

impl super::AddTwo for AddTwoImpl {
  fn add(&self, x: i32) -> i32 {
    add_two(x)
  }
}

module! {
  pub AddTwoModuleImpl: super::AddTwoModule {
    components = [AddTwoImpl],
    providers = []
  }
}

fn add_two(x: i32) -> i32 {
  x + 2
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_two_works() {
    let result = add_two(1);
    assert_eq!(result, 3);
  }
}
