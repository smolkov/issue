use once_cell::sync::Lazy;
use std::{
    fs,
    path::{Path, PathBuf},
};

const BACKLOG: &str = "backlog.json";
const WORKING: &str = "working.json";
const LABELS: &str = "labels.json";
const CONFIG: &str = "config.toml";

pub static WORKSPACE: Lazy<Workspace> = Lazy::new(Workspace::new);

pub struct Workspace {
    directory: PathBuf,
    backlog: PathBuf,
    working: PathBuf,
    config: PathBuf,
    labels: PathBuf,
}

impl Workspace {
    pub fn new() -> Workspace {
        let directory = dirs::config_dir()
            .unwrap_or(PathBuf::from("."))
            .join(".issue");
        if !directory.is_dir() {
            if let Err(e) = fs::create_dir_all(&directory) {
                panic!("Create workspace directory error - {}", e);
            }
        }
        let backlog = directory.join(BACKLOG);
        let working = directory.join(WORKING);
        let config = directory.join(CONFIG);
        let labels = directory.join(LABELS);
        if !backlog.is_file() {
            if let Err(e) = fs::write(&backlog, "[]") {
                panic!("Create empty backlog error - {}", e);
            }
        }
        if !working.is_file() {
            if let Err(e) = fs::write(&working, "[]") {
                panic!("Create empty backlog work log error - {}", e);
            }
        }
        if !labels.is_file() {
            if let Err(e) = fs::write(&labels, "[]") {
                panic!("Create empty labels error - {}", e);
            }
        }
        Workspace {
            directory,
            backlog,
            working,
            config,
            labels,
        }
    }
    pub fn directory(&self) -> &Path {
        &self.directory
    }
    pub fn backlog(&self) -> &Path {
        &self.backlog
    }
    pub fn working(&self) -> &Path {
        &self.working
    }
    pub fn config(&self) -> &Path {
        &self.config
    }
    pub fn labels(&self) -> &Path {
        &self.labels
    }
}


impl Default for Workspace {
    fn default() -> Self {
        Self::new()
    }
}