# Rust期末项目

本项目是一个使用 Rust 编写的命令行代码静态分析工具，也是 Rust 课程期末项目。它可以递归扫描指定目录下的源码文件，统计基础代码指标，并根据预设规则检查常见代码质量问题，最后生成文本、JSON 或 HTML 格式的分析报告。

## 支持的文件类型

目前支持分析以下源码文件：

-   Rust：`.rs`
-   Python：`.py`
-   C/C++：`.c`、`.cc`、`.cpp`

默认分析目录为`./upload`，也可以通过命令行参数指定其他目录。

## 实现说明

本项目采用模块化设计，整体流程如下：

1.  命令行模块读取用户参数；
2.  根据参数生成分析配置；
3.  分析模块递归扫描目标目录；
4.  根据文件扩展名识别源码类型；
5.  对源码文件进行指标统计和规则检查；
6.  汇总生成统一分析报告；
7.  根据用户选择输出文本、JSON 或 HTML 报告。


## 项目功能

本项目主要实现了以下功能：

-   递归扫描指定目录中的源码文件
-   统计代码基础指标
-   检查常见代码质量问题
-   支持自定义规则阈值
-   支持 Text、JSON、HTML 三种输出格式
-   支持将分析结果输出到终端或保存为文件
-   提供测试用例验证核心功能

## 基础统计内容

程序会统计以下信息：

-   文件数量
-   总行数
-   代码行数
-   注释行数
-   空白行数
-   函数数量
-   平均行长度
-   最长行长度
-   最大嵌套深度

对于不同语言，还会统计部分语言相关结构，例如：

-   Rust：`mod`、`impl`、`struct`、`enum`、`trait`、`match`、`unsafe`
-   Python：`class`、循环、`match`、缩进深度
-   C/C++：`struct`、`enum`、`class`、循环、`switch`

## 规则检查

本项目实现了一些基础规则检查，包括：

-   Rust 中的`.unwrap()`、`.expect()`、`todo!()`、`unimplemented!()`、`dbg!()`、`unsafe`
-   Python 中的`eval()`、`exec()`、裸`except:`、`print()`、`pass`
-   C/C++ 中的`gets()`、`strcpy()`、`strcat()`、`printf()`、`cout <<`、`system()`
-   通用规则，如单行过长、函数过长、参数过多、文件过大、嵌套过深

问题级别分为：

-   `Info`：提示信息
-   `Warning`：警告问题
-   `Error`：严重问题

其中，文本报告和 HTML 报告主要展示`Warning`和`Error`，便于人工阅读；JSON 报告保留完整分析数据，便于后续处理。

## 项目结构

```
src/
├── main.rs
├── lib.rs
├── cli.rs
├── model.rs
├── analyzer.rs
└── report.rs

tests/
├── analyzer_test.rs
├── cli_test.rs
└── report_test.rs

upload/
├── main.rs
└── lab.py
```

各模块说明：

-   `main.rs`：程序入口
-   `lib.rs`：模块导出和公共接口
-   `cli.rs`：命令行参数解析和整体流程控制
-   `model.rs`：定义配置、报告、问题、错误等数据结构
-   `analyzer.rs`：实现核心静态分析逻辑
-   `report.rs`：生成文本、JSON 和 HTML 报告
-   `tests/`：测试代码目录
-   `upload/`：默认待分析文件目录

## 环境要求

运行本项目需要安装 Rust 开发环境。

可以通过以下命令检查 Rust 是否安装成功：

```
rustc --version
cargo --version
```

## 编译项目

```
cargo build
```

## 运行项目

默认分析`./upload`目录，并以文本格式输出：

```
cargo run
```

指定分析目录和输出格式：

```
cargo run -- --path ./upload --format text
```

## 输出 JSON 报告

输出到终端：

```
cargo run -- --path ./upload --format json
```

保存到文件：

```
cargo run -- --path ./upload --format json --output report.json
```

## 输出 HTML 报告

```
cargo run -- --path ./upload --format html --output report.html
```

生成后可以使用浏览器打开`report.html`查看分析结果。

## 命令行参数

```
Options:
  -p, --path <PATH>
          要分析的目录路径
          默认值：./upload

  -f, --format <FORMAT>
          输出格式
          可选值：text, json, html
          默认值：text

  -o, --output <OUTPUT>
          输出文件路径
          不指定时输出到终端

      --max-line-length <MAX_LINE_LENGTH>
          最大行长度
          默认值：120

      --max-function-length <MAX_FUNCTION_LENGTH>
          最大函数长度
          默认值：80

      --max-parameters <MAX_PARAMETERS>
          最大参数数量
          默认值：6

      --max-nesting-depth <MAX_NESTING_DEPTH>
          最大嵌套深度
          默认值：5

      --max-file-lines <MAX_FILE_LINES>
          最大文件行数
          默认值：800

  -h, --help
          打印帮助信息

  -V, --version
          打印版本信息
```

## 示例命令

分析默认目录：

```
cargo run
```

分析指定目录：

```
cargo run -- --path ./upload
```

生成文本报告：

```
cargo run -- --path ./upload --format text
```

生成 JSON 文件：

```
cargo run -- --path ./upload --format json --output report.json
```

生成 HTML 文件：

```
cargo run -- --path ./upload --format html --output report.html
```

自定义规则阈值：

```
cargo run -- --path ./upload --format text --max-line-length 100 --max-function-length 50 --max-parameters 4 --max-nesting-depth 4 --max-file-lines 500
```

## 运行测试

运行全部测试：

```
cargo test
```

测试内容主要包括：

-   分析功能测试
-   命令行功能测试
-   报告生成测试

## 代码格式检查

提交前可以运行：

```
cargo fmt
cargo clippy
```
无严重警告且通过代码检查

## 项目地址

[https://github.com/paradise611/rust\_homework.git](https://github.com/paradise611/rust_homework.git)