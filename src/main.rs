use anyhow::Result;
use clap::Parser;

use issue::cli::Args;
use issue::config::Config;
use issue::repository::Repository;

fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::load().unwrap_or_default();
    let mut repository = Repository::new(config);
    if repository.load().is_err() {
        println!("First time start? issue repository don't exist( create a new issue repository)");
        repository.save()?;
    }
    args.command.run(&mut repository)?;
    Ok(())
}
