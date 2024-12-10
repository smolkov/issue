use anyhow::Result;
use clap::Parser;

use crate::repository::Repository;

#[derive(Debug, Parser)]
pub struct Cli {
    /// Issue id
    issue: usize,
    /// Time entry Example: 1h 30m
    entry: Vec<String>,
}



impl Cli {
    pub fn run(&self, repository: &mut Repository) -> Result<()> {
		let issue = repository.get(self.issue).ok_or(anyhow::anyhow!("Issue index {} not found",self.issue))?;

        let id = if self.issue > 0 {
            self.issue - 1
        } else {
            self.issue
        };
        repository.start(id)?;
        repository.save()?;
        Ok(())
    }
	fn seconds (&self) -> u64{
		let mut seconds = 0;

		for t in self.entry.iter() {
			match t.chars().last() {
				Some('m') => { let _ = t[..t.len()-1].parse::<u64>().map(|minuts|seconds += minuts*60);},
				Some('h') => { let _ = t[..t.len()-1].parse::<u64>().map(|hours| seconds += hours*60*60);},
				_ => {let _  = t[..t.len()-1].parse::<u64>().map(|sec|seconds += sec);}
				
			}
		}
		seconds
	}
}
