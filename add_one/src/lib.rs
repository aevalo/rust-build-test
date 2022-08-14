use shaku::{Component, module};

use libadder::Adder;

pub fn add_one(x: i32) -> i32 {
  x + 1
}

#[derive(Component)]
#[shaku(interface = Adder)]
pub struct AdderImpl;

impl Adder for AdderImpl {
  fn add(&self, x: i32) -> i32 {
    add_one(x)
  }
}

module! {
  pub AddOneModule {
    components = [AdderImpl],
    providers = []
  }
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
