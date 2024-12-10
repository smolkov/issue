use anyhow::Result;
use clap::Parser;

use crate::data::Pagination;
use crate::repository::Repository;

#[derive(Debug, Parser)]
pub struct Cli {
    /// Offset
    offset: Option<u32>,
    /// Limit
    limit: Option<u32>,
}

impl Cli {
    pub fn run(&self, repository: &Repository) -> Result<()> {
        let pagination = Pagination {
            offset: self.offset,
            limit: self.limit,
        };
        let issues = repository.list(pagination);
        for (id, issue) in issues.iter().enumerate() {
            println!("{} {}", id + 1, issue.title);
        }
        Ok(())
    }
}
