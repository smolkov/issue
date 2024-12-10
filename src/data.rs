use std::time::Duration;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Issue {
    pub id: String,
    pub title: String,
    pub content: String,
    pub label: Vec<String>,
    pub created: DateTime<Utc>,
    pub started: Option<DateTime<Utc>>,
}

impl Issue {
    pub fn new(title: &str) -> Issue {
        let id = Uuid::new_v4().to_string();
        Issue {
            id,
            title: title.to_owned(),
            content: "".to_owned(),
            label: Vec::new(),
            created: Utc::now(),
            started: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TimeEntry {
    pub id: String,
    pub date: DateTime<Utc>,
    pub duration: Duration,
}
