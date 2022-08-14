use shaku::HasComponent;

#[cfg(any(feature = "add_one", feature = "add_two"))]
fn adds(num: i32) {
  #[cfg(feature = "add_one")]
  println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
  #[cfg(feature = "add_two")]
  println!("Hello, world! {num} plus two is {}!", add_two::add_two(num));
}

// Mutually exclusive features can be done.
// See notes here: https://doc.rust-lang.org/cargo/reference/features.html#mutually-exclusive-features
//#[cfg(all(feature = "add_one", feature = "add_two"))]
//compile_error!("feature \"add_one\" and feature \"add_two\" cannot be enabled at the same time");

fn main() {
  let module = logger::LoggingModule::builder()
    .with_component_parameters::<logger::datelogger::DateLoggerImpl>(logger::datelogger::DateLoggerImplParameters {
      today: "Jan 26".to_string(),
      year: 2020
    })
    .build();

  let date_logger: &dyn logger::datelogger::DateLogger = module.resolve_ref();
  date_logger.log_date();

  #[cfg(any(feature = "add_one", feature = "add_two"))]
  adds(10);
}
