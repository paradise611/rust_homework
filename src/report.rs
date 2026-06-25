use crate::model::{AnalysisReport, AppResult, OutputFormat};

pub fn render_report(report: &AnalysisReport, format: OutputFormat) -> AppResult<String> {
    match format {
        OutputFormat::Text => Ok(render_text_report(report)),
        OutputFormat::Json => Ok(render_json_report(report)?),
        OutputFormat::Html => Ok(render_html_report(report)),
    }
}

fn render_text_report(report: &AnalysisReport) -> String {
    let mut out = String::new();
    let visible_total_issues = visible_issue_count(report);

    out.push_str("Rust Homework Static Analysis Report\n");
    out.push_str("File Count\n");
    out.push_str("===================================\n\n");

    out.push_str("Rust 作业静态分析报告\n");
    out.push_str("====================\n\n");
    out.push_str(&format!("项目路径: {}\n", report.root_path));
    out.push_str(&format!("生成时间: {}\n", report.generated_at));
    out.push_str(&format!("文件数量: {}\n", report.summary.file_count));
    out.push_str(&format!("问题总数: {}\n", visible_total_issues));
    out.push_str(&format!("总行数: {}\n", report.summary.total_lines));
    out.push_str(&format!("代码行数: {}\n", report.summary.total_code_lines));
    out.push_str(&format!(
        "注释行数: {}\n",
        report.summary.total_comment_lines
    ));
    out.push_str(&format!(
        "空白行数: {}\n\n",
        report.summary.total_blank_lines
    ));

    for file in &report.files {
        let issues = visible_issues(&file.issues);

        out.push_str(&format!("文件: {}\n", file.path));
        out.push_str(&format!("  问题数: {}\n", issues.len()));
        out.push_str(&format!("  总行数: {}\n", file.metrics.total_lines));
        out.push_str(&format!("  代码行: {}\n", file.metrics.code_lines));
        out.push_str(&format!("  注释行: {}\n", file.metrics.comment_lines));
        out.push_str(&format!("  空白行: {}\n", file.metrics.blank_lines));
        out.push_str(&format!("  函数数量: {}\n", file.metrics.function_count));
        out.push_str(&format!(
            "  最大嵌套深度: {}\n",
            file.metrics.max_nesting_depth
        ));

        if !file.metrics.functions.is_empty() {
            out.push_str("  函数列表:\n");
            for func in &file.metrics.functions {
                out.push_str(&format!(
                    "    - {} (第 {}-{} 行, 参数 {}, 长度 {} 行)\n",
                    func.name,
                    func.start_line,
                    func.end_line,
                    func.parameter_count,
                    func.line_count
                ));
            }
        }

        if issues.is_empty() {
            out.push_str("  未发现警告或严重问题。\n");
        } else {
            out.push_str("  问题详情:\n");
            for issue in issues {
                out.push_str(&format!(
                    "    - [{}] 规则: {} | 第 {} 行 | {}\n",
                    severity_label(&issue.severity.to_string()),
                    issue.rule_name,
                    issue.position.line,
                    issue.message
                ));

                if !issue.snippet.trim().is_empty() {
                    out.push_str(&format!("      代码: {}\n", issue.snippet.trim()));
                }
            }
        }

        out.push('\n');
    }

    out
}

fn render_json_report(report: &AnalysisReport) -> AppResult<String> {
    Ok(serde_json::to_string_pretty(report)?)
}

