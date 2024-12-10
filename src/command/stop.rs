use anyhow::Result;
use clap::Parser;

use crate::repository::Repository;

#[derive(Debug, Parser)]
pub struct Cli {
    /// New todo title
    issue: usize,
}

impl Cli {
    pub fn run(&self, repository: &mut Repository) -> Result<()> {
        let id = if self.issue > 0 {
            self.issue - 1
        } else {
            self.issue
        };
        repository.stop(id)?;
        repository.save()?;
        Ok(())
    }
}
