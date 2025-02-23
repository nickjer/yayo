mod account;
mod accounts;
mod command;
mod data_file;
mod otp;

use anyhow::Result;
use clap::{Parser, crate_name};

use accounts::Accounts;
use command::Command;
use data_file::DataFile;

fn main() -> Result<()> {
  let command = Command::parse();

  let data_file = DataFile::new(crate_name!())?;
  let mut accounts = match data_file.read()? {
    Some(string) => Accounts::from_string(&string)?,
    None => Accounts::new(),
  };

  command.sub_command.call(&mut accounts)?;

  data_file.write(&accounts.to_string()?)?;
  Ok(())
}
