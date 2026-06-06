use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use walkdir::WalkDir;

use crate::model::{
    AnalysisConfig, AnalysisReport, AppError, AppResult, FileMetrics, FileReport, FunctionInfo,
    Issue, Position, Severity, Summary,
};

// ====================
// Project Entry
// ====================

pub fn analyze_project(config: &AnalysisConfig) -> AppResult<AnalysisReport> {
    let files = collect_source_files(&config.root_path)?;

    if files.is_empty() {
        return Err(AppError::InvalidInput(
            "no supported source files found in target directory".to_string(),
        ));
    }

    let mut reports = Vec::new();

    for path in files {
        let report = match detect_language(&path) {
            Some("rust") => analyze_rust_file(&path, config)?,
            Some("python") => analyze_python_file(&path, config)?,
            Some("cpp") => analyze_cpp_file(&path, config)?,
            _ => continue,
        };

        reports.push(report);
    }

    let summary = Summary::from_files(&reports);

    Ok(AnalysisReport {
        root_path: config.root_path.to_string_lossy().to_string(),
        generated_at: current_timestamp_string(),
        summary,
        files: reports,
    })
}

// ====================
// File Collection
// ====================

fn collect_source_files(root: &Path) -> AppResult<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(root) {
        let entry = entry?;
        let path = entry.path();

        if entry.file_type().is_dir() && should_skip_dir(path) {
            continue;
        }

        if entry.file_type().is_file() && is_supported_file(path) {
            files.push(path.to_path_buf());
        }
    }

    Ok(files)
}

fn should_skip_dir(path: &Path) -> bool {
    let dir_name = match path.file_name().and_then(|name| name.to_str()) {
        Some(name) => name,
        None => return false,
    };

    matches!(
        dir_name,
        "target" | ".git" | ".idea" | ".vscode" | "node_modules" | ".cargo"
    )
}

fn is_supported_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|s| s.to_str()),
        Some("rs" | "py" | "c" | "cc" | "cpp")
    )
}

fn detect_language(path: &Path) -> Option<&'static str> {
    match path.extension().and_then(|s| s.to_str()) {
        Some("rs") => Some("rust"),
        Some("py") => Some("python"),
        Some("c") | Some("cc") | Some("cpp") => Some("cpp"),
        _ => None,
    }
}

// ====================
// Shared Helpers
// ====================

fn read_file_lines(path: &Path) -> AppResult<Vec<String>> {
    let content = fs::read_to_string(path)?;
    Ok(content.lines().map(|line| line.to_string()).collect())
}

fn compute_basic_metrics(lines: &[String], metrics: &mut FileMetrics) {
    let mut total_length = 0usize;
    let mut longest = 0usize;

    for line in lines {
        let trimmed = line.trim();
        let line_len = line.chars().count();

        total_length += line_len;
        longest = longest.max(line_len);

        if trimmed.is_empty() {
            metrics.blank_lines += 1;
        } else if is_comment_line(trimmed) {
            metrics.comment_lines += 1;
        } else {
            metrics.code_lines += 1;
        }
    }

    metrics.total_lines = lines.len();
    metrics.longest_line_length = longest;
    metrics.average_line_length = if lines.is_empty() {
        0.0
    } else {
        total_length as f64 / lines.len() as f64
    };
}

fn is_comment_line(trimmed: &str) -> bool {
    trimmed.starts_with("//")
        || trimmed.starts_with("/*")
        || trimmed.starts_with('*')
        || trimmed.starts_with('#')
}

fn path_to_string(path: &Path) -> String {
    path.to_string_lossy().to_string()
}

fn current_timestamp_string() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs().to_string(),
        Err(_) => "0".to_string(),
    }
}

fn build_issue(
    severity: Severity,
    rule_name: &str,
    message: &str,
    path: &Path,
    line: usize,
    snippet: &str,
) -> Issue {
    Issue {
        rule_name: rule_name.to_string(),
        severity,
        file_path: path_to_string(path),
        position: Position { line, column: 1 },
        message: message.to_string(),
        snippet: snippet.to_string(),
    }
}

