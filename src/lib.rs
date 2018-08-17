#[macro_use]
extern crate structopt;
extern crate clap_flags;
#[macro_use]
extern crate failure;

use failure::ResultExt;

mod cli;
mod error;

pub use cli::Cli;
pub use error::{Error, ErrorKind, Result};

/// Does a thing
pub fn do_thing() -> ::Result<()> {
  do_inner_thing().context(ErrorKind::Other)?;
  Ok(())
}

fn do_inner_thing() -> ::Result<()> {
  Err(ErrorKind::Other)?;
  Ok(())
}
