use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install asked binary
    Install {
        /// The source link to download from
        target: String,
    },
    /// Update your binary except the one that has been froozen
    Update {
        /// The source link to download from
        target: String,
    },
    /// Get taret info
    Info {
        /// The source link to download from
        target: String,
    },
    /// List binary in config
    List {

    },
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}
