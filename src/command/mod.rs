use anyhow::Result;
use clap::{Subcommand,Parser};
use clap::CommandFactory;
use clap_complete::{shells, Generator, Shell};
use crate::repository::Repository;
use crate::cli::Args;

pub mod add;
pub mod delete;
pub mod info;
pub mod label;
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
    /// Generate shell completions
    Completions(Completions),
}

#[derive(Debug, Subcommand)]
pub enum  Shells {
    /// bash completions
   Bash,
    /// zsh completions
   Zsh, 
   /// fish completions
   Fish,
}
#[derive(Debug, Parser)]
pub struct Completions {
    #[command(subcommand)]
    shells: Shells
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
            Command::Completions(shells) => match shells.shells {
                Shells::Bash => generate_completions(shells::Bash),
                Shells::Zsh => generate_completions(shells::Zsh),
                Shells::Fish => generate_completions(shells::Fish),
            },

        }
        Ok(())
    }
}


fn generate_completions<G:Generator>(generator:G) {
    let mut stdout = std::io::stdout();
    let mut args = Args::command();
    clap_complete::generate(
            generator,
            &mut args,
            "issue".to_string(),
            &mut stdout, 
    ); 
}