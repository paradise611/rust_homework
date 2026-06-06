use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn cli_runs_with_text_output() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("main.rs"), "fn main() {}\n").unwrap();

    let mut cmd = Command::cargo_bin("rust_homework").unwrap();
    cmd.arg("--path").arg(dir.path());
    cmd.assert().success();
}

#[test]
fn cli_runs_with_json_output() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("lib.rs"), "pub fn demo() {}\n").unwrap();

    let mut cmd = Command::cargo_bin("rust_homework").unwrap();
    cmd.arg("--path")
        .arg(dir.path())
        .arg("--format")
        .arg("json");

    let output = cmd.assert().success().get_output().stdout.clone();
    let text = String::from_utf8(output).unwrap();
    assert!(text.contains("\"files\""));
}

#[test]
fn cli_writes_html_file() {
    let dir = tempdir().unwrap();
    let output_dir = tempdir().unwrap();

    fs::write(dir.path().join("main.rs"), "fn main() {}\n").unwrap();
    let output_file = output_dir.path().join("report.html");

    let mut cmd = Command::cargo_bin("rust_homework").unwrap();
    cmd.arg("--path")
        .arg(dir.path())
        .arg("--format")
        .arg("html")
        .arg("--output")
        .arg(&output_file);

    cmd.assert().success();

    let content = fs::read_to_string(output_file).unwrap();
    assert!(content.contains("<html"));
    assert!(content.contains("rust_homework Static Analysis Report"));
}

#[test]
fn cli_accepts_custom_thresholds() {
    let dir = tempdir().unwrap();

    fs::write(
        dir.path().join("lib.rs"),
        r#"
pub fn f(a:i32,b:i32,c:i32,d:i32,e:i32) -> i32 { a+b+c+d+e }
"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("rust_homework").unwrap();
    cmd.arg("--path")
        .arg(dir.path())
        .arg("--max-parameters")
        .arg("2");

    cmd.assert().success();
}
