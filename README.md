# rust_homework

`rust_homework` 是一个用 Rust 编写的简单代码静态分析工具，适合作为 Rust 课程期末项目。它会递归扫描指定目录中的 `.rs` 文件，统计基本代码指标，并检查一些常见的代码质量问题。

## 项目特点

- 主要代码全部使用 Rust 编写
- 支持命令行运行
- 使用 `lib.rs` 和 `cli.rs` 做职责划分
- 测试文件全部放在 `tests/` 目录
- 支持文本、JSON、HTML 三种输出格式
- 代码结构不复杂，适合单人项目

## 功能

### 1. 基本统计

- 文件数量
- 总行数
- 代码行数
- 注释行数
- 空行数
- 函数数量
- 模块数量
- `struct / enum / trait / impl` 数量
- `match / loop / unsafe` 统计
- 最大嵌套深度
- 平均行长度
- 最长行长度

### 2. 规则检查

- `unwrap()` 使用检查
- `expect()` 使用检查
- `todo!()` 检查
- `unimplemented!()` 检查
- `panic!()` 检查
- `dbg!()` 检查
- `unsafe` 检查
- 过长行检查
- 过长函数检查
- 参数过多检查
- 嵌套过深检查
- 文件过大检查
- 文件末尾多余空行检查
- 注释占比过高检查
- 单字符变量名检查

## 运行方式

### 文本输出

```bash
cargo run -- --path . --format text

Options:
  -p, --path <PATH>                    要分析的目录路径 [default: ./upload]
  -f, --format <FORMAT>                输出格式 [default: text] [possible values: text, json, html]
  -o, --output <OUTPUT>                输出文件路径（不指定则输出到终端）
      --max-line-length <MAX_LINE_LENGTH>          最大行长度 [default: 120]
      --max-function-length <MAX_FUNCTION_LENGTH>  最大函数长度 [default: 80]
      --max-parameters <MAX_PARAMETERS>            最大参数数量 [default: 6]
      --max-nesting-depth <MAX_NESTING_DEPTH>      最大嵌套深度 [default: 5]
      --max-file-lines <MAX_FILE_LINES>            最大文件行数 [default: 800]
  -h, --help                           Print help
  -V, --version                        Print version