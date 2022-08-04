use clap::Parser;
use seedsplit::Cli;

fn main() {
    let cli = Cli::parse();
    let ans = seedsplit::run(cli);
    if let Err(e) = ans {
        eprintln!("Error: {}", e);
    }
}
