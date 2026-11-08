# Ri18n

[English](README.md) | [中文](README.zh-CN.md)

**Ri18n** 把 i18n 资源文件生成成目标语言的类型化源码。

你用 JSON / YAML / TOML / `.properties` 维护翻译；Ri18n 输出一份 Java、Rust 或 TypeScript 文件，内含常量、支持语言列表，以及 `translate(locale, key)` 查表 API——调用方用编译期常量 key，而不是散落的裸字符串。

| | |
|---|---|
| **输入** | `.json` · `.yaml` / `.yml` · `.toml` · `.properties` |
| **目标语言** | Java · Rust · TypeScript |
| **版本** | 0.2.0 |

---

## 快速开始

```shell
# Java
ri18n -s path/to/i18n -t out/ -n Ri18n -p com.example.i18n java

# Rust
ri18n -s path/to/i18n -t out/ -n ri18n rust

# Rust：启用 formatx 宏做带参格式化
ri18n -s path/to/i18n -t out/ -n ri18n rust --formatx-enable

# TypeScript
ri18n -s path/to/i18n -t out/ -n Ri18n typescript

# 仅生成常量与支持语言列表
ri18n -s path/to/i18n -t out/ -n Ri18nConstantsOnly -c java
```

`-n` / `--name` 会**原样**用作类型名与文件名（例如 Java → `Ri18n.java`，Rust → `ri18n.rs`，TypeScript → `Ri18n.ts`）。请按目标语言命名规范自行选择（如 Java 用 `Ri18nConstantsOnly`）。

---

## 命令行参数

| 参数 | 说明 |
|------|------|
| `-s`, `--src-dir` | i18n 资源目录或单个文件（**必填**） |
| `-t`, `--target-dir` | 输出目录（默认 `./`） |
| `-n`, `--name` | 生成类型 / 文件名主干（默认 `Ri18n`） |
| `-p`, `--package` | 可选包 / 模块路径：`com.a.b`、`com/a/b` 或 `com::a::b` |
| `-c`, `--constants-only` | 只生成常量与支持语言列表（不含 `translate` API） |
| 子命令 | `java` · `rust [--formatx-enable]` · `typescript` |

长选项同名环境变量也可传入（clap `env`）。

---

## 输入文件

### 格式与 key

嵌套 map 会展平为点分 key：

```yaml
# i18n_en.yaml
animal:
  cat:
    name: "Tiger"
    desc: "Name is {s}, age is {int}"
```

→ `animal.cat.name`、`animal.cat.desc`。

### 语言识别

1. **文件名提示** — 如 `i18n_zh.toml`、`messages-de.properties` → ISO 639-1  
2. **内容回退** — 文件名无法识别时用 `whatlang` 检测  

无法识别语言的文件会跳过并告警；若最终没有任何条目，CLI 以错误退出。

同一文件中同一 key **不同 value** 会报错；完全相同的重复则忽略。

---

## 生成 API（结构）

典型产物包括：

| 部分 | 作用 |
|------|------|
| `SupportLanguage` + 语言列表 | 枚举 / 校验当前是否支持某语言 |
| `Iso639Codes` / `keys` 常量 | 类型化的语言码与翻译 key |
| `translate(locale, key)` | 按语言与 key 查字符串 / 模板 |
| 带参变体 | Java / TypeScript：`translateWithArgs(...)` · Rust：`translate_with_args!`（需 `--formatx-enable` 且依赖 [`formatx`](https://crates.io/crates/formatx)） |

### 占位符

文案中可使用类型化占位符，Ri18n 会按目标语言改写：

| 占位符示例 | 含义 |
|------------|------|
| `{s}` `{str}` `{string}` `{text}` | 字符串 |
| `{int}` `{integer}` `{u32}` `{i64}` … | 整数 |
| `{float}` `{double}` `{f32}` `{f64}` | 浮点 |
| `{bool}` `{boolean}` | 布尔 |
| `{c}` `{char}` | 字符 |
| `{hex}` `{exp}` | 十六进制 / 科学计数 |

Java → `String.formatted` 风格（`%s`、`%d` 等）。TypeScript → 运行时 `formatTemplate`。Rust + formatx → `{}`。

---

## 日志

告警通过 `tracing` 输出到 stderr（默认级别 `warn`）。

```shell
RUST_LOG=debug ri18n -s ./i18n -t ./out -n Ri18n java
```

---

## 开发

```shell
cargo nextest run
UPDATE_GOLDEN=1 cargo nextest run -E 'test(codegen)'   # 刷新代码生成快照
```

需要安装 [Task](https://taskfile.dev/)（macOS：`brew install go-task`）。

---

## 交叉编译

通过 `Taskfile.yml` 发布 macOS / Linux / Windows 二进制（产物在 `dist/<triple>/`）：

```shell
task setup        # rustup target + 主机工具（macOS 装 zig/mingw，其它平台装 cross）
task build:all    # macOS + Linux musl + Windows gnu
task build:macos  # Apple Silicon + Intel
task build:linux  # x86_64 / aarch64 musl
task build:windows
```

在 **macOS** 上：Linux 走 [`cargo-zigbuild`](https://github.com/rust-cross/cargo-zigbuild)，Windows 走 `mingw-w64`（无需 Docker）。在 **Linux / Windows** 主机上：非本机目标仍用 [`cross`](https://github.com/cross-rs/cross) + Docker；Apple 镜像说明见 `Cross.toml`。
