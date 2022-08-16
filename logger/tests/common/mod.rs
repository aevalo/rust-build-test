use shaku::Component;

use logger::datelogger::{DateLoggerImpl, DateLoggerImplParameters};
use logger::logger::Logger;
use logger::LoggingModule;

#[derive(Component)]
#[shaku(interface = Logger)]
pub struct FakeOutputImpl;

impl Logger for FakeOutputImpl {
  fn log(&self, _content: &str) {
    // Log nothing during testing
  }
}

pub fn build_module(today: String, year: usize) -> LoggingModule {
  LoggingModule::builder()
    .with_component_override::<dyn Logger>(Box::new(FakeOutputImpl))
    .with_component_parameters::<DateLoggerImpl>(DateLoggerImplParameters {
      today, year
    })
    .build()
}
