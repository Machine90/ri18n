---
name: rust-unittest
description: >-
  用 rstest（case / fixture）+ test-log + nextest 编写与运行 Rust 单元测试；
  异步测试按需挂 tokio runtime。对外 pub 接口放 tests/test_xxx.rs（workspace 则为
  crates/<crate>/tests/test_xxx.rs），pub(crate) 及更窄（pub(super) / pub(self) /
  私有）写在定义所在的 .rs 里。当用户写/改 Rust 单测、参数化用例、fixture、
  async test、cargo nextest、决定测试放 tests/ 还是源文件，或提到 rust-unittest 时激活。
---

# Rust Unittest（rstest + nextest）

## 原则

1. **按可见性放测试**：对外 `pub` 接口写在 `tests/test_xxx.rs`；`pub(crate)` 及更窄（`pub(super)` / `pub(self)` / 私有）写在定义所在的 `.rs` 里。
2. **参数化优先**：多组输入/期望用 `#[rstest]` + `#[case]`，不要复制粘贴多个几乎相同的 `#[test]`。
3. **共享准备用 fixture**：重复的 setup（路径、客户端、临时目录）抽成 `#[fixture]`，不要在每个 case 里手写一遍。
4. **日志可观测**：测试函数加 `#[test_log::test]`（或包一层 async runtime），本地用 `RUST_LOG=debug` 排障。
5. **用 nextest 跑**：本地与 CI 统一 `cargo nextest run`，不要默认教用户只写 `cargo test`。
6. **异步显式 runtime**：`async fn` 测试必须指定 tokio（或项目选定的）异步环境；按场景选线程数。

## 测试放哪

先看被测项的可见性，再决定文件。两种位置不要混用。

| 可见性 | 谁能调用 | 测试位置 |
|--------|----------|----------|
| `pub`，且从 crate 根到该项的路径全是 `pub`（含 `pub use` 再导出） | 外部 crate | `tests/test_<模块或接口>.rs`（workspace：`crates/<crate>/tests/...`） |
| `pub(crate)`、`pub(super)`、`pub(self)`、`pub(in path)`、无修饰的私有项 | 仅本 crate，或更窄 | 定义所在的那个 `.rs` 末尾 `#[cfg(test)] mod tests` |
| 写了 `pub`，但所在模块不是对外公开的（父模块是 `pub(crate)` / 私有） | 外部 crate 到不了 | 同上，写在定义所在的 `.rs` 里 |

`tests/` 下每个 `test_*.rs` 是独立集成测试 crate，只能看见对外 API，访问不到 `pub(crate)` 和私有项。同文件里的 `mod tests` 用 `use super::*` 能看见该模块的私有项。私有逻辑不要挪到 `tests/`；对外接口不要只藏在源文件里测。

### 对外 `pub`：`tests/test_xxx.rs`

以一个对外公开的斐波那契 API 为例：

```
src/fibonacci.rs          # pub fn fibonacci(...)
tests/test_fibonacci.rs   # 只测对外契约
```

`tests/test_fibonacci.rs` 本身就是测试入口。不要再包 `#[cfg(test)] mod tests`，也不要 `use super::*`。用 crate 名当外部调用方导入：

```rust
use my_crate::fibonacci;
use rstest::rstest;

#[rstest]
#[test_log::test]
#[case(0, 0)]
#[case(1, 1)]
#[case(10, 55)]
fn test_fibonacci_public_contract(#[case] n: u32, #[case] expected: u64) {
    assert_eq!(fibonacci(n), expected);
}
```

约定：

- 文件名是 `test_` + 模块名或对外入口名：`test_fibonacci.rs`、`test_parser.rs`。一个对外模块或一组相关 `pub` API 一个文件。
- 只测外部契约：入参、返回值、可观察副作用。不要为了测内部细节把私有项改成 `pub`。
- 对外行为从 `pub` 入口覆盖；这个入口内部的私有辅助函数，分支仍留在定义所在的 `.rs` 里测。
- 多个 `test_*.rs` 共享 fixture / 帮助函数时，放到 `tests/common/mod.rs`，测试文件里写 `mod common;`。不要建 `tests/common.rs`：`tests/` 根下的 `.rs` 会被编成另一个集成测试 crate。
- `cargo nextest run -p <crate>` 会同时跑 `src` 里的单测和 `tests/` 里的集成测试。

### `pub(crate)` 及更窄：所在 `.rs` 的 `mod tests`

`pub(crate)`、`pub(super)`、`pub(self)`、`pub(in path)` 和私有函数，测试写在定义它们的文件底部：

