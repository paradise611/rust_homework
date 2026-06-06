use crate::analyzer::analyze_project;
use crate::model::{AnalysisConfig, AppResult, OutputFormat, RuleThresholds};
use crate::report::render_report;
use clap::{Parser, ValueEnum};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliFormat {
    Text,
    Json,
    Html,
}

impl From<CliFormat> for OutputFormat {
    fn from(value: CliFormat) -> Self {
        match value {
            CliFormat::Text => OutputFormat::Text,
            CliFormat::Json => OutputFormat::Json,
            CliFormat::Html => OutputFormat::Html,
        }
    }
}

#[derive(Debug, Parser)]
#[command(
    name = "rust_homework",
    version,
    about = "A simple Rust static analysis tool for course homework"
)]
struct Args {
    #[arg(short, long, default_value = "./upload")]
    path: PathBuf,

    #[arg(short, long, value_enum, default_value = "text")]
    format: CliFormat,

    #[arg(short, long)]
    output: Option<PathBuf>,

    #[arg(long, default_value_t = 120)]
    max_line_length: usize,

    #[arg(long, default_value_t = 80)]
    max_function_length: usize,

    #[arg(long, default_value_t = 6)]
    max_parameters: usize,

    #[arg(long, default_value_t = 5)]
    max_nesting_depth: usize,

    #[arg(long, default_value_t = 800)]
    max_file_lines: usize,
}

pub fn run() -> AppResult<()> {
    let args = Args::parse();

    let config = AnalysisConfig {
        root_path: args.path,
        format: args.format.into(),
        output_path: args.output,
        thresholds: RuleThresholds {
            max_line_length: args.max_line_length,
            max_function_length: args.max_function_length,
            max_parameters: args.max_parameters,
            max_nesting_depth: args.max_nesting_depth,
            max_file_lines: args.max_file_lines,
        },
    };

    execute(config)
}

pub fn execute(config: AnalysisConfig) -> AppResult<()> {
    let report = analyze_project(&config)?;
    let rendered = render_report(&report, config.format)?;

    if let Some(path) = &config.output_path {
        fs::write(path, rendered)?;
        println!("Report saved to: {}", path.display());
    } else {
        println!("{rendered}");
    }

    Ok(())
}
