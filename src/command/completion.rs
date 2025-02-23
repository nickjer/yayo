use anyhow::Result;
use clap::{Args, CommandFactory, crate_name};
use clap_complete::{Shell, generate};

use super::Command;

/// Output shell completion
#[derive(Args, Debug)]
pub struct Completion {
  /// Name of the shell
  pub shell: Shell,
}

impl Completion {
  pub fn call(self) -> Result<()> {
    let mut cmd = Command::command();
    generate(self.shell, &mut cmd, crate_name!(), &mut std::io::stdout());
    Ok(())
  }
}
