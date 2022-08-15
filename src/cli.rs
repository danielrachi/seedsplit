use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    /// Operation to perform
    #[clap(subcommand)]
    pub command: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Splits seedphrase
    Split {
        /// Seedprhase to split. Seedphrase must have min 12 and max 24 words.
        #[clap(value_parser, required(true), min_values(12), max_values(24))]
        seedphrase: Vec<String>,
    },
    /// Rebuilds seedphrase from keys A and B. Keys must have the same number of words.
    Rebuild {
        /// Key A to rebuild seedphrase
        #[clap(value_parser, required(true), min_values(12), max_values(24))]
        key_a: Vec<String>,
        /// Key B to rebuild seedphrase
        #[clap(
            value_parser,
            required(true),
            last(true),
            min_values(12),
            max_values(24)
        )]
        key_b: Vec<String>,
    },
}
