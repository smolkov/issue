use anyhow::Result;
use clap::Parser;

use crate::data::Pagination;
use crate::repository::Repository;

use crate::utils::{print_issue_info,issue_id};

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
            if self.ids.is_empty() || self.ids.iter().find(|&&index| issue_id(index) == id).is_some() {
                print_issue_info(id+1, issue);
                println!("\n");
            }
        }
        Ok(())
    }
}

