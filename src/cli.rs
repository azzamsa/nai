use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use clap_verbosity_flag::Verbosity;

#[derive(Parser)]
#[command(
    name = "nai",
    version,
    about = "Nai ⌚. \nMeasure the duration of meaningful pursuits",
    after_long_help = "Bugs can be reported on GitHub: https://github.com/azzamsa/nai/issues"
)]
pub struct Opts {
    /// Specify an alternate configuration file
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Declare wnhen to use colors
    #[arg(
        short,
        long,
        value_enum,
        default_value_t = Color::Auto,
        help = "When to use colors",
        long_help,
    )]
    pub color: Color,

    /// Use verbose output
    #[command(flatten)]
    pub verbose: Verbosity,
}

#[derive(Clone, ValueEnum)]
pub enum Color {
    /// show colors if the output goes to an interactive console (default)
    Auto,
    /// always use colorized output
    Always,
    /// do not use colorized output
    Never,
}

impl Color {
    pub fn as_str(&self) -> &'static str {
        match self {
            Color::Auto => "auto",
            Color::Never => "never",
            Color::Always => "always",
        }
    }
}
