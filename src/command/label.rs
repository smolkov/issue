use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::{data::Label, repository::{self, Repository}};
use crate::utils::issue_id;

#[derive(Debug, Parser)]
struct Add {
    /// Issue id
    issue: usize,
    /// Labels to add   bug or type:bug
    labels: Vec<String>,
}

#[derive(Debug, Parser)]
struct Create {
    /// Label name
    label: String,
    /// Color
    color: String,
    /// Description
    description: Vec<String>,
}

#[derive(Debug,Parser)]
/// Show all labels
struct Show{}

#[derive(Debug, Subcommand)]
pub enum  Command {
    /// Add new label to issue 
    Add(Add),
    /// Create new label
    Create(Create), 
    /// Show all labels
    Show(Show),
}

#[derive(Debug,Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub fn run(&self, repository: &mut Repository) -> Result<()> {
        match &self.command {
            Command::Add(cmd) => cmd.run(repository),
            Command::Create(cmd) => cmd.run(repository),
            Command::Show(cmd) => cmd.run(repository),
        }
    }
}

impl Add {
    fn run(&self, repository: &mut Repository) -> Result<()> {
        println!("Add labels {} to {}",self.labels.join(" "),self.issue);
        let mut issue = repository.get_issue(issue_id(self.issue)).ok_or(anyhow::anyhow!("Issue index {} not found",self.issue))?;
        for label in self.labels.iter()  {
            let label = repository.get_label(label).unwrap_or(Label::new(label,"white",""));
            issue.add_label(&label.name);
        }
        println!("{:#?}",issue);
        repository.update_backlog(issue)?;
        repository.save()?;
        Ok(())
    }
}

impl Create {
    fn run(&self, repository: &mut Repository) -> Result<()> {
        println!("create label {} color {} description:{}",self.label,self.color,self.description.join(" "));
        if repository.get_label(&self.label).is_none() {
            let label = Label::new(&self.label, &self.color, self.description.join(" ").as_str());
            repository.add_label(&label);
            repository.save()?;
            Ok(())
        }else {
            println!("Label {} already exist",self.label);
            Ok(())
        }
    }
}


impl Show {
    fn run(&self,repository: &mut Repository) -> Result<()> {
        println!("Show all labels:");
        let max_len = repository.labels().iter().map(|l|l.name.len()).max().unwrap_or(10);
        for label in repository.labels() {
            println!("{:<max_len$} {:<10} {}",label.name,label.color,label.description);
        }
        Ok(())
    }
}