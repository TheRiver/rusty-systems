use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
#[command(propagate_version = true)]
#[clap(
    name = "lsystem", 
    about="Create SVGs from L-Systems. \nSee https://theriver.github.io/rusty-systems/ for more"
)]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Command
}

#[derive(Debug, Args)]
pub struct InterpretationArgs {
    /// The input file, eg: a plant file
    pub file: Box<std::path::Path>, 
    /// Where the SVG file should be saved 
    #[arg(short, long, default_value = "out.svg")]
    pub output: Box<std::path::Path>,
    /// The image width
    #[arg(long, default_value = "500")]
    pub width: usize,
    #[arg(long, default_value = "500")]
    /// The image height
    pub height: usize
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Derive and interpret an SVG of a production string
    Interpret(InterpretationArgs),
    /// Describe the tokens available for use
    Describe,
}