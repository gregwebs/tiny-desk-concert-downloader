mod scraper;
mod archive_scraper;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scrape a single Tiny Desk Concert page
    Scrape {
        /// URL of the Tiny Desk Concert page
        url: String,
    },
    /// Scrape the archive for a specific time period
    Archive {
        /// Year in YYYY format
        year: String,
        /// Month in MM format
        month: String,
        /// Optional day in DD format
        day: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Scrape { url } => {
            scraper::scrape_data(url)
        },
        Commands::Archive { year, month, day } => {
            archive_scraper::scrape_archive(year, month, day.as_deref())
        },
    }
}
