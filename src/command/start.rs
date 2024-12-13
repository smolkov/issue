use anyhow::Result;
use clap::Parser;

use crate::repository::Repository;
use crate::utils::issue_id;

#[derive(Debug, Parser)]
pub struct Cli {
    /// New todo title
    issue: usize,
}

impl Cli {
    pub fn run(&self, repository: &mut Repository) -> Result<()> {
       
        repository.start(issue_id(self.issue))?;
        repository.save()?;
        Ok(())
    }
}
