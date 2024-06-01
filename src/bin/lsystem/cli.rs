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
pub struct DeriveArgs {
    /// A plant file
    /// 
    /// It should have the format
    /// 
    /// ```
    /// n = 6
    /// delta = 22.5
    /// 
    /// initial: X
    /// 
    /// Forward -> Forward Forward
    /// X -> Forward + [ [ X ] - X ] - Forward [ - Forward X ] + X
    /// ```
    pub file: Box<std::path::Path>, 
    /// Where the SVG file should be saved 
    #[arg(short, long)]
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
    /// Derive an SVG of a production string
    Derive(DeriveArgs),
    /// Describe the tokens available for use
    Describe,
}