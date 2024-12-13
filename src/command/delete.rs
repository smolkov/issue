use anyhow::Result;
use clap::Parser;

use crate::repository::Repository;

#[derive(Debug, Parser)]
pub struct Cli {
    /// issues id to delete
    ids: Vec<u32>,
}

impl Cli {
    pub fn run(&self, repository: &mut Repository) -> Result<()> {
        let mut ids = self.ids.clone();
        ids.sort();
        for id in ids.iter().rev() {
            let id = if *id > 0 { *id - 1 } else { *id };

            if repository.delete(id as usize).is_err() {
                println!("issue {} don't exist in repository", id);
            }
        }
        repository.save()?;
        Ok(())
    }
}
