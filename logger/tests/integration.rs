use shaku::HasComponent;

use logger::datelogger::DateLogger;
use logger::logger::Logger;

mod common;

#[test]
fn can_log() {
  let logging_module = common::build_module("Jan 26".to_string(), 2020);
  let logger: &dyn Logger = logging_module.resolve_ref();
  logger.log("Hello, World!!!");
}

#[test]
fn can_log_date() {
  let logging_module = common::build_module("Jan 26".to_string(), 2020);
  let logger: &dyn DateLogger = logging_module.resolve_ref();
  logger.log_date();
}
