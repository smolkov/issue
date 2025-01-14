use std::time::Duration;

use anyhow::Result;
use chrono::{DateTime, Utc};
use chrono::prelude::*;
use clap::Parser;
use regex::Regex;

use crate::repository::Repository;
use crate::utils::issue_id;

#[derive(Debug, Parser)]
pub struct Cli {
    /// Issue id
    issue: usize,
    /// Time entry Example: 1h 30m
    entry: Vec<String>,
}

impl Cli {
    pub fn run(&self, repository: &mut Repository) -> Result<()> {
        let mut issue = repository
            .get_issue(issue_id(self.issue))
            .ok_or(anyhow::anyhow!("Issue index {} not found", self.issue))?;
        let entry = WorkEntry::new(&mut self.entry.clone());
        println!(
            "add work entry for issue:{} {} {}",
            self.issue,
            entry.date.to_string(),
            entry.dur.as_secs()
        );
        issue.spend_time(entry.dur);
        repository.add_time_entry(&issue, entry.date, entry.dur)?;
        repository.update_backlog(issue)?;
        repository.save()?;
        Ok(())
    }
}

struct WorkEntry {
    date: DateTime<Utc>,
    dur: Duration,
}

impl WorkEntry {
    fn date(entry: &mut Vec<String>) -> DateTime<Utc> {
        if let Some(date) = entry.first() {
            if let Some(date) = parse_date(date.as_str()) {
                let _ = entry.remove(0);
                println!("add date:{}", date.to_rfc3339());
                return date;
            }
        }
        Utc::now()
    }
    fn seconds(entry: &mut Vec<String>) -> u64 {
        let mut seconds = 0;

        for t in entry.iter() {
            match t.chars().last() {
                Some('m') => {
                    let _ = t[..t.len() - 1]
                        .parse::<u64>()
                        .map(|minuts| seconds += minuts * 60);
                }
                Some('h') => {
                    let _ = t[..t.len() - 1]
                        .parse::<u64>()
                        .map(|hours| seconds += hours * 60 * 60);
                }
                _ => {
                    let _ = t[..t.len() - 1].parse::<u64>().map(|sec| seconds += sec);
                }
            }
        }
        seconds
    }
    fn new(entry: &mut Vec<String>) -> WorkEntry {
        let date = Self::date(entry);
        let dur = Duration::from_secs(Self::seconds(entry));
        WorkEntry { date, dur }
    }
}

fn parse_date(date: &str) -> Option<DateTime<Utc>> {
    let re = Regex::new(r"(?<y>[0-9]{4})-(?<m>[0-9]{2})-(?<d>[0-9]{2})").unwrap();
    if re.is_match(date) {
        if let Some(caps) = re.captures(date) {
            let y = caps.name("y")?.as_str().parse::<i32>().ok()?;
            let m = caps.name("m")?.as_str().parse::<u32>().ok()?;
            let d = caps.name("d")?.as_str().parse::<u32>().ok()?;
            let ndt = NaiveDate::from_ymd_opt(y, m, d)?.and_hms_opt(9, 0, 0)?;
            let dt:DateTime<Utc> = DateTime::from_naive_utc_and_offset(ndt,Utc);
            return Some(dt)
        }
    }
    None
    // parse date used regex.
}
