use anyhow::Result;
use clap::{Args, ValueHint};

use crate::accounts::Accounts;

/// View the code for an account
#[derive(Args, Debug)]
pub struct View {
  /// Name of the account
  #[clap(value_hint = ValueHint::Other)]
  pub account: String,
}

impl View {
  pub fn call(self, accounts: &mut Accounts) -> Result<()> {
    let code = accounts.get_code(self.account)?;
    println!("{}", code);
    Ok(())
  }
}
