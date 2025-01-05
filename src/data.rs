use std::time::Duration;

use anyhow::Result;
use chrono::{prelude::*, TimeDelta};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::HOUR_IN_SECOND;
use crate::repository::Repository;

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
    pub spend_time: Option<Duration>,
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
            spend_time: None,
        }
    }

    pub fn add_label(&mut self, label: &str) {
        //TODO: create copy from string.
        if self.label.contains(&label.to_string()) {
            return;
        }
        if let Some((scope, _)) = label.split_once(":") {
            if let Some(pos) = self.label.iter_mut().position(|l| l.starts_with(scope)) {
                self.label[pos] = label.to_owned();
            } else {
                self.label.push(label.to_owned());
            }
        } else {
            self.label.push(label.to_owned());
        }
    }

    pub fn spend_time(&mut self, duration: Duration) {
        if let Some(st) = self.spend_time.as_mut() {
            *st = duration;
        } else {
            self.spend_time = Some(duration);
        }
    }

    pub fn start(&mut self) {
        self.started = Some(Utc::now());
    }

    pub fn stop(&mut self, repository: &mut Repository) -> Result<()> {
        if let Some(start_time) = self.started {
            let diff = Utc::now() - start_time;
            let sec = if diff.num_seconds() < 0 {
                0_u64
            } else {
                // a working session can't be longer as 8 h
                if diff.num_seconds() > 8 * HOUR_IN_SECOND {
                    8 * HOUR_IN_SECOND as u64
                } else {
                    diff.num_seconds() as u64 
                }
            };
            self.spend_time(Duration::from_secs(sec));
            repository.add_time_entry(self, start_time, Duration::from_secs(sec))?;
            self.started = None;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TimeEntry {
    pub id: String,
    pub date: DateTime<Utc>,
    pub duration: Duration,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Label {
    pub name: String,
    pub color: String,
    pub description: String,
}

impl Label {
    pub fn new(name: &str, color: &str, description: &str) -> Label {
        Label {
            name: name.to_owned(),
            color: color.to_owned(),
            description: description.to_owned(),
        }
    }
}
