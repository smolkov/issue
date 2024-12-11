use anyhow::Result;
use chrono::{TimeDelta, Utc};
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
        let now = Utc::now();
        let title_len = issues.iter().map(|s| s.title.len()).max().unwrap_or(10);
        println!(
            "{:<4}{:<4} {:<title_len$} {}",
            "ID", "Age", "Description", "Urg."
        );
        for (id, issue) in issues.iter().enumerate() {
            println!(
                "{:<4}{:<4} {}",
                id + 1,
                print_age(now - issue.created),
                issue.title
            );
        }
        Ok(())
    }
}

const MINUT_IN_SECOND: i64 = 60;
const HOUR_IN_SECOND: i64 = MINUT_IN_SECOND * 60;
const DAY_IN_SECOND: i64 = HOUR_IN_SECOND * 24;
const MONTH_IN_SECOND: i64 = DAY_IN_SECOND * 30;
const YEAR_IN_SECOND: i64 = DAY_IN_SECOND * 364;

fn print_age(timedelta: TimeDelta) -> String {
    match timedelta.num_seconds() {
        sec if sec > YEAR_IN_SECOND => format!("{}y", sec / YEAR_IN_SECOND),
        sec if sec > MONTH_IN_SECOND => format!("{}mo", sec / MONTH_IN_SECOND),
        sec if sec > DAY_IN_SECOND => format!("{}d", sec / DAY_IN_SECOND),
        sec if sec > HOUR_IN_SECOND => format!("{}h", sec / HOUR_IN_SECOND),
        sec if sec > MINUT_IN_SECOND => format!("{}m", sec / MINUT_IN_SECOND),
        sec => format!("{}s", sec),
    }
}
