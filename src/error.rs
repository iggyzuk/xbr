// todo: use error when xbr fails
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum XBRError {
  OpeningFile,
  LoadingFromMemory,
}

impl Display for XBRError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "XBRError")
  }
}

impl Error for XBRError {}
