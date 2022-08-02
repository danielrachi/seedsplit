use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// Operation to perform
    #[clap(subcommand)]
    command: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
    /// Splits seedphrase
    Split {
        /// Seedprhase to split
        #[clap(value_parser, required(true))]
        seedphrase: Vec<String>,
    },
    /// Rebuilds seedphrase from keys A and B
    Rebuild {
        /// Key A to rebuild seedphrase
        #[clap(value_parser, required(true))]
        key_a: Vec<String>,
        /// Key B to rebuild seedphrase
        #[clap(value_parser, required(true), last(true))]
        key_b: Vec<String>,
    },
}
fn main() {
    let cli = Cli::parse();
    match cli.command {
        Subcommands::Split { seedphrase } => println!("{:?}", seedphrase),
        Subcommands::Rebuild { key_a, key_b } => {
            println!("key a: {:?}", key_a);
            println!("key b: {:?}", key_b);
        }
    }
}
