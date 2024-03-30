use anyhow::Result;
use clap::{Args, ValueHint};

use crate::accounts::Accounts;

/// Delete an account
#[derive(Args, Debug)]
pub struct Delete {
  /// Name of the account
  #[clap(value_hint = ValueHint::Other)]
  pub account: String,
}

impl Delete {
  pub fn call(self, accounts: &mut Accounts) -> Result<()> {
    accounts.delete(self.account)
  }
}
