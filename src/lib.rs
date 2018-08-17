#[macro_use]
extern crate structopt;
extern crate clap_flags;
#[macro_use]
extern crate failure;

use failure::ResultExt;
use std::io;

mod cli;
mod error;

pub use cli::Cli;
pub use error::{Error, ErrorKind, Result};

/// Test function.
pub fn do_thing() -> ::Result<()> {
  do_io_thing().context(ErrorKind::Other)?;
  Ok(())
}

/// Test function.
fn do_io_thing() -> ::Result<()> {
  Err(ErrorKind::Io(io::Error::from_raw_os_error(98)))?;
  Ok(())
}

/// Test function.
pub fn do_regular_thing() -> ::Result<()> {
  Err(ErrorKind::Other)?;
  Ok(())
}
