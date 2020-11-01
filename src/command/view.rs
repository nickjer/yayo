use anyhow::Result;
use clap::Clap;

use crate::accounts::Accounts;

/// View the code for an account
#[derive(Clap)]
pub struct View {
  /// Name of the account
  pub account: String,
}

impl View {
  pub fn call(self, accounts: &mut Accounts) -> Result<()> {
    let code = accounts.get_code(self.account)?;
    println!("{}", code);
    Ok(())
  }
}
