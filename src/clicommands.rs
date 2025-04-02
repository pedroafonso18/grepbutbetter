use clap::Parser;

#[derive(Parser)]
#[command(name = "grepbutbetter", about = "Search for patterns in files")]
pub struct Cli {
    // search line
    #[clap(value_name = "PATTERN")]
    pub find: String,
    
    /// search in files
    #[clap(value_name = "FILE", num_args = 1..)]
    pub file: Vec<String>,
    
    #[clap(short, long, help = "Perform a case-insensitive search")]
    pub ignore_case: bool,
}

impl Cli {
    pub fn parse() -> Self {
        Cli::parse_from(std::env::args())
    }
}