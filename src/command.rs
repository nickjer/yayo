mod add;
mod completion;
mod delete;
mod list;
mod view;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::accounts::Accounts;
use add::Add;
use completion::Completion;
use delete::Delete;
use list::List;
use view::View;

#[derive(Subcommand, Debug)]
pub enum SubCommand {
  Add(Add),
  Completion(Completion),
  Delete(Delete),
  List(List),
  View(View),
}

impl SubCommand {
  pub fn call(self, accounts: &mut Accounts) -> Result<()> {
    match self {
      SubCommand::Add(sub_command) => sub_command.call(accounts),
      SubCommand::Completion(sub_command) => sub_command.call(),
      SubCommand::Delete(sub_command) => sub_command.call(accounts),
      SubCommand::List(sub_command) => sub_command.call(accounts),
      SubCommand::View(sub_command) => sub_command.call(accounts),
    }
  }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Command {
  #[clap(subcommand)]
  pub sub_command: SubCommand,
}
