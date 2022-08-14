extern crate logger;

use shaku::Component;

use logger::datelogger::{DateLoggerImpl, DateLoggerImplParameters};
use logger::logger::Logger;
use logger::LoggingModule;

#[derive(Component)]
#[shaku(interface = Logger)]
pub struct FakeOutput;

impl Logger for FakeOutput {
  fn log(&self, _content: &str) {
    // Log nothing during testing
  }
}

pub fn build_module(today: String, year: usize) -> LoggingModule {
  return LoggingModule::builder()
    .with_component_override::<dyn Logger>(Box::new(FakeOutput))
    .with_component_parameters::<DateLoggerImpl>(DateLoggerImplParameters {
      today: today,
      year: year
    })
    .build();
}
