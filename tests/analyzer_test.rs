use rust_homework::analyze_project;
use rust_homework::{AnalysisConfig, OutputFormat, RuleThresholds};
use std::fs;
use tempfile::tempdir;

#[test]
fn analyze_simple_project() {
    let dir = tempdir().unwrap();

    fs::write(
        dir.path().join("main.rs"),
        r#"
fn main() {
    let value = Some(1).unwrap();
    println!("{}", value);
}
"#,
    )
    .unwrap();

    let config = AnalysisConfig {
        root_path: dir.path().to_path_buf(),
        format: OutputFormat::Text,
        output_path: None,
        thresholds: RuleThresholds::default(),
    };

    let report = analyze_project(&config).unwrap();
    assert_eq!(report.summary.file_count, 1);
    assert!(report.summary.total_issues >= 1);
}

#[test]
fn detect_todo_and_expect() {
    let dir = tempdir().unwrap();

    fs::write(
        dir.path().join("lib.rs"),
        r#"
pub fn demo() {
    let _x = Some(1).expect("bad");
    todo!("finish");
}
"#,
    )
    .unwrap();

    let config = AnalysisConfig {
        root_path: dir.path().to_path_buf(),
        format: OutputFormat::Text,
        output_path: None,
        thresholds: RuleThresholds::default(),
    };

    let report = analyze_project(&config).unwrap();
    let issues = &report.files[0].issues;

    assert!(issues.iter().any(|i| i.rule_name == "rust.unwrap"));
    assert!(issues.iter().any(|i| i.rule_name == "rust.todo"));
}

#[test]
fn detect_long_function() {
    let dir = tempdir().unwrap();

    fs::write(
        dir.path().join("lib.rs"),
        r#"
pub fn long_function() {
    println!("1");
    println!("2");
    println!("3");
    println!("4");
    println!("5");
    println!("6");
    println!("7");
    println!("8");
    println!("9");
}
"#,
    )
    .unwrap();

    let config = AnalysisConfig {
        root_path: dir.path().to_path_buf(),
        format: OutputFormat::Text,
        output_path: None,
        thresholds: RuleThresholds {
            max_function_length: 5,
            ..RuleThresholds::default()
        },
    };

    let report = analyze_project(&config).unwrap();
    assert!(report.files[0]
        .issues
        .iter()
        .any(|i| i.rule_name == "rust.long_function"));
}

#[test]
fn detect_too_many_parameters() {
    let dir = tempdir().unwrap();

    fs::write(
        dir.path().join("lib.rs"),
        r#"
pub fn many_params(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> i32 {
    a + b + c + d + e + f + g
}
"#,
    )
    .unwrap();

    let config = AnalysisConfig {
        root_path: dir.path().to_path_buf(),
        format: OutputFormat::Text,
        output_path: None,
        thresholds: RuleThresholds {
            max_parameters: 4,
            ..RuleThresholds::default()
        },
    };

    let report = analyze_project(&config).unwrap();
    assert!(report.files[0]
        .issues
        .iter()
        .any(|i| i.rule_name == "rust.too_many_parameters"));
}

#[test]
fn detect_deep_nesting() {
    let dir = tempdir().unwrap();

    fs::write(
        dir.path().join("main.rs"),
        r#"
fn main() {
    if true {
        while true {
            for _ in 0..1 {
                if true {
                    loop {
                        break;
                    }
                }
            }
        }
    }
}
"#,
    )
    .unwrap();

    let config = AnalysisConfig {
        root_path: dir.path().to_path_buf(),
        format: OutputFormat::Text,
        output_path: None,
        thresholds: RuleThresholds {
            max_nesting_depth: 3,
            ..RuleThresholds::default()
        },
    };

    let report = analyze_project(&config).unwrap();
    assert!(report.files[0]
        .issues
        .iter()
        .any(|i| i.rule_name == "style.deep_nesting"));
}

#[test]
fn ignore_non_rust_files() {
    let dir = tempdir().unwrap();

    fs::write(dir.path().join("a.txt"), "hello").unwrap();
    fs::write(dir.path().join("b.md"), "# title").unwrap();
    fs::write(dir.path().join("main.rs"), "fn main() {}\n").unwrap();

    let config = AnalysisConfig {
        root_path: dir.path().to_path_buf(),
        format: OutputFormat::Text,
        output_path: None,
        thresholds: RuleThresholds::default(),
    };

    let report = analyze_project(&config).unwrap();
    assert_eq!(report.summary.file_count, 1);
}

#[test]
fn count_basic_metrics() {
    let dir = tempdir().unwrap();

    fs::write(
        dir.path().join("lib.rs"),
        r#"
// comment
pub struct User {
    pub id: i32,
}

pub enum State {
    Ready,
}

pub trait Worker {
    fn work(&self);
}

impl User {
    pub fn work(&self) {}
}
"#,
    )
    .unwrap();

    let config = AnalysisConfig {
        root_path: dir.path().to_path_buf(),
        format: OutputFormat::Text,
        output_path: None,
        thresholds: RuleThresholds::default(),
    };

    let report = analyze_project(&config).unwrap();
    let metrics = &report.files[0].metrics;

    assert_eq!(metrics.struct_count, 1);
    assert_eq!(metrics.enum_count, 1);
    assert_eq!(metrics.trait_count, 1);
    assert_eq!(metrics.impl_count, 1);
    assert!(metrics.function_count >= 1);
}

#[test]
fn detect_dbg_and_unwrap() {
    let dir = tempdir().unwrap();

    fs::write(
        dir.path().join("main.rs"),
        r#"
fn main() {
    dbg!(123);
    let x = Some(1).unwrap();
    println!("{}", x);
}
"#,
    )
    .unwrap();

    let config = AnalysisConfig {
        root_path: dir.path().to_path_buf(),
        format: OutputFormat::Text,
        output_path: None,
        thresholds: RuleThresholds::default(),
    };

    let report = analyze_project(&config).unwrap();
    let issues = &report.files[0].issues;

    assert!(issues.iter().any(|i| i.rule_name == "rust.dbg"));
    assert!(issues.iter().any(|i| i.rule_name == "rust.unwrap"));
}
