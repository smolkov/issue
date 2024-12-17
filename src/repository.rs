use std::cmp::PartialOrd;
use std::fs;
use std::time::Duration;

use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::config::Config;
use crate::data::{Issue, Label, Pagination, TimeEntry};

use crate::workspace::WORKSPACE;

#[derive(Debug)]
pub struct Repository {
    pub backlog: Vec<Issue>,
    pub working: Vec<TimeEntry>,
    pub labels: Vec<Label>,
}

impl Repository {
    pub fn new(_config: Config) -> Repository {
        Repository {
            backlog: Vec::new(),
            working: Vec::new(),
            labels: Vec::new(),
        }
    }
    pub fn backlog(&self) -> &Vec<Issue> {
        &self.backlog
    }
    pub fn working(&self) -> &Vec<TimeEntry> {
        &self.working
    }
    pub fn labels(&self) -> &Vec<Label> {
        &self.labels
    }
    pub fn add_backlog(&mut self, todo: &Issue) {
        self.backlog.push(todo.clone())
    }
    pub fn add_label(&mut self, label: &Label) {
        self.labels.push(label.clone())
    }
    pub fn add_time_entry(
        &mut self,
        issue: &Issue,
        date: DateTime<Utc>,
        duration: Duration,
    ) -> Result<()> {
        let entry = TimeEntry {
            id: issue.id.clone(),
            date,
            duration,
        };
        self.working.push(entry);
        self.working.sort_by(|a, b| {
            a.date
                .partial_cmp(&b.date)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Ok(())
    }
    pub fn delete(&mut self, id: usize) -> Result<Issue> {
        if id >= self.backlog.len() {
            return Err(anyhow::anyhow!(
                "issue repository can't remove id{} - max size {}",
                id,
                self.backlog.len()
            ));
        }
        Ok(self.backlog.remove(id))
    }
    pub fn get_issue(&self, id: usize) -> Option<Issue> {
        self.backlog.get(id).cloned()
    }
    pub fn get_label(&self, label: &str) -> Option<Label> {
        self.labels.iter().find(|&l| l.name == label).cloned()
    }
    pub fn list(&self, pagination: Pagination) -> Vec<Issue> {
        let mut todos = Vec::new();
        let offset = pagination.offset.unwrap_or(0);
        let mut limit = pagination.limit.unwrap_or(u32::MAX);

        for todo in self.backlog.iter().skip(offset as usize) {
            if limit > 0 {
                todos.push(todo.clone());
            } else {
                break;
            }
            limit -= 1;
        }
        todos
    }
    pub fn save(&self) -> Result<()> {
        fs::write(WORKSPACE.backlog(), serde_json::to_string(&self.backlog)?)?;
        fs::write(WORKSPACE.working(), serde_json::to_string(&self.working)?)?;
        fs::write(WORKSPACE.labels(), serde_json::to_string(&self.labels)?)?;
        Ok(())
    }
    pub fn load(&mut self) -> Result<()> {
        if !WORKSPACE.backlog().is_file() {
            if let Err(e) = fs::write(&WORKSPACE.backlog(), "[]") {
                panic!("Create empty backlog error - {}", e);
            }
        }
        if !WORKSPACE.working().is_file() {
            if let Err(e) = fs::write(&WORKSPACE.working(), "[]") {
                panic!("Create empty backlog work log error - {}", e);
            }
        }
        if !WORKSPACE.labels().is_file() {
            if let Err(e) = fs::write(&WORKSPACE.labels(), "[]") {
                panic!("Create empty labels error - {}", e);
            }
        }
        self.backlog = serde_json::from_str(fs::read_to_string(WORKSPACE.backlog())?.as_str())?;
        self.working = serde_json::from_str(fs::read_to_string(WORKSPACE.working())?.as_str())?;
        self.labels = serde_json::from_str(fs::read_to_string(WORKSPACE.labels())?.as_str())?;
        Ok(())
    }
    pub fn update_backlog(&mut self, issue: Issue) -> Result<()> {
        if let Some(i) = self.backlog.iter_mut().find(|i| i.id == issue.id) {
            *i = issue;
        }
        Ok(())
    }
}
