use clap::Parser;
use crate::domain::Direction;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(default_value = "next")]
    /// Direction to switch workspace ('next' or 'prev[ious]')
    pub direction: Direction,
}

impl Args {
    pub fn parse_args() -> Self {
        Args::parse()
    }
}
