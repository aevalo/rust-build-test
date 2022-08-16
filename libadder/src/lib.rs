use shaku::{Component, HasComponent, Interface, module};
use std::sync::Arc;

#[cfg(feature = "add_one")]
pub mod add_one;
#[cfg(feature = "add_two")]
pub mod add_two;
#[cfg(feature = "add_three")]
pub mod add_three;

pub trait MyComponent: Interface {
  fn can_execute(&self) -> bool;
}

#[derive(Component)]
#[shaku(interface = MyComponent)]
pub struct MyComponentImpl {
    #[shaku(inject)]
    auth_manager: Arc<dyn AuthManager>
}

impl MyComponent for MyComponentImpl {
  fn can_execute(&self) -> bool {
    self.auth_manager.do_auth("execute")
  }
}

pub trait AuthManager: Interface {
  fn do_auth(&self, permission: &str) -> bool;
}
pub trait AuthModule: HasComponent<dyn AuthManager> {}

pub trait AddOne: Interface {
  fn add(&self, x: i32) -> i32;
}
pub trait AddOneModule: HasComponent<dyn AddOne> {}

pub trait AddTwo: Interface {
  fn add(&self, x: i32) -> i32;
}
pub trait AddTwoModule: HasComponent<dyn AddTwo> {}

pub trait AddThree: Interface {
  fn add(&self, x: i32) -> i32;
}
pub trait AddThreeModule: HasComponent<dyn AddThree> {}

module! {
  pub AdderModule {
    components = [MyComponentImpl],
    providers = [],

    use dyn AuthModule {
      components = [dyn AuthManager],
      providers = []
    },

    use dyn AddOneModule {
      components = [dyn AddOne],
      providers = [],
    },

    use dyn AddTwoModule {
      components = [dyn AddTwo],
      providers = [],
    },

    use dyn AddThreeModule {
      components = [dyn AddThree],
      providers = [],
    }
  }
}
