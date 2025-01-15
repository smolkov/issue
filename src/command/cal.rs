use anyhow::Result;
// use chrono::Utc;
use clap::Parser;
// use crossterm::{execute, style, style::Color};
// use std::io::stdout;

use crate::repository::Repository;
// use crate::utils::{self, print_age};

#[derive(Debug, Parser)]
pub struct Cli {
    // Offset
    // offset: Option<u32>,
    // Limit
    // limit: Option<u32>,
}

impl Cli {
    pub fn run(&self, repository: &Repository) -> Result<()> {
        println!("show calender information");
        let time_entry = repository.working();
        for entry in time_entry {
            println!("ID:[{}] {}:{}h",entry.id,entry.date.to_string(),entry.duration.as_secs_f64()/3600.0)
        }
        Ok(())
    }
}
