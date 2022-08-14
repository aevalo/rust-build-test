use shaku::Interface;

pub trait Adder: Interface {
  fn add(&self, x: i32) -> i32;
}
