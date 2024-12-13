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
    pub fn add_label(&mut self,label: &str) {
        //TODO: create copy from string.
        if self.label.contains(&label.to_string()) {
            return;
        }
        if let Some((scope,_)) = label.split_once(":") {
            if let Some(pos)=self.label.iter_mut().position(|l| l.starts_with(scope)) {
                self.label[pos] = label.to_owned();
            }
        }else {
            self.label.push(label.to_owned());
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TimeEntry {
    pub id: String,
    pub date: DateTime<Utc>,
    pub duration: Duration,
}


#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct Label {
    pub name: String,
    pub color: String,
    pub description: String,

}

impl Label {
    pub fn new(name:&str,color:&str,description:&str) -> Label {
        Label{
            name: name.to_owned(),
            color: color.to_owned(),
            description: description.to_owned(),
        }
    }
}