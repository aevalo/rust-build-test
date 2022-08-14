use std::sync::Arc;
use shaku::{Component, Interface};

use super::logger::Logger;

pub trait DateLogger: Interface {
  fn log_date(&self);
}

#[derive(Component)]
#[shaku(interface = DateLogger)]
pub struct DateLoggerImpl {
  #[shaku(inject)]
  logger: Arc<dyn Logger>,
  today: String,
  year: usize,
}

impl DateLogger for DateLoggerImpl {
  fn log_date(&self) {
    self.logger.log(&format!("Today is {}, {}", self.today, self.year));
  }
}
