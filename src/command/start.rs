use anyhow::Result;
use clap::Parser;

use crate::repository::Repository;
use crate::utils::issue_id;

#[derive(Debug, Parser)]
pub struct Cli {
    /// All Issue id you want to stop working timer
    issue: Vec<usize>,
}

impl Cli {
    pub fn run(&self, repository: &mut Repository) -> Result<()> {
        for id in self.issue.iter() {
            if let Some(mut issue) = repository.get_issue(issue_id(*id)) {
                issue.start();    
                repository.update_backlog(issue)?;
            }
        }
        Ok(())
    }
}