```rust
fn fib_pair(n: u32) -> (u64, u64) {
    // 私有辅助：返回 (fib(n), fib(n+1))
    // ...
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[test_log::test]
    #[case(0, (0, 1))]
    #[case(5, (5, 8))]
    fn test_fib_pair(#[case] n: u32, #[case] expected: (u64, u64)) {
        // use super::* 可见同模块的 pub(crate) 与私有项
        assert_eq!(fib_pair(n), expected);
    }
}
```

约定：

- 测试跟被测项同一个文件：`fibonacci.rs` 里的非对外逻辑测在 `fibonacci.rs`，不要堆到 `lib.rs`，也不要放到 `tests/`。
- 子模块同理：各自的 `pub(crate)` / 私有项在该文件内测。
- 集成测试 crate 编译期就访问不到这些项，把它们写进 `tests/test_xxx.rs` 过不了编译。

## 依赖（workspace / crate）

在 workspace `[workspace.dependencies]` 与 crate `[dev-dependencies]` 中按需声明；单 crate 仓库则直接写在根 `Cargo.toml`：

```toml
# workspace Cargo.toml（若有）
[workspace.dependencies]
rstest = { version = "0.25" }
test-log = { version = "0.2" }
# 仅当有 async 测试时
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }

# crate Cargo.toml
[dev-dependencies]
rstest = { workspace = true }       # 或直接写 version
test-log = { workspace = true }
# tokio = { workspace = true }  # 有 async 测试再加
```

- 同步测试：**不要**为测试强行引入 tokio。
- `test-log` 需要配合 `tracing` / `env_logger` 等（按项目已有日志栈）；属性写法仍是 `#[test_log::test]`。

## 同步参数化（基准写法）

下面是**同文件 `mod tests`** 的写法，被测函数是私有的（见「测试放哪」）：

```rust
#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[test_log::test]
    #[case(0, 0)]
    #[case(1, 1)]
    #[case(2, 1)]
    #[case(10, 55)]
    fn test_fibonacci(#[case] n: u32, #[case] expected: u64) {
        assert_eq!(fibonacci(n), expected);
    }
}
```

要点：

| 属性 / 参数 | 作用 |
|-------------|------|
| `#[rstest]` | 启用参数化；每个 `#[case]` 生成一个独立测试 |
| `#[case(...)]` | 一组实参，顺序对应函数上的 `#[case]` 形参 |
| `#[case] n: u32` | 声明该参数来自 case |
| `#[test_log::test]` | 替代裸 `#[test]`，打开 test-log |
| 返回 `Result<()>` | 可用 `?`；失败用 `assert_*` / `panic!` |

多参数 case 示例（含 `Result` 期望）：

```rust
#[rstest]
#[test_log::test]
#[case(0, Ok(0))]
#[case(1, Ok(1))]
#[case(93, Err(()))] // 溢出上界示例
fn test_fibonacci_checked(
    #[case] n: u32,
    #[case] expected: Result<u64, ()>,
) {
    let got = fibonacci_checked(n).map_err(|_| ());
    assert_eq!(got, expected);
}
```

## Fixture

用 `#[fixture]` 共享构造逻辑；测试参数直接按名注入（类型匹配即可）。

```rust
use rstest::{fixture, rstest};

#[fixture]
fn known_sequence() -> Vec<u64> {
    vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
}

#[rstest]
#[test_log::test]
#[case(0)]
#[case(5)]
#[case(10)]
fn test_fibonacci_matches_sequence(
    known_sequence: Vec<u64>, // 自动注入 fixture
    #[case] n: usize,
) {
    assert_eq!(fibonacci(n as u32), known_sequence[n]);
}
```

规则：

- fixture 函数名 = 测试参数名（rstest 按名注入）。
- fixture 可依赖其它 fixture：`fn client(base_url: String) -> Client { ... }`。
- 需要一次性昂贵 setup 时，优先 fixture；不要在每个 `#[case]` 里重复 IO。
- 临时资源（目录/端口）在 fixture 内创建，靠 `Drop`（如 `TempDir`）清理。

## 异步测试（tokio）

`async fn` **必须**指定 runtime。约定用 `test_log` 包一层 `tokio::test`：

```rust
#[rstest]
#[test_log::test(tokio::test(flavor = "multi_thread", worker_threads = 1))]
#[case(10)]
#[case(20)]
async fn test_fibonacci_async(#[case] n: u32) {
    let out = fibonacci_async(n).await;
    assert!(out > 0 || n == 0);
}
```

### 如何选 `worker_threads`

按**被测代码真实并发需求**选，不要一律多线程：

