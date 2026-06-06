use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Walkdir(walkdir::Error),
    Json(serde_json::Error),
    InvalidInput(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "io error: {err}"),
            AppError::Walkdir(err) => write!(f, "walkdir error: {err}"),
            AppError::Json(err) => write!(f, "json error: {err}"),
            AppError::InvalidInput(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::Io(value)
    }
}

impl From<walkdir::Error> for AppError {
    fn from(value: walkdir::Error) -> Self {
        AppError::Walkdir(value)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        AppError::Json(value)
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
    Html,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleThresholds {
    pub max_line_length: usize,
    pub max_function_length: usize,
    pub max_parameters: usize,
    pub max_nesting_depth: usize,
    pub max_file_lines: usize,
}

impl Default for RuleThresholds {
    fn default() -> Self {
        Self {
            max_line_length: 120,
            max_function_length: 80,
            max_parameters: 6,
            max_nesting_depth: 5,
            max_file_lines: 800,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    pub root_path: PathBuf,
    pub format: OutputFormat,
    pub output_path: Option<PathBuf>,
    pub thresholds: RuleThresholds,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            root_path: PathBuf::from("."),
            format: OutputFormat::Text,
            output_path: None,
            thresholds: RuleThresholds::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

impl Display for Severity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Info => write!(f, "Info"),
            Severity::Warning => write!(f, "Warning"),
            Severity::Error => write!(f, "Error"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub rule_name: String,
    pub severity: Severity,
    pub file_path: String,
    pub position: Position,
    pub message: String,
    pub snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub start_line: usize,
    pub end_line: usize,
    pub parameter_count: usize,
    pub line_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileMetrics {
    pub total_lines: usize,
    pub code_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
    pub function_count: usize,
    pub module_count: usize,
    pub impl_count: usize,
    pub struct_count: usize,
    pub enum_count: usize,
    pub trait_count: usize,
    pub match_count: usize,
    pub loop_count: usize,
    pub unsafe_count: usize,
    pub max_nesting_depth: usize,
    pub average_line_length: f64,
    pub longest_line_length: usize,
    pub functions: Vec<FunctionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileReport {
    pub path: String,
    pub metrics: FileMetrics,
    pub issues: Vec<Issue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Summary {
    pub file_count: usize,
    pub total_issues: usize,
    pub total_lines: usize,
    pub total_code_lines: usize,
    pub total_comment_lines: usize,
    pub total_blank_lines: usize,
    pub total_functions: usize,
    pub total_modules: usize,
    pub total_structs: usize,
    pub total_enums: usize,
    pub total_traits: usize,
    pub total_impls: usize,
    pub total_matches: usize,
    pub total_loops: usize,
    pub total_unsafe_blocks: usize,
    pub severity_count: BTreeMap<String, usize>,
}

impl Summary {
    pub fn from_files(files: &[FileReport]) -> Self {
        let mut summary = Summary {
            file_count: files.len(),
            ..Summary::default()
        };

        for file in files {
            summary.total_lines += file.metrics.total_lines;
            summary.total_code_lines += file.metrics.code_lines;
            summary.total_comment_lines += file.metrics.comment_lines;
            summary.total_blank_lines += file.metrics.blank_lines;
            summary.total_functions += file.metrics.function_count;
            summary.total_modules += file.metrics.module_count;
            summary.total_structs += file.metrics.struct_count;
            summary.total_enums += file.metrics.enum_count;
            summary.total_traits += file.metrics.trait_count;
            summary.total_impls += file.metrics.impl_count;
            summary.total_matches += file.metrics.match_count;
            summary.total_loops += file.metrics.loop_count;
            summary.total_unsafe_blocks += file.metrics.unsafe_count;
            summary.total_issues += file.issues.len();

            for issue in &file.issues {
                let key = issue.severity.to_string();
                *summary.severity_count.entry(key).or_insert(0) += 1;
            }
        }

        summary
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub root_path: String,
    pub generated_at: String,
    pub summary: Summary,
    pub files: Vec<FileReport>,
}
