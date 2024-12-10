use once_cell::sync::Lazy;
use std::{
    fs,
    path::{Path, PathBuf},
};

const BACKLOG: &str = "backlog.json";
const WORKING: &str = "working.json";
const CONFIG: &str = "config.toml";

pub static WORKSPACE: Lazy<Workspace> = Lazy::new(|| Workspace::new());

pub struct Workspace {
    directory: PathBuf,
    backlog: PathBuf,
    working: PathBuf,
    config: PathBuf,
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
        Workspace {
            directory,
            backlog,
            working,
            config
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
}
