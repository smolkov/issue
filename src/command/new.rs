use anyhow::Result;
use clap::Parser;

use crate::data::Issue;
use crate::repository::Repository;

#[derive(Debug, Parser)]
pub struct Cli {
    /// New todo title
    title: Vec<String>,
}

impl Cli {
    pub fn run(&self, repository: &mut Repository) -> Result<()> {
        let title = self.title.join(" ");
        let issue = Issue::new(&title);
        repository.add(&issue);
        repository.save()?;
        Ok(())
    }
}