fn render_html_report(report: &AnalysisReport) -> String {
    let mut out = String::new();
    let visible_total_issues = visible_issue_count(report);

    out.push_str("<!DOCTYPE html>\n");
    out.push_str("<html lang=\"zh-CN\">\n");
    out.push_str("<head>\n");
    out.push_str("  <meta charset=\"UTF-8\">\n");
    out.push_str("  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    out.push_str("  <title>rust_homework report</title>\n");
    out.push_str("  <style>\n");
    out.push_str("    :root {\n");
    out.push_str("      --bg: #f5f1e8;\n");
    out.push_str("      --paper: rgba(255, 252, 246, 0.92);\n");
    out.push_str("      --panel: #fffdf8;\n");
    out.push_str("      --line: #ded6c8;\n");
    out.push_str("      --text: #2c2722;\n");
    out.push_str("      --muted: #74695e;\n");
    out.push_str("      --accent: #8c5a3c;\n");
    out.push_str("      --accent-soft: #efe3d3;\n");
    out.push_str("      --ok: #5b7f5f;\n");
    out.push_str("      --warn: #b7791f;\n");
    out.push_str("      --err: #b34a36;\n");
    out.push_str("      --shadow: 0 18px 48px rgba(89, 67, 44, 0.08);\n");
    out.push_str("    }\n");
    out.push_str("    * { box-sizing: border-box; }\n");
    out.push_str("    body {\n");
    out.push_str("      margin: 0;\n");
    out.push_str("      font-family: \"Noto Serif SC\", \"Source Han Serif SC\", \"PingFang SC\", \"Microsoft YaHei\", serif;\n");
    out.push_str("      color: var(--text);\n");
    out.push_str("      background:\n");
    out.push_str(
        "        radial-gradient(circle at top left, rgba(140, 90, 60, 0.10), transparent 32%),\n",
    );
    out.push_str("        linear-gradient(180deg, #f7f3eb 0%, #f1ebe1 100%);\n");
    out.push_str("      line-height: 1.7;\n");
    out.push_str("    }\n");
    out.push_str("    .page {\n");
    out.push_str("      max-width: 1100px;\n");
    out.push_str("      margin: 0 auto;\n");
    out.push_str("      padding: 48px 20px 64px;\n");
    out.push_str("    }\n");
    out.push_str("    .hero {\n");
    out.push_str("      background: var(--paper);\n");
    out.push_str("      border: 1px solid rgba(140, 90, 60, 0.14);\n");
    out.push_str("      border-radius: 24px;\n");
    out.push_str("      padding: 32px;\n");
    out.push_str("      box-shadow: var(--shadow);\n");
    out.push_str("      backdrop-filter: blur(10px);\n");
    out.push_str("      margin-bottom: 24px;\n");
    out.push_str("    }\n");
    out.push_str("    .eyebrow {\n");
    out.push_str("      font-size: 12px;\n");
    out.push_str("      letter-spacing: 0.14em;\n");
    out.push_str("      text-transform: uppercase;\n");
    out.push_str("      color: var(--muted);\n");
    out.push_str("      margin-bottom: 10px;\n");
    out.push_str("    }\n");
    out.push_str("    h1 {\n");
    out.push_str("      margin: 0;\n");
    out.push_str("      font-size: clamp(28px, 4vw, 42px);\n");
    out.push_str("      line-height: 1.2;\n");
    out.push_str("      font-weight: 700;\n");
    out.push_str("    }\n");
    out.push_str("    .subtitle {\n");
    out.push_str("      margin-top: 12px;\n");
    out.push_str("      color: var(--muted);\n");
    out.push_str("      font-size: 15px;\n");
    out.push_str("    }\n");
    out.push_str("    .summary-grid {\n");
    out.push_str("      display: grid;\n");
    out.push_str("      grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));\n");
    out.push_str("      gap: 14px;\n");
    out.push_str("      margin-top: 28px;\n");
    out.push_str("    }\n");
    out.push_str("    .stat {\n");
    out.push_str("      background: var(--panel);\n");
    out.push_str("      border: 1px solid var(--line);\n");
    out.push_str("      border-radius: 18px;\n");
    out.push_str("      padding: 18px;\n");
    out.push_str("    }\n");
    out.push_str("    .stat-label {\n");
    out.push_str("      font-size: 13px;\n");
    out.push_str("      color: var(--muted);\n");
    out.push_str("      margin-bottom: 8px;\n");
    out.push_str("    }\n");
    out.push_str("    .stat-value {\n");
    out.push_str("      font-size: 28px;\n");
    out.push_str("      font-weight: 700;\n");
    out.push_str("      color: var(--accent);\n");
    out.push_str("    }\n");
    out.push_str("    .meta {\n");
    out.push_str("      margin-top: 20px;\n");
    out.push_str("      padding-top: 18px;\n");
    out.push_str("      border-top: 1px solid var(--line);\n");
    out.push_str("      color: var(--muted);\n");
    out.push_str("      font-size: 14px;\n");
    out.push_str("    }\n");
    out.push_str("    .files {\n");
    out.push_str("      display: grid;\n");
    out.push_str("      gap: 18px;\n");
    out.push_str("    }\n");
    out.push_str("    .file-card {\n");
    out.push_str("      background: var(--paper);\n");
    out.push_str("      border: 1px solid rgba(140, 90, 60, 0.12);\n");
    out.push_str("      border-radius: 22px;\n");
    out.push_str("      padding: 24px;\n");
    out.push_str("      box-shadow: var(--shadow);\n");
    out.push_str("    }\n");
    out.push_str("    .file-header {\n");
    out.push_str("      display: flex;\n");
    out.push_str("      justify-content: space-between;\n");
    out.push_str("      gap: 16px;\n");
    out.push_str("      align-items: flex-start;\n");
    out.push_str("      flex-wrap: wrap;\n");
    out.push_str("      margin-bottom: 16px;\n");
    out.push_str("    }\n");
    out.push_str("    .file-title {\n");
    out.push_str("      margin: 0;\n");
    out.push_str("      font-size: 20px;\n");
    out.push_str("      word-break: break-all;\n");
    out.push_str("    }\n");
    out.push_str("    .badge {\n");
    out.push_str("      display: inline-flex;\n");
    out.push_str("      align-items: center;\n");
    out.push_str("      gap: 8px;\n");
    out.push_str("      padding: 8px 12px;\n");
    out.push_str("      border-radius: 999px;\n");
    out.push_str("      background: var(--accent-soft);\n");
    out.push_str("      color: var(--accent);\n");
    out.push_str("      font-size: 13px;\n");
    out.push_str("      white-space: nowrap;\n");
    out.push_str("    }\n");
    out.push_str("    .metric-row {\n");
    out.push_str("      display: grid;\n");
    out.push_str("      grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));\n");
    out.push_str("      gap: 10px;\n");
    out.push_str("      margin-bottom: 18px;\n");
    out.push_str("    }\n");
    out.push_str("    .metric-box {\n");
    out.push_str("      background: var(--panel);\n");
    out.push_str("      border: 1px solid var(--line);\n");
    out.push_str("      border-radius: 14px;\n");
    out.push_str("      padding: 12px 14px;\n");
    out.push_str("    }\n");
    out.push_str("    .metric-box strong {\n");
    out.push_str("      display: block;\n");
    out.push_str("      font-size: 12px;\n");
    out.push_str("      color: var(--muted);\n");
    out.push_str("      margin-bottom: 6px;\n");
    out.push_str("      font-weight: 500;\n");
    out.push_str("    }\n");
    out.push_str("    .metric-box span {\n");
    out.push_str("      font-size: 18px;\n");
    out.push_str("      font-weight: 700;\n");
    out.push_str("    }\n");
    out.push_str("    h2.section-title {\n");
    out.push_str("      margin: 0 0 14px;\n");
    out.push_str("      font-size: 16px;\n");
    out.push_str("    }\n");
    out.push_str("    .functions, .issues {\n");
    out.push_str("      margin-top: 16px;\n");
    out.push_str("    }\n");
    out.push_str("    ul.clean {\n");
    out.push_str("      list-style: none;\n");
    out.push_str("      padding: 0;\n");
    out.push_str("      margin: 0;\n");
    out.push_str("      display: grid;\n");
    out.push_str("      gap: 10px;\n");
    out.push_str("    }\n");
    out.push_str("    .list-card {\n");
    out.push_str("      background: var(--panel);\n");
    out.push_str("      border: 1px solid var(--line);\n");
    out.push_str("      border-radius: 14px;\n");
    out.push_str("      padding: 14px 16px;\n");
    out.push_str("    }\n");
    out.push_str("    .issue-top {\n");
    out.push_str("      display: flex;\n");
    out.push_str("      justify-content: space-between;\n");
    out.push_str("      gap: 12px;\n");
    out.push_str("      flex-wrap: wrap;\n");
    out.push_str("      margin-bottom: 8px;\n");
    out.push_str("    }\n");
    out.push_str("    .issue-rule {\n");
    out.push_str("      font-family: \"JetBrains Mono\", \"Consolas\", monospace;\n");
    out.push_str("      font-size: 13px;\n");
    out.push_str("      color: var(--accent);\n");
    out.push_str("    }\n");
    out.push_str("    .severity {\n");
    out.push_str("      padding: 4px 10px;\n");
    out.push_str("      border-radius: 999px;\n");
    out.push_str("      font-size: 12px;\n");
    out.push_str("      font-weight: 700;\n");
    out.push_str("    }\n");
    out.push_str("    .severity-warning { background: #fbefda; color: var(--warn); }\n");
    out.push_str("    .severity-error { background: #f8e1dc; color: var(--err); }\n");
    out.push_str("    .issue-message {\n");
    out.push_str("      margin: 0;\n");
    out.push_str("      font-size: 14px;\n");
    out.push_str("    }\n");
    out.push_str("    .issue-snippet {\n");
    out.push_str("      margin-top: 10px;\n");
    out.push_str("      padding: 12px 14px;\n");
    out.push_str("      background: #f8f5ef;\n");
    out.push_str("      border-radius: 12px;\n");
    out.push_str("      border: 1px dashed var(--line);\n");
    out.push_str("      font-family: \"JetBrains Mono\", \"Consolas\", monospace;\n");
    out.push_str("      font-size: 13px;\n");
    out.push_str("      overflow-x: auto;\n");
    out.push_str("      white-space: pre-wrap;\n");
    out.push_str("      word-break: break-word;\n");
    out.push_str("    }\n");
    out.push_str("    .empty {\n");
    out.push_str("      color: var(--muted);\n");
    out.push_str("      background: var(--panel);\n");
    out.push_str("      border: 1px solid var(--line);\n");
    out.push_str("      border-radius: 14px;\n");
    out.push_str("      padding: 14px 16px;\n");
    out.push_str("    }\n");
    out.push_str("    @media (max-width: 640px) {\n");
    out.push_str("      .page { padding: 24px 14px 40px; }\n");
    out.push_str("      .hero, .file-card { padding: 20px; border-radius: 18px; }\n");
    out.push_str("      .stat-value { font-size: 24px; }\n");
    out.push_str("    }\n");
    out.push_str("  </style>\n");
    out.push_str("</head>\n");
    out.push_str("<body>\n");
    out.push_str("  <div class=\"page\">\n");
    out.push_str("    <section class=\"hero\">\n");
    out.push_str("      <div class=\"eyebrow\">rust_homework Static Analysis Report</div>\n");
    out.push_str("      <h1>Rust 作业静态分析报告</h1>\n");
    out.push_str("      <p class=\"subtitle\">本报告仅展示警告与严重问题，已隐藏提示级别信息，方便优先处理真正需要修复的项。</p>\n");
    out.push_str("      <div class=\"summary-grid\">\n");
    out.push_str(&format!(
        "        <div class=\"stat\"><div class=\"stat-label\">文件数量</div><div class=\"stat-value\">{}</div></div>\n",
        report.summary.file_count
    ));
    out.push_str(&format!(
        "        <div class=\"stat\"><div class=\"stat-label\">问题总数</div><div class=\"stat-value\">{}</div></div>\n",
        visible_total_issues
    ));
    out.push_str(&format!(
        "        <div class=\"stat\"><div class=\"stat-label\">总行数</div><div class=\"stat-value\">{}</div></div>\n",
        report.summary.total_lines
    ));
    out.push_str(&format!(
        "        <div class=\"stat\"><div class=\"stat-label\">代码行数</div><div class=\"stat-value\">{}</div></div>\n",
        report.summary.total_code_lines
    ));
    out.push_str(&format!(
        "        <div class=\"stat\"><div class=\"stat-label\">注释行数</div><div class=\"stat-value\">{}</div></div>\n",
        report.summary.total_comment_lines
    ));
    out.push_str(&format!(
        "        <div class=\"stat\"><div class=\"stat-label\">空白行数</div><div class=\"stat-value\">{}</div></div>\n",
        report.summary.total_blank_lines
    ));
    out.push_str("      </div>\n");
    out.push_str("      <div class=\"meta\">\n");
    out.push_str(&format!(
        "        <div><strong>项目路径：</strong>{}</div>\n",
        escape_html(&report.root_path)
    ));
    out.push_str(&format!(
        "        <div><strong>生成时间：</strong>{}</div>\n",
        escape_html(&report.generated_at)
    ));
    out.push_str("      </div>\n");
    out.push_str("    </section>\n");

    out.push_str("    <section class=\"files\">\n");
    for file in &report.files {
        let issues = visible_issues(&file.issues);

        out.push_str("      <article class=\"file-card\">\n");
        out.push_str("        <div class=\"file-header\">\n");
        out.push_str(&format!(
            "          <h2 class=\"file-title\">{}</h2>\n",
            escape_html(&file.path)
        ));
        out.push_str(&format!(
            "          <div class=\"badge\">{} 个问题</div>\n",
            issues.len()
        ));
        out.push_str("        </div>\n");

        out.push_str("        <div class=\"metric-row\">\n");
        out.push_str(&metric_box("总行数", file.metrics.total_lines));
        out.push_str(&metric_box("代码行", file.metrics.code_lines));
        out.push_str(&metric_box("注释行", file.metrics.comment_lines));
        out.push_str(&metric_box("空白行", file.metrics.blank_lines));
        out.push_str(&metric_box("函数数", file.metrics.function_count));
        out.push_str(&metric_box("最大嵌套", file.metrics.max_nesting_depth));
        out.push_str("        </div>\n");

        out.push_str("        <div class=\"functions\">\n");
        out.push_str("          <h2 class=\"section-title\">函数信息</h2>\n");
        if file.metrics.functions.is_empty() {
            out.push_str("          <div class=\"empty\">未识别到函数。</div>\n");
        } else {
            out.push_str("          <ul class=\"clean\">\n");
            for func in &file.metrics.functions {
                out.push_str(&format!(
                    "            <li class=\"list-card\"><strong>{}</strong><br>第 {}-{} 行，参数 {} 个，长度 {} 行</li>\n",
                    escape_html(&func.name),
                    func.start_line,
                    func.end_line,
                    func.parameter_count,
                    func.line_count
                ));
            }
            out.push_str("          </ul>\n");
        }
        out.push_str("        </div>\n");

        out.push_str("        <div class=\"issues\">\n");
        out.push_str("          <h2 class=\"section-title\">问题详情</h2>\n");
        if issues.is_empty() {
            out.push_str("          <div class=\"empty\">未发现警告或严重问题。</div>\n");
        } else {
            out.push_str("          <ul class=\"clean\">\n");
            for issue in issues {
                let sev_class = severity_class(&issue.severity.to_string());
                let sev_label = severity_label(&issue.severity.to_string());

                out.push_str("            <li class=\"list-card\">\n");
                out.push_str("              <div class=\"issue-top\">\n");
                out.push_str(&format!(
                    "                <span class=\"issue-rule\">{}</span>\n",
                    escape_html(&issue.rule_name)
                ));
                out.push_str(&format!(
                    "                <span class=\"severity {}\">{}</span>\n",
                    sev_class, sev_label
                ));
                out.push_str("              </div>\n");
                out.push_str(&format!(
                    "              <p class=\"issue-message\">第 {} 行: {}</p>\n",
                    issue.position.line,
                    escape_html(&issue.message)
                ));

                if !issue.snippet.trim().is_empty() {
                    out.push_str(&format!(
                        "              <div class=\"issue-snippet\">{}</div>\n",
                        escape_html(issue.snippet.trim())
                    ));
                }

                out.push_str("            </li>\n");
            }
            out.push_str("          </ul>\n");
        }
        out.push_str("        </div>\n");

        out.push_str("      </article>\n");
    }
    out.push_str("    </section>\n");
    out.push_str("  </div>\n");
    out.push_str("</body>\n");
    out.push_str("</html>\n");

    out
}

fn visible_issue_count(report: &AnalysisReport) -> usize {
    report
        .files
        .iter()
        .map(|file| visible_issues(&file.issues).len())
        .sum()
}

fn visible_issues<T>(issues: &[T]) -> Vec<&T>
where
    T: HasSeverity,
{
    issues
        .iter()
        .filter(|issue| is_visible(issue.severity()))
        .collect()
}

fn is_visible(severity: &str) -> bool {
    matches!(severity.to_ascii_lowercase().as_str(), "warning" | "error")
}

fn metric_box(label: &str, value: usize) -> String {
    format!(
        "          <div class=\"metric-box\"><strong>{}</strong><span>{}</span></div>\n",
        label, value
    )
}

fn severity_label(severity: &str) -> &'static str {
    match severity.to_ascii_lowercase().as_str() {
        "error" => "严重",
        "warning" => "警告",
        _ => "",
    }
}

fn severity_class(severity: &str) -> &'static str {
    match severity.to_ascii_lowercase().as_str() {
        "error" => "severity-error",
        "warning" => "severity-warning",
        _ => "",
    }
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

trait HasSeverity {
    fn severity(&self) -> &str;
}

impl HasSeverity for crate::model::Issue {
    fn severity(&self) -> &str {
        match self.severity.to_string().to_ascii_lowercase().as_str() {
            "error" => "error",
            "warning" => "warning",
            _ => "info",
        }
    }
}
