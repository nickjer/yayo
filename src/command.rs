mod add;
mod delete;
mod list;
mod view;

use anyhow::Result;
use clap::{crate_authors, crate_description, crate_version, Clap};

use crate::accounts::Accounts;
use add::Add;
use delete::Delete;
use list::List;
use view::View;

#[derive(Clap)]
pub enum SubCommand {
  Add(Add),
  Delete(Delete),
  List(List),
  View(View),
}

impl SubCommand {
  pub fn call(self, accounts: &mut Accounts) -> Result<()> {
    match self {
      SubCommand::Add(sub_command) => sub_command.call(accounts),
      SubCommand::Delete(sub_command) => sub_command.call(accounts),
      SubCommand::List(sub_command) => sub_command.call(accounts),
      SubCommand::View(sub_command) => sub_command.call(accounts),
    }
  }
}

#[derive(Clap)]
#[clap(
  version = crate_version!(),
  author = crate_authors!(),
  about = crate_description!()
)]
pub struct Command {
  #[clap(subcommand)]
  pub sub_command: SubCommand,
}