fn build_file_report(path: &Path, metrics: FileMetrics, issues: Vec<Issue>) -> FileReport {
    FileReport {
        path: path_to_string(path),
        metrics,
        issues,
    }
}

fn count_parameters(signature: &str) -> usize {
    let start = match signature.find('(') {
        Some(pos) => pos,
        None => return 0,
    };
    let end = match signature[start..].find(')') {
        Some(pos) => start + pos,
        None => return 0,
    };

    let params = &signature[start + 1..end].trim();
    if params.is_empty() {
        0
    } else {
        params.split(',').count()
    }
}

// ====================
// Rust Analysis
// ====================

fn analyze_rust_file(path: &Path, config: &AnalysisConfig) -> AppResult<FileReport> {
    let lines = read_file_lines(path)?;
    let mut metrics = FileMetrics::default();
    let mut issues = Vec::new();

    compute_basic_metrics(&lines, &mut metrics);
    analyze_rust_structure(&lines, &mut metrics);

    for (idx, line) in lines.iter().enumerate() {
        let line_no = idx + 1;
        let trimmed = line.trim();

        if trimmed.contains("fn ") {
            let function = build_rust_function_info(&lines, idx);
            if let Some(function) = function {
                if function.line_count > config.thresholds.max_function_length {
                    issues.push(build_issue(
                        Severity::Warning,
                        "rust.long_function",
                        "function is longer than configured threshold",
                        path,
                        function.start_line,
                        line,
                    ));
                }

                if function.parameter_count > config.thresholds.max_parameters {
                    issues.push(build_issue(
                        Severity::Warning,
                        "rust.too_many_parameters",
                        "function has too many parameters",
                        path,
                        function.start_line,
                        line,
                    ));
                }

                metrics.functions.push(function);
            }
        }

        if trimmed.contains(".unwrap()") || trimmed.contains(".expect(") {
            issues.push(build_issue(
                Severity::Warning,
                "rust.unwrap",
                "use of unwrap/expect may cause panic",
                path,
                line_no,
                line,
            ));
        }

        if trimmed.contains("todo!") || trimmed.contains("unimplemented!") {
            issues.push(build_issue(
                Severity::Warning,
                "rust.todo",
                "unfinished code marker found",
                path,
                line_no,
                line,
            ));
        }

        if trimmed.contains("unsafe") {
            issues.push(build_issue(
                Severity::Info,
                "rust.unsafe",
                "unsafe code detected",
                path,
                line_no,
                line,
            ));
        }

        if trimmed.contains("dbg!(") {
            issues.push(build_issue(
                Severity::Info,
                "rust.dbg",
                "debug macro dbg!() found",
                path,
                line_no,
                line,
            ));
        }
    }

    metrics.function_count = metrics.functions.len();
    check_shared_rules(path, &lines, &mut issues, &mut metrics, config);

    Ok(build_file_report(path, metrics, issues))
}

fn analyze_rust_structure(lines: &[String], metrics: &mut FileMetrics) {
    let mut current_depth = 0usize;
    let mut max_depth = 0usize;

    for line in lines {
        let trimmed = line.trim();

        if trimmed.starts_with("mod ") {
            metrics.module_count += 1;
        }
        if trimmed.starts_with("impl ") {
            metrics.impl_count += 1;
        }
        if trimmed.starts_with("struct ") || trimmed.contains(" struct ") {
            metrics.struct_count += 1;
        }
        if trimmed.starts_with("enum ") || trimmed.contains(" enum ") {
            metrics.enum_count += 1;
        }
        if trimmed.starts_with("trait ") || trimmed.contains(" trait ") {
            metrics.trait_count += 1;
        }
        if trimmed.contains("match ") {
            metrics.match_count += 1;
        }
        if trimmed.contains(" for ") || trimmed.starts_with("for ") || trimmed.contains("while ") {
            metrics.loop_count += 1;
        }
        if trimmed.contains("unsafe") {
            metrics.unsafe_count += 1;
        }

        for ch in line.chars() {
            if ch == '{' {
                current_depth += 1;
                max_depth = max_depth.max(current_depth);
            } else if ch == '}' && current_depth > 0 {
                current_depth -= 1;
            }
        }
    }

    metrics.max_nesting_depth = max_depth;
}

