use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "templ-icons")]
#[command(
    about = "CLI tool to generate icon templates for Templ.",
    long_about = "A CLI to generate icon templates from icon libraries like Lucide into Templ templates."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        about = "Generate Templ templates from Lucide icons",
        long_about = "Generate Templ functions for specified Lucide icons and output to a file."
    )]
    Lucide {
        #[arg(
            short,
            long,
            value_delimiter = ',',
            help = "Comma-separated list of Lucide icon names to generate"
        )]
        icons: Vec<String>,

        #[arg(short, long, help = "Path to output the generated Templ template file")]
        output: PathBuf,
    },
}
