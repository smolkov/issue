use anyhow::Result;
use clap::Parser;

use crate::data::Pagination;
use crate::repository::Repository;

use crate::utils::{issue_id, print_issue_info};

#[derive(Debug, Parser)]
pub struct Cli {
    /// List of issue IDs to print information about( Leave it empty to print all)
    ids: Vec<usize>,
}

impl Cli {
    pub fn run(&self, repository: &Repository) -> Result<()> {
        let pagination = Pagination {
            offset: None,
            limit: None,
        };
        let issues = repository.list(pagination);
        for (id, issue) in issues.iter().enumerate() {
            if self.ids.is_empty() || self.ids.iter().any(|&index| issue_id(index) == id) {
                print_issue_info(id + 1, issue, repository)?;
                println!("\n");
            }
        }
        Ok(())
    }
}