fn build_rust_function_info(lines: &[String], start_idx: usize) -> Option<FunctionInfo> {
    let line = lines.get(start_idx)?.trim();
    let fn_pos = line.find("fn ")?;
    let after_fn = &line[fn_pos + 3..];
    let name_end = after_fn.find('(')?;
    let name = after_fn[..name_end].trim().to_string();
    let parameter_count = count_parameters(line);

    let mut brace_balance = 0isize;
    let mut started_body = false;
    let mut end_line = start_idx + 1;

    for (idx, current_line) in lines.iter().enumerate().skip(start_idx) {
        for ch in current_line.chars() {
            if ch == '{' {
                brace_balance += 1;
                started_body = true;
            } else if ch == '}' {
                brace_balance -= 1;
            }
        }

        if started_body && brace_balance <= 0 {
            end_line = idx + 1;
            break;
        }
    }

    Some(FunctionInfo {
        name,
        start_line: start_idx + 1,
        end_line,
        parameter_count,
        line_count: end_line.saturating_sub(start_idx),
    })
}

// ====================
// Python Analysis
// ====================

fn analyze_python_file(path: &Path, config: &AnalysisConfig) -> AppResult<FileReport> {
    let lines = read_file_lines(path)?;
    let mut metrics = FileMetrics::default();
    let mut issues = Vec::new();

    compute_basic_metrics(&lines, &mut metrics);
    analyze_python_structure(&lines, &mut metrics);

    for (idx, line) in lines.iter().enumerate() {
        let line_no = idx + 1;
        let trimmed = line.trim();

        if trimmed.starts_with("def ") {
            let function = build_python_function_info(&lines, idx);
            if let Some(function) = function {
                if function.line_count > config.thresholds.max_function_length {
                    issues.push(build_issue(
                        Severity::Warning,
                        "python.long_function",
                        "function is longer than configured threshold",
                        path,
                        function.start_line,
                        line,
                    ));
                }

                if function.parameter_count > config.thresholds.max_parameters {
                    issues.push(build_issue(
                        Severity::Warning,
                        "python.too_many_parameters",
                        "function has too many parameters",
                        path,
                        function.start_line,
                        line,
                    ));
                }

                metrics.functions.push(function);
            }
        }

        if trimmed.contains("eval(") {
            issues.push(build_issue(
                Severity::Warning,
                "python.eval",
                "use of eval() is risky",
                path,
                line_no,
                line,
            ));
        }

        if trimmed.contains("exec(") {
            issues.push(build_issue(
                Severity::Warning,
                "python.exec",
                "use of exec() is risky",
                path,
                line_no,
                line,
            ));
        }

        if trimmed == "except:" || trimmed.starts_with("except:") {
            issues.push(build_issue(
                Severity::Warning,
                "python.bare_except",
                "bare except clause found",
                path,
                line_no,
                line,
            ));
        }

        if trimmed.contains("print(") {
            issues.push(build_issue(
                Severity::Info,
                "python.print",
                "possible debug print found",
                path,
                line_no,
                line,
            ));
        }

        if trimmed == "pass" {
            issues.push(build_issue(
                Severity::Info,
                "python.pass",
                "pass statement found, check if logic is incomplete",
                path,
                line_no,
                line,
            ));
        }
    }

    metrics.function_count = metrics.functions.len();
    check_shared_rules(path, &lines, &mut issues, &mut metrics, config);

    Ok(build_file_report(path, metrics, issues))
}

