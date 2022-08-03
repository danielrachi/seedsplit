use clap::Parser;
use seedsplit::Cli;

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    seedsplit::run(cli)?;
    Ok(())
}
