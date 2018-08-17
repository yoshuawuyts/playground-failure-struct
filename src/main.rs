#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

#[macro_use]
extern crate human_panic;
extern crate structopt;
#[macro_use]
extern crate log;
extern crate exitfailure;
extern crate failure;
extern crate playground_failure_struct;

use exitfailure::ExitFailure;
use playground_failure_struct::Cli;
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
  setup_panic!();
  let args = Cli::from_args();
  args.logger.log_all(args.verbosity.log_level())?;
  playground_failure_struct::do_thing()?;
  info!("program started");
  Ok(())
}
