mod cli;
mod converters;

use clap::Parser;
use cli::{Cli, Commands};
use std::io::{self};

use crate::converters::{IconConverter, LucideConverter};

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Lucide { icons, output } => LucideConverter::generate_templates(icons, output)?,
    }

    Ok(())
}
