use shaku::module;

pub mod logger;
pub mod datelogger;

module! {
  pub LoggingModule {
    components = [logger::LoggerImpl, datelogger::DateLoggerImpl],
    providers = []
  }
}
