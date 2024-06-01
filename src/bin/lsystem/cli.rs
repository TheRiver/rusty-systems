use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Command
}

#[derive(Debug, Args)]
pub struct DeriveArgs {
    pub file: Box<std::path::Path>, // todo Figure out how to document
    #[arg(short, long)]
    pub output: Box<std::path::Path>,
    #[arg(long, default_value = "500")]
    pub width: usize,
    #[arg(long, default_value = "500")]
    pub height: usize
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Derive(DeriveArgs),
    Describe,
}