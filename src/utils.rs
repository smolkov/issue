use std::io;
use std::io::stdout;
use std::str::FromStr;

use chrono::TimeDelta;
use anyhow::Result;
use crossterm::{execute, style, style::Color};

use crate::data::Issue;
use crate::repository::Repository;

pub const ID: &'static str = "ID";
pub const AGE: &'static str = "Age";
pub const DESCRIPTION: &'static str = "Description";
pub const CREATED: &'static str = "Created";
pub const UUID: &'static str = "Uuid";
pub const LABELS: &'static str = "Labels";
pub const URG: &'static str = "Urg.";
pub const NAME: &'static str = "Name";
pub const VALUE: &'static str = "Value";

pub const ISSUE_NAMES: [&'static str; 4] = [ID, DESCRIPTION, CREATED, LABELS];
pub const NAME_WIDTH: usize = 12;

const MINUT_IN_SECOND: i64 = 60;
const HOUR_IN_SECOND: i64 = MINUT_IN_SECOND * 60;
const DAY_IN_SECOND: i64 = HOUR_IN_SECOND * 24;
const MONTH_IN_SECOND: i64 = DAY_IN_SECOND * 30;
const YEAR_IN_SECOND: i64 = DAY_IN_SECOND * 364;

pub fn print_age(timedelta: TimeDelta) -> String {
    match timedelta.num_seconds() {
        sec if sec > YEAR_IN_SECOND => format!("{}y", sec / YEAR_IN_SECOND),
        sec if sec > MONTH_IN_SECOND => format!("{}mo", sec / MONTH_IN_SECOND),
        sec if sec > DAY_IN_SECOND => format!("{}d", sec / DAY_IN_SECOND),
        sec if sec > HOUR_IN_SECOND => format!("{}h", sec / HOUR_IN_SECOND),
        sec if sec > MINUT_IN_SECOND => format!("{}m", sec / MINUT_IN_SECOND),
        sec => format!("{}s", sec),
    }
}



pub fn print_issue_info(id: usize, issue: &Issue,repository: &Repository) -> Result<()>{
    execute!(
        stdout(),
        style::SetAttribute(style::Attribute::Bold),
        style::SetAttribute(style::Attribute::Underlined),
        style::SetBackgroundColor(Color::Black),
        style::SetForegroundColor(Color::Cyan),
        style::Print(format!("{:<NAME_WIDTH$} {}", NAME, VALUE)),
        style::SetAttribute(style::Attribute::NoUnderline),
        style::ResetColor,
        style::Print("\n"),
        // cursor::MoveToNextLine(0)
    )?;
    execute!(
        stdout(),
        style::Print(format!("{:<NAME_WIDTH$} {}\n", ID, id)),
        style::Print(format!("{:<NAME_WIDTH$} {}\n", DESCRIPTION, issue.title)),
        style::Print(format!("{:<NAME_WIDTH$} {}\n", CREATED, issue.created.format("%Y.%m.%d %H:%M:%S"))),
        style::Print(format!("{:<NAME_WIDTH$} {}\n", UUID, issue.id)),
        // cursor::MoveToNextLine(0)
    )?;
    execute!(
        stdout(), 
        style::ResetColor,
        style::Print(format!("{:<NAME_WIDTH$}", LABELS)), 
    )?;
    for label in issue.label.iter() {
        if let Some(label) = repository.get_label(label) {
            execute!(
                stdout(), 
                style::ResetColor,
                style::SetBackgroundColor(Color::from_str(&label.color).unwrap_or(Color::Black)), 
                style::Print(format!("{}",label.name)), 
                style::ResetColor,
            )?;
        }
    }
    execute!(
        stdout(), 
        style::ResetColor,
        style::Print("\n"), 
    )?;
    Ok(())
}

pub fn issue_id(id: usize) -> usize {
    if id > 0 {
        id - 1
    } else {
        id
    }
}
