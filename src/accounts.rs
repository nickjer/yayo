use crate::account::Account;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Accounts {
  accounts: Vec<Account>,
}

impl<'a> IntoIterator for &'a Accounts {
  type Item = &'a Account;
  type IntoIter = std::slice::Iter<'a, Account>;

  fn into_iter(self) -> Self::IntoIter {
    self.accounts.iter()
  }
}

impl Default for Accounts {
  fn default() -> Self {
    Self::new()
  }
}

impl Accounts {
  pub fn new() -> Accounts {
    Accounts {
      accounts: Vec::new(),
    }
  }

  pub fn from_string(string: &str) -> Result<Accounts> {
    serde_json::from_str(&string)
      .context("Failed to parse serialized accounts from data file")
  }

  pub fn to_string(&self) -> Result<String> {
    serde_json::to_string(&self)
      .context("Failed to serialize the accounts for storing in data file")
  }

  pub fn get_code(&self, name: String) -> Result<String> {
    match self.accounts.iter().find(|&account| *account == name) {
      Some(account) => account.get_code(),
      None => Err(anyhow!("Account does not exist")),
    }
  }

  pub fn push(
    &mut self,
    name: String,
    secret: String,
    algorithm: String,
    digits: usize,
    step: u64,
  ) -> Result<()> {
    match self.accounts.iter().find(|&account| *account == name) {
      Some(_account) => Err(anyhow!("Account already exists")),
      None => {
        let account = Account::new(name, algorithm, digits, step);
        account.save_secret(secret)?;
        self.accounts.push(account);
        Ok(())
      }
    }
  }

  pub fn delete(&mut self, name: String) -> Result<()> {
    match self.accounts.iter().position(|account| *account == name) {
      Some(index) => {
        let account = self.accounts.remove(index);
        account.delete_secret()
      }
      None => Err(anyhow!("Account does not exist")),
    }
  }
}
