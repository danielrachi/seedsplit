use clap::Parser;
use seedsplit::{run, Cli};

fn main() {
    let cli = Cli::parse();
    run(cli);
}
