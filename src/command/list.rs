use anyhow::Result;
use chrono::Utc;
use clap::Parser;
use crossterm::style::style;
use crossterm::{execute, style, style::Color};
use std::io::stdout;

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

        execute!(
            stdout(),
            style::SetAttribute(style::Attribute::Bold),
            style::SetAttribute(style::Attribute::Underlined),
            // style::SetBackgroundColor(Color::Black),
            style::SetForegroundColor(Color::Cyan),
            style::Print(format!(
                "{:<4}{:<4} {:<title_len$} {}\n",
                utils::ID,
                utils::AGE,
                utils::DESCRIPTION,
                utils::URG
            )),
            style::SetAttribute(style::Attribute::NoUnderline),
            style::ResetColor,
            // cursor::MoveToNextLine(0)
        )?;
        for (id, issue) in issues.iter().enumerate() {
            let background = if issue.started.is_some() {
                Color::DarkYellow
            }else {
                Color::Reset
            };
            execute!(
                stdout(),
                style::SetAttribute(style::Attribute::Bold),
                style::SetBackgroundColor(background),
                style::SetForegroundColor(Color::Cyan),
                style::Print(format!("{:<4}", id + 1,)),
                style::SetAttribute(style::Attribute::NoBold),
                style::SetForegroundColor(Color::Reset),
                // cursor::MoveToNextLine(0)
            )?;
          
            execute!(
                stdout(),
                style::SetBackgroundColor(background),
                style::Print(format!(
                    "{:<4} {:<title_len$} {}",
                    print_age(now - issue.created),
                    issue.title,
                    0
                )),
                style::SetForegroundColor(Color::Reset),
                style::ResetColor,
                // cursor::MoveToNextLine(0)
            )?;
            execute!( stdout(),style::ResetColor,style::Print("\n"))?;
        }
        Ok(())
    }
}
