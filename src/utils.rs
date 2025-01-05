use std::io::stdout;
use std::str::FromStr;

use anyhow::Result;
use chrono::TimeDelta;
use crossterm::{execute, style, style::Color};

use crate::data::{Issue,Label};
use crate::repository::Repository;

pub const ID: &str = "ID";
pub const AGE: &str = "Age";
pub const DESCRIPTION: &str = "Description";
pub const CREATED: &str = "Created";
pub const UUID: &str = "Uuid";
pub const LABELS: &str = "Labels";
pub const SPEND_TIME: &str = "Spend time";
pub const URG: &str = "Urg.";
pub const NAME: &str = "Name";
pub const VALUE: &str = "Value";

pub const ISSUE_NAMES: [&str; 4] = [ID, DESCRIPTION, CREATED, LABELS];
pub const NAME_WIDTH: usize = 12;

pub const MINUT_IN_SECOND: i64 = 60;
pub const HOUR_IN_SECOND: i64 = MINUT_IN_SECOND * 60;
pub const DAY_IN_SECOND: i64 = HOUR_IN_SECOND * 24;
pub const MONTH_IN_SECOND: i64 = DAY_IN_SECOND * 30;
pub const YEAR_IN_SECOND: i64 = DAY_IN_SECOND * 364;

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

pub fn print_issue_info(id: usize, issue: &Issue, repository: &Repository) -> Result<()> {
    let description_len = issue.title.len();
    let id_len = issue.id.len();
    let label_len = issue.label.join(" ").len();

    let value_width = if description_len > id_len && description_len > label_len {
        description_len
    }else if id_len > label_len {
        id_len
    }else {
        label_len
    };
     
    execute!(
        stdout(),
        style::SetAttribute(style::Attribute::Bold),
        style::SetAttribute(style::Attribute::Underlined),
        style::SetBackgroundColor(Color::Black),
        style::SetForegroundColor(Color::Cyan),
        style::Print(format!("{:<NAME_WIDTH$} {:<value_width$}", NAME, VALUE)),
        style::SetAttribute(style::Attribute::NoUnderline),
        style::ResetColor,
        style::Print("\n"),
    )?;
    execute!(
        stdout(),
        style::Print(format!("{:<NAME_WIDTH$} {}\n", ID, id)),
        style::Print(format!("{:<NAME_WIDTH$} {}\n", DESCRIPTION, issue.title)),
        style::Print(format!(
            "{:<NAME_WIDTH$} {}\n",
            CREATED,
            issue.created.format("%Y.%m.%d %H:%M:%S")
        )),
        style::Print(format!("{:<NAME_WIDTH$} {}\n", UUID, issue.id)),
        // cursor::MoveToNextLine(0)
    )?;
    execute!(
        stdout(),
        style::ResetColor,
        style::Print(format!("{:<NAME_WIDTH$}", LABELS)),
    )?;
    for label in issue.label.iter() {
        let label = repository
            .get_label(label)
            .unwrap_or(Label::new(label, "black", ""));
        execute!(
            stdout(),
            style::ResetColor,
            style::SetBackgroundColor(Color::from_str(&label.color).unwrap_or(Color::Black)),
            style::Print(label.name.as_str()),
            style::ResetColor,
            style::Print(" "),
        )?;
    }
    execute!(stdout(), style::ResetColor, style::Print("\n"),)?;

    execute!(stdout(), style::ResetColor, 
    style::Print(format!("{:<NAME_WIDTH$} {:.2}h\n", SPEND_TIME, issue.spend_time.as_ref().map(|td|td.as_secs() as f64 / HOUR_IN_SECOND as f64 ).unwrap_or(0f64))),
    style::Print("\n"),)?;

    Ok(())
}

pub fn issue_id(id: usize) -> usize {
    if id > 0 {
        id - 1
    } else {
        id
    }
}
