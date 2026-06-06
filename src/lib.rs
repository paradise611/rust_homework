pub mod analyzer;
pub mod cli;
pub mod model;
pub mod report;

pub use analyzer::analyze_project;
pub use model::{
    AnalysisConfig, AnalysisReport, AppError, FileMetrics, FileReport, FunctionInfo, Issue,
    OutputFormat, Position, RuleThresholds, Severity, Summary,
};
pub use report::render_report;
