use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize)]
#[clap(name = "AtacamaLifeAnalyzer", version = "1.0")]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Serialize, Deserialize)]
pub enum Commands {
    /// Analyze soil biodiversity data
    Analyze(AnalyzeArgs),
    /// Generate reports from analysis results
    Report(ReportArgs),
}

#[derive(clap::Args, Debug, Serialize, Deserialize)]
pub struct AnalyzeArgs {
    /// Input CSV file with soil sample data
    #[clap(short, long)]
    pub input: String,
    
    /// Output JSON file for analysis results
    #[clap(short, long)]
    pub output: String,
    
    /// Enable verbose logging
    #[clap(long)]
    pub verbose: bool,
}

#[derive(clap::Args, Debug, Serialize, Deserialize)]
pub struct ReportArgs {
    /// Input JSON file with analysis results
    #[clap(short, long)]
    pub input: String,
    
    /// Output file for generated report
    #[clap(short, long)]
    pub output: String,
    
    /// Report template to use
    #[clap(short, long, default_value = "default")]
    pub template: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}