use serde::Deserialize;
use std::io;

#[derive(Deserialize, Debug)]
pub struct Package {
  pub repo: String,
  pub version: String,
  pub status: Option<String>,
}

#[derive(Debug)]
pub enum Error {
  Reqwest(reqwest::Error),
  Http(reqwest::StatusCode),
  Io(io::Error),
}

impl From<reqwest::Error> for Error {
  fn from(err: reqwest::Error) -> Self {
    Error::Reqwest(err)
  }
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Self {
    Error::Io(err)
  }
}
