use anyhow::Result;
use chrono::Utc;
use clap::Parser;

use crate::data::Pagination;
use crate::repository::Repository;
use crate::utils::{self, print_age};

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
        let now = Utc::now();
        let title_len = issues
            .iter()
            .map(|s| s.title.len())
            .max()
            .unwrap_or(utils::DESCRIPTION.len());
        println!(
            "{:<4}{:<4} {:<title_len$} {}",
            utils::ID,
            utils::AGE,
            utils::DESCRIPTION,
            utils::URG
        );
        for (id, issue) in issues.iter().enumerate() {
            println!(
                "{:<4}{:<4} {:<title_len$} {}",
                id + 1,
                print_age(now - issue.created),
                issue.title,
                0,
            );
        }
        Ok(())
    }
}
