use clap::Parser;
use seedsplit::Cli;

fn main() {
    let cli = Cli::parse();
    let ans = seedsplit::run(cli);
    if let Err(e) = ans {
        eprintln!("\x1b[31mError\x1b[0m: {}", e);
    }
}
