use anyhow::Result;
use clap::Clap;

use crate::accounts::Accounts;

/// Delete an account
#[derive(Clap)]
pub struct Delete {
  /// Name of the account
  pub account: String,
}

impl Delete {
  pub fn call(self, accounts: &mut Accounts) -> Result<()> {
    accounts.delete(self.account)
  }
}
