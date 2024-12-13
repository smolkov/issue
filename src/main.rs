use anyhow::Result;
use clap::Parser;

use issue::cli::Args;
use issue::config::Config;
use issue::repository::Repository;

fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::load().unwrap_or_default();
    let mut repository = Repository::new(config);
    if let Err(e) = repository.load() {
        return Err(anyhow::anyhow!("Load repository error - {e}"));
    }
    args.command.run(&mut repository)?;
    Ok(())
}
