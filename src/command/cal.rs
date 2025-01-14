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
    pub fn run(&self, _repository: &Repository) -> Result<()> {
        println!("show calender information");
        Ok(())
    }
}