| 场景 | 推荐 |
|------|------|
| 单任务 async（一次 await、无 spawn 扇出） | `worker_threads = 1` |
| 内部 `tokio::spawn` / 多任务 join，但仍轻量 | `worker_threads = 2` 或少量固定值 |
| 明确需要并行跑多个 CPU/IO bound 任务，或测 runtime 调度 | 按并行度设（如 4）；并在注释说明原因 |
| 只要 current-thread 语义（无多线程调度） | `flavor = "current_thread"`（无 `worker_threads`） |

示例 — current_thread：

```rust
#[rstest]
#[test_log::test(tokio::test(flavor = "current_thread"))]
async fn test_async_current_thread() {
    // ...
}
```

示例 — 需要更多 worker：

```rust
#[rstest]
#[test_log::test(tokio::test(flavor = "multi_thread", worker_threads = 4))]
async fn test_async_fanout() {
    // 被测逻辑会 spawn 多个任务并 join
}
```

注意：

- 同步测试继续用 `#[test_log::test]`，**不要**套 tokio。
- `#[rstest]` 在上，`#[test_log::test(...)]` 在下（或紧跟），再跟 `#[case]`。
- 异步测试里同样可用 `#[case]` / fixture；fixture 若是 sync，rstest 会照常注入；需要 async fixture 时再查 rstest 文档，优先保持 fixture 同步。

## 用 nextest 运行

```bash
# 整个 workspace（与 CI 对齐）
cargo nextest run --workspace --no-fail-fast --no-tests pass

# 单 crate / 过滤
cargo nextest run -p my-crate
cargo nextest run -p my-crate -- test_fibonacci

# 打开日志
RUST_LOG=debug cargo nextest run -p my-crate -- test_fibonacci
```

说明：

- 本地复现问题优先与 CI 同一套 nextest 命令。
- `--no-tests pass`：某 package 无测试时不失败（workspace 友好）。
- 未安装时：`cargo install --locked cargo-nextest`。
- 需要 libtest 兼容输出时可用 `cargo test`，但**默认工作流是 nextest**。

## Agent 操作流程

写或改单测时按序执行：

1. 按可见性选定位置。对外 `pub`（从 crate 根可达）→ 新建或补 `tests/test_xxx.rs`。`pub(crate)` / `pub(super)` / `pub(self)` / `pub(in path)` / 私有 → 写在定义所在 `.rs` 的 `#[cfg(test)] mod tests`。
2. 判断同步 vs 异步；异步则选 `flavor` / `worker_threads`。
3. 多组输入 → `#[rstest]` + `#[case]`；重复 setup → `#[fixture]`。
4. 挂 `#[test_log::test]` 或 `#[test_log::test(tokio::test(...))]`。
5. 断言清晰：`assert_eq!` / `assert!`；`Result` 期望可用 `Ok`/`Err` 分支比较。对外接口只断言外部可观察行为。
6. 用 `cargo nextest run -p <crate> -- <filter>` 验证；全量再用 workspace 命令。`tests/` 与源文件内的 `mod tests` 都会被这条命令跑到。

## 检查清单

- [ ] 对外 `pub` 接口在 `tests/test_xxx.rs`，用 crate 名导入，没有再套 `#[cfg(test)] mod tests`
- [ ] `pub(crate)` / `pub(super)` / `pub(self)` / 私有项的测试在定义所在 `.rs` 内，并用 `use super::*`
- [ ] 参数化用了 `rstest` + `case`，无大量复制的 `#[test]`
- [ ] 共享 setup 用了 `fixture`（若适用）
- [ ] 同步：`#[test_log::test]`；异步：带 tokio 的 `test_log::test(...)`
- [ ] `worker_threads` / `flavor` 与被测并发匹配，并避免无意义的多线程
- [ ] crate `[dev-dependencies]` 已声明 `rstest` / `test-log`（及必要时的 `tokio`）
- [ ] 已用 `cargo nextest run` 跑通相关用例

## 反模式

- 对外 `pub` 接口只在源文件 `mod tests` 里测，没有 `tests/test_xxx.rs`
- 把 `pub(crate)` 或私有函数的测试写进 `tests/`（集成测试 crate 访问不到）
- 为了在 `tests/` 里测内部实现，把私有项改成 `pub`
- 建 `tests/common.rs` 当共享模块（会被编成独立测试目标）；共享代码用 `tests/common/mod.rs`
- 为同步逻辑强行加 `tokio::test`
- 所有 async 测试都写很大的 `worker_threads`「以防万一」
- 用多个几乎相同的 `#[test]` 代替 `#[case]`
- 在 case 里硬编码本机绝对路径或密钥
- 只写 `cargo test` 文档、与 CI nextest 脱节

## 与其它 skill

- 依赖版本放 workspace / 根 `Cargo.toml`；本机 env 与 CI 流程按仓库既有约定。
- 变更说明与版本 bump 见 `changelog-update` skill。
