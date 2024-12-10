use anyhow::Result;
use clap::Subcommand;

use crate::repository::Repository;

pub mod add;
pub mod delete;
pub mod list;
pub mod new;
pub mod start;
pub mod stop;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create new task
    New(new::Cli),
    /// Delete task
    Delete(delete::Cli),
    /// Show todo list
    List(list::Cli),
    /// Start working on issue
    Start(start::Cli),
    /// Stop working on issue
    Stop(stop::Cli),
    /// Add time entry to issue
    Add(add::Cli),
}

impl Command {
    pub fn run(&self, repository: &mut Repository) -> Result<()> {
        match self {
            Command::New(cli) => cli.run(repository)?,
            Command::Delete(cli) => cli.run(repository)?,
            Command::List(cli) => cli.run(repository)?,
            Command::Start(cli) => cli.run(repository)?,
            Command::Stop(cli) => cli.run(repository)?,
            Command::Add(cli) => cli.run(repository)?,
        }
        Ok(())
    }
}
