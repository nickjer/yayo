use anyhow::Result;
use clap::Args;

use crate::accounts::Accounts;

/// List all accounts
#[derive(Args, Debug)]
pub struct List {}

impl List {
  pub fn call(self, accounts: &Accounts) -> Result<()> {
    println!(
      "{}",
      accounts
        .into_iter()
        .map(|account| format!("{}", account))
        .collect::<Vec<String>>()
        .join("\n\n")
    );
    Ok(())
  }
}
