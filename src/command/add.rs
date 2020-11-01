use anyhow::Result;
use clap::Clap;

use crate::accounts::Accounts;

/// Add a new account
#[derive(Clap)]
pub struct Add {
  /// Name of the account
  pub account: String,
  /// Secret key used to generate code
  pub secret: String,
  /// Algorithm used to generate code
  #[clap(
    short,
    long,
    env = "YAYO_ALGORITHM",
    default_value = "SHA1",
    possible_values = &["SHA1", "SHA256", "SHA512"]
  )]
  pub algorithm: String,
  /// Number of digits composing code
  #[clap(short, long, env = "YAYO_DIGITS", default_value = "6")]
  pub digits: usize,
  /// Duration in seconds of step
  #[clap(short, long, env = "YAYO_STEP", default_value = "30")]
  pub step: u64,
}

impl Add {
  pub fn call(self, accounts: &mut Accounts) -> Result<()> {
    accounts.push(
      self.account,
      self.secret,
      self.algorithm,
      self.digits,
      self.step,
    )
  }
}
