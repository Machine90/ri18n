# CHANGELOG

## 20261108

### ri18n
- 新增 TypeScript 代码生成后端（`Target::Typescript`），按与 Java/Rust 相同的 IR → `IntoXxx` 模式输出 `.ts` 模块（`translate` / `translateWithArgs`、`Keys`、`Iso639Codes`）
- 新增 Python 代码生成后端（`Target::Python`），输出 `.py` 模块（`@dataclass SupportLanguage`、`translate` / `translate_with_args`、`Keys`、`Iso639Codes`；占位符走 `str.format`）
- **Breaking**：生成 API 由 `get_by_locale` 重命名为 `translate`，带参接口改为独立名称 `translate_with_args` / `translateWithArgs`，避免无重载语言与 Rust 宏冲突
- **Breaking**：`I18nCodeGen::new` 改为返回 `Result`，加载失败时向上传播错误而不再静默/panic
- CLI 改为 `Result` 退出码路径，错误信息经 `anyhow` 输出；接入 `tracing` / `RUST_LOG` 可配置日志
- 加固 `I18nFileReader`：缺失路径、无法识别语言、空目录、YAML 解析失败等均有明确错误；同 locale 冲突 key 失败、同值重复可接受；跨 locale key 不一致时告警
- 稳定 codegen 输出顺序（`fn_match_entries` 等改用有序集合）；修复 `Package` 路径解析（`.` / `/` / `::`）；Rust `translate_with_args` 宏改为 `include_str!` 模板
- 补充 README（中/英）用法说明；增加 Java / Rust / TypeScript / Python golden 快照与 reader / package / naming 等单元测试
- 新增 `Taskfile.yml` 简化多平台发布构建（`task build:all`）
    1. macOS 主机：Apple 目标用原生 `cargo`；Linux musl 用 `cargo-zigbuild`；Windows gnu 用 `mingw-w64`
    2. Linux / Windows 主机：非本机目标仍走 `cross` + Docker
    3. `task setup` 自动安装 rustup target 与对应工具链依赖
- 整理 `Cross.toml`：注明 Linux/Windows 使用 `ghcr.io/cross-rs` 默认镜像即可；Apple 目标因 SDK 授权仅保留社区镜像说明（本机 Mac 优先不用 cross）

#### Breaking Change:
0.2.0
