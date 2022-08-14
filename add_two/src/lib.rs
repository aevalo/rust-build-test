use shaku::{Component, module};

use libadder::Adder;

pub fn add_two(x: i32) -> i32 {
  x + 2
}

#[derive(Component)]
#[shaku(interface = Adder)]
pub struct AdderImpl;

impl Adder for AdderImpl {
  fn add(&self, x: i32) -> i32 {
    add_two(x)
  }
}

module! {
  pub AddTwoModule {
    components = [AdderImpl],
    providers = []
  }
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