fn analyze_python_structure(lines: &[String], metrics: &mut FileMetrics) {
    let mut max_indent = 0usize;

    for line in lines {
        let trimmed = line.trim_start();

        if trimmed.starts_with("class ") {
            metrics.struct_count += 1;
        }
        if trimmed.starts_with("for ") || trimmed.starts_with("while ") {
            metrics.loop_count += 1;
        }
        if trimmed.starts_with("match ") {
            metrics.match_count += 1;
        }

        let indent = line.len() - trimmed.len();
        max_indent = max_indent.max(indent / 4);
    }

    metrics.max_nesting_depth = max_indent;
}

fn build_python_function_info(lines: &[String], start_idx: usize) -> Option<FunctionInfo> {
    let line = lines.get(start_idx)?.trim();
    if !line.starts_with("def ") {
        return None;
    }

    let after_def = &line[4..];
    let name_end = after_def.find('(')?;
    let name = after_def[..name_end].trim().to_string();
    let parameter_count = count_parameters(line);

    let base_indent = lines.get(start_idx)?.chars().take_while(|c| *c == ' ').count();
    let mut end_line = start_idx + 1;

    for (idx, current_line) in lines.iter().enumerate().skip(start_idx + 1) {
        let trimmed = current_line.trim();

        if trimmed.is_empty() {
            continue;
        }

        let indent = current_line.chars().take_while(|c| *c == ' ').count();
        if indent <= base_indent {
            break;
        }

        end_line = idx + 1;
    }

    Some(FunctionInfo {
        name,
        start_line: start_idx + 1,
        end_line,
        parameter_count,
        line_count: end_line.saturating_sub(start_idx),
    })
}

// ====================
// C/C++ Analysis
// ====================

fn analyze_cpp_file(path: &Path, config: &AnalysisConfig) -> AppResult<FileReport> {
    let lines = read_file_lines(path)?;
    let mut metrics = FileMetrics::default();
    let mut issues = Vec::new();

    compute_basic_metrics(&lines, &mut metrics);
    analyze_cpp_structure(&lines, &mut metrics);

    for (idx, line) in lines.iter().enumerate() {
        let line_no = idx + 1;
        let trimmed = line.trim();

        if looks_like_cpp_function(trimmed) {
            let function = build_cpp_function_info(&lines, idx);
            if let Some(function) = function {
                if function.line_count > config.thresholds.max_function_length {
                    issues.push(build_issue(
                        Severity::Warning,
                        "cpp.long_function",
                        "function is longer than configured threshold",
                        path,
                        function.start_line,
                        line,
                    ));
                }

                if function.parameter_count > config.thresholds.max_parameters {
                    issues.push(build_issue(
                        Severity::Warning,
                        "cpp.too_many_parameters",
                        "function has too many parameters",
                        path,
                        function.start_line,
                        line,
                    ));
                }

                metrics.functions.push(function);
            }
        }

        if trimmed.contains("gets(") {
            issues.push(build_issue(
                Severity::Error,
                "cpp.gets",
                "unsafe gets() usage found",
                path,
                line_no,
                line,
            ));
        }

        if trimmed.contains("strcpy(") || trimmed.contains("strcat(") {
            issues.push(build_issue(
                Severity::Warning,
                "cpp.unsafe_string",
                "unsafe C string function found",
                path,
                line_no,
                line,
            ));
        }

        if trimmed.contains("printf(") || trimmed.contains("cout <<") {
            issues.push(build_issue(
                Severity::Info,
                "cpp.output",
                "possible debug output found",
                path,
                line_no,
                line,
            ));
        }

        if trimmed.contains("malloc(")
            || trimmed.contains("free(")
            || trimmed.contains("new ")
            || trimmed.contains("delete ")
        {
            issues.push(build_issue(
                Severity::Info,
                "cpp.memory",
                "manual memory management detected",
                path,
                line_no,
                line,
            ));
        }

        if trimmed.contains("system(") {
            issues.push(build_issue(
                Severity::Warning,
                "cpp.system",
                "system() call found",
                path,
                line_no,
                line,
            ));
        }
    }

    metrics.function_count = metrics.functions.len();
    check_shared_rules(path, &lines, &mut issues, &mut metrics, config);

    Ok(build_file_report(path, metrics, issues))
}

