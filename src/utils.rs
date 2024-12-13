use chrono::TimeDelta;

use crate::data::Issue;

pub const ID: &'static str = "ID";
pub const AGE: &'static str = "Age";
pub const DESCRIPTION: &'static str = "Description";
pub const CREATED: &'static str = "Created";
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

pub fn print_issue_info(id: usize, issue: &Issue) {
    println!("{:<NAME_WIDTH$} {}", NAME, VALUE);
    println!("{:<NAME_WIDTH$} {}", ID, id);
    println!("{:<NAME_WIDTH$} {}", DESCRIPTION, issue.title);
    println!(
        "{:<NAME_WIDTH$} {}",
        CREATED,
        issue.created.format("%Y.%m.%d %H:%M:%S")
    );
    println!("{:<NAME_WIDTH$} {}", LABELS, issue.label.join(" "));
}

pub fn issue_id(id: usize) -> usize {
    if id > 0 {
        id - 1
    } else {
        id
    }
}
