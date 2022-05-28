use anyhow::{anyhow, Result};
use clap::crate_name;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::otp::Otp;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
  name: String,
  algorithm: String,
  digits: usize,
  step: u64,
}

impl PartialEq<String> for Account {
  fn eq(&self, other: &String) -> bool {
    self.name == *other
  }
}

impl std::fmt::Display for Account {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Account: {}\n\
      Algorithm: {}\n\
      Digits: {}\n\
      Step: {}",
      self.name, self.algorithm, self.digits, self.step
    )
  }
}

impl Account {
  pub fn new(
    name: String,
    algorithm: String,
    digits: usize,
    step: u64,
  ) -> Self {
    Self {
      name,
      algorithm,
      digits,
      step,
    }
  }

  pub fn save_secret(&self, secret: String) -> Result<()> {
    self
      .keyring()
      .set_password(&secret)
      .map_err(|e| anyhow!("Failed saving secret ({:?})", e))
  }

  pub fn delete_secret(&self) -> Result<()> {
    self
      .keyring()
      .delete_password()
      .map_err(|e| anyhow!("Failed deleting secret ({:?})", e))
  }

  pub fn get_code(&self) -> Result<String> {
    let time = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)?
      .as_secs() as u64;
    let secret = self.get_secret()?;
    let otp = Otp::new(&secret, self.algorithm.trim(), self.digits, self.step);
    otp.get_code(time)
  }

  fn get_secret(&self) -> Result<String> {
    self
      .keyring()
      .get_password()
      .map_err(|e| anyhow!("Failed reading secret ({:?})", e))
  }

  fn keyring(&self) -> keyring::Entry {
    keyring::Entry::new(crate_name!(), &self.name)
  }
}
