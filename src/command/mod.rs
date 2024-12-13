use anyhow::Result;
use clap::Subcommand;

use crate::repository::Repository;

pub mod add;
pub mod delete;
pub mod list;
pub mod label;
pub mod new;
pub mod start;
pub mod stop;
pub mod info;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create new task
    New(new::Cli),
    /// Delete task
    Delete(delete::Cli),
    /// Show todo list
    List(list::Cli),
    /// Show todo list
    Label(label::Cli),
    /// Show todo list
    Info(info::Cli),
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
            Command::Label(cli) => cli.run(repository)?,
            Command::Info(cli) => cli.run(repository)?,
            Command::Start(cli) => cli.run(repository)?,
            Command::Stop(cli) => cli.run(repository)?,
            Command::Add(cli) => cli.run(repository)?,
        }
        Ok(())
    }
}