fn analyze_cpp_structure(lines: &[String], metrics: &mut FileMetrics) {
    let mut current_depth = 0usize;
    let mut max_depth = 0usize;

    for line in lines {
        let trimmed = line.trim();

        if trimmed.starts_with("struct ") {
            metrics.struct_count += 1;
        }
        if trimmed.starts_with("enum ") {
            metrics.enum_count += 1;
        }
        if trimmed.starts_with("class ") {
            metrics.struct_count += 1;
        }
        if trimmed.starts_with("for ") || trimmed.starts_with("while ") {
            metrics.loop_count += 1;
        }
        if trimmed.contains("switch") {
            metrics.match_count += 1;
        }

        for ch in line.chars() {
            if ch == '{' {
                current_depth += 1;
                max_depth = max_depth.max(current_depth);
            } else if ch == '}' && current_depth > 0 {
                current_depth -= 1;
            }
        }
    }

    metrics.max_nesting_depth = max_depth;
}

fn looks_like_cpp_function(trimmed: &str) -> bool {
    trimmed.contains('(')
        && trimmed.contains(')')
        && trimmed.contains('{')
        && !trimmed.starts_with("if ")
        && !trimmed.starts_with("for ")
        && !trimmed.starts_with("while ")
        && !trimmed.starts_with("switch ")
        && !trimmed.starts_with("//")
}

fn build_cpp_function_info(lines: &[String], start_idx: usize) -> Option<FunctionInfo> {
    let line = lines.get(start_idx)?.trim();
    if !looks_like_cpp_function(line) {
        return None;
    }

    let before_paren = line.split('(').next()?.trim();
    let name = before_paren.split_whitespace().last()?.to_string();
    let parameter_count = count_parameters(line);

    let mut brace_balance = 0isize;
    let mut started_body = false;
    let mut end_line = start_idx + 1;

    for (idx, current_line) in lines.iter().enumerate().skip(start_idx) {
        for ch in current_line.chars() {
            if ch == '{' {
                brace_balance += 1;
                started_body = true;
            } else if ch == '}' {
                brace_balance -= 1;
            }
        }

        if started_body && brace_balance <= 0 {
            end_line = idx + 1;
            break;
        }
    }

    Some(FunctionInfo {
        name,
        start_line: start_idx + 1,
        end_line,
        parameter_count,
        line_count: end_line.saturating_sub(start_idx),
    })
}

// ====================
// Shared Rules
// ====================

fn check_shared_rules(
    path: &Path,
    lines: &[String],
    issues: &mut Vec<Issue>,
    metrics: &mut FileMetrics,
    config: &AnalysisConfig,
) {
    for (idx, line) in lines.iter().enumerate() {
        let line_no = idx + 1;

        if line.chars().count() > config.thresholds.max_line_length {
            issues.push(build_issue(
                Severity::Info,
                "style.long_line",
                "line longer than configured threshold",
                path,
                line_no,
                line,
            ));
        }
    }

    if lines.len() > config.thresholds.max_file_lines {
        issues.push(build_issue(
            Severity::Warning,
            "size.large_file",
            "file is larger than configured threshold",
            path,
            1,
            lines.first().map(|s| s.as_str()).unwrap_or(""),
        ));
    }

    if metrics.max_nesting_depth > config.thresholds.max_nesting_depth {
        issues.push(build_issue(
            Severity::Warning,
            "style.deep_nesting",
            "nesting depth is larger than configured threshold",
            path,
            1,
            "",
        ));
    }
}