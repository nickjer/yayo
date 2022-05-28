use anyhow::Result;
use clap::Args;

use crate::accounts::Accounts;

/// Delete an account
#[derive(Args, Debug)]
pub struct Delete {
  /// Name of the account
  pub account: String,
}

impl Delete {
  pub fn call(self, accounts: &mut Accounts) -> Result<()> {
    accounts.delete(self.account)
  }
}
