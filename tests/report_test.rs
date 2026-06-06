use rust_homework::analyze_project;
use rust_homework::{render_report, AnalysisConfig, OutputFormat, RuleThresholds};
use std::fs;
use tempfile::tempdir;

#[test]
fn render_text_report() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("main.rs"), "fn main() {}\n").unwrap();

    let report = analyze_project(&AnalysisConfig {
        root_path: dir.path().to_path_buf(),
        format: OutputFormat::Text,
        output_path: None,
        thresholds: RuleThresholds::default(),
    })
    .unwrap();

    let text = render_report(&report, OutputFormat::Text).unwrap();
    assert!(text.contains("Rust Homework Static Analysis Report"));
    assert!(text.contains("File Count"));
}

#[test]
fn render_json_report() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("lib.rs"), "pub fn demo() {}\n").unwrap();

    let report = analyze_project(&AnalysisConfig {
        root_path: dir.path().to_path_buf(),
        format: OutputFormat::Json,
        output_path: None,
        thresholds: RuleThresholds::default(),
    })
    .unwrap();

    let text = render_report(&report, OutputFormat::Json).unwrap();
    assert!(text.contains("\"summary\""));
    assert!(text.contains("\"files\""));
}

#[test]
fn render_html_report() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("lib.rs"), "pub fn demo() {}\n").unwrap();

    let report = analyze_project(&AnalysisConfig {
        root_path: dir.path().to_path_buf(),
        format: OutputFormat::Html,
        output_path: None,
        thresholds: RuleThresholds::default(),
    })
    .unwrap();

    let text = render_report(&report, OutputFormat::Html).unwrap();
    assert!(text.contains("<!DOCTYPE html>"));
    assert!(text.contains("rust_homework report"));
}

#[test]
fn report_contains_issue_information() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("main.rs"),
        r#"
fn main() {
    let x = Some(1).unwrap();
    println!("{}", x);
}
"#,
    )
    .unwrap();

    let report = analyze_project(&AnalysisConfig {
        root_path: dir.path().to_path_buf(),
        format: OutputFormat::Text,
        output_path: None,
        thresholds: RuleThresholds::default(),
    })
    .unwrap();

    let text = render_report(&report, OutputFormat::Text).unwrap();
    assert!(text.contains("rust.unwrap"));
}