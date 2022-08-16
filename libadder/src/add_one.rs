use shaku::{Component, module};

#[derive(Component)]
#[shaku(interface = super::AddOne)]
pub struct AddOneImpl;

impl super::AddOne for AddOneImpl {
  fn add(&self, x: i32) -> i32 {
    add_one(x)
  }
}

module! {
  pub AddOneModuleImpl: super::AddOneModule {
    components = [AddOneImpl],
    providers = []
  }
}

fn add_one(x: i32) -> i32 {
  x + 1
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_one_works() {
    let result = add_one(1);
    assert_eq!(result, 2);
  }
}
