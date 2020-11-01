use anyhow::Result;
use clap::Clap;

use crate::accounts::Accounts;

/// List all accounts
#[derive(Clap)]
pub struct List {}

impl List {
  pub fn call(self, accounts: &mut Accounts) -> Result<()> {
    println!("deserialized = {:?}", accounts);
    Ok(())
  }
}
