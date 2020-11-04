use anyhow::Result;
use clap::Clap;

use crate::accounts::Accounts;

/// List all accounts
#[derive(Clap)]
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
