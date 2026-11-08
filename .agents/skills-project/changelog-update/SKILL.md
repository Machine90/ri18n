---
name: changelog-update
description: >-
  按仓库 CHANGELOG.md 约定追加或更新变更记录：按 YYYYMMDD 日期分节、
  按 crate/模块分节、条目下按兼容性写 Bump Version 或 Breaking Change；
  版本相对 main 按 MR 只定一次（兼容 patch / 破坏性升 minor），未合并前不叠升。
  当用户要求更新 changelog、写变更说明、bump/breaking 版本并记入 CHANGELOG，
  或提到 CHANGELOG.md 时激活。
---

# CHANGELOG 更新 Skill

## 概述

本仓库使用根目录 **`CHANGELOG.md`** 记录各 crate 的变更。更新 changelog 时须**同步**对应 crate 的 `Cargo.toml` `version`，两者写同一版本号。

| 项 | 约定 |
|----|------|
| 文件 | 仓库根 `CHANGELOG.md`（与 workspace `Cargo.toml` 同级） |
| 日期节 | `## YYYYMMDD`（紧凑日期，无分隔符；**新日期插在最前**） |
| 模块节 | `### <crate-name>`（与 `Cargo.toml` 的 `[package].name` 一致） |
| 条目 | `-` 列表描述「做了什么 / 为何」；细节可用缩进编号列表 |
| 版本标题 | 兼容 → `#### Bump Version:`；破坏性 → `#### Breaking Change:`；下一行写**本 MR 目标** semver |

## 版本号规则（兼容 vs Breaking）

先判断改动对**对外暴露接口**（公开 API、函数/方法签名、对外参数、调用方必须改动的契约）是否兼容，再决定如何升版本与 CHANGELOG 标题。版本形如 `MAJOR.MINOR.PATCH`（例：`0.1.0` / `1.0.0`）。

以 **main（或 MR 合入目标分支）上该 crate 的当前版本** 为基线：

| 类型 | 判定 | 相对 main 怎么变 | CHANGELOG 标题 | 示例（main = `1.0.0`） |
|------|------|------------------|----------------|------------------------|
| **Bump**（兼容） | 兼容性改动；对外暴露的接口与参数**无变化** | 升 **PATCH** | `#### Bump Version:` | `1.0.0` → `1.0.1` |
| **Breaking**（不兼容） | 不兼容外部调用（参数/签名变化、删除或重命名公开 API 等） | 升 **MINOR**，PATCH 归零 | `#### Breaking Change:` | `1.0.0` → `1.1.0` |

补充约定：

- **只看对外契约**：crate 内部私有改动、未导出类型/函数的调整 → 按 **Bump**。
- **新增**对外 API 且不破坏旧调用 → 按 **Bump**；用户另有要求时听用户的。
- **首次入库**（main 尚无该 crate）：本 MR 目标版本一般为 `0.1.0`（或用户指定），标题用 `#### Bump Version:`，不必再对「同 MR 内后续提交」空 bump。
- 每个模块节末尾**只写一个**版本标题：兼容用 `Bump Version`，破坏性用 `Breaking Change`，**不要**在 Breaking 场景误写成 Bump Version。

## MR 未合并：版本只定一次（重要）

**在代码 MR 合并进 main 之前**，该 MR / 功能分支上对某一 crate 的版本与 CHANGELOG 版本行 **只按上表相对 main 定一次**，之后无论同分支上再改多少次 changelog 条目或代码，**都不得再次叠升版本号**。

| 规则 | 说明 |
|------|------|
| 基线 | 用 main（合入目标）上该 crate 的 `version`，不要用本分支上「上一次自己 bump 过」的中间值当基线再加一档 |
| 定版 | 整次 MR 的改动集若整体兼容 → 一次 Bump（如 `1.0.0`→`1.0.1`，标题 `Bump Version`）；若含 Breaking → 一次 Breaking（如 `1.0.0`→`1.1.0`，标题 `Breaking Change`） |
| 冻结 | 定版后继续追加/润色 CHANGELOG、加单测、改 CI 等 → **Cargo.toml 与版本标题/版本号保持该 MR 目标不变** |
| 禁止叠升 | 未合并前**不要**在已是 `1.0.1` 时再改为 `1.0.2`，或在已是 `1.1.0` 时再改为 `1.2.0` / `1.1.1` |
| 严重级别上调（例外） | 若本 MR 原先按 Bump 写成 `1.0.1` + `Bump Version`，后来又引入 Breaking → **允许一次性改正**为 `1.1.0` + `Breaking Change`（相对 main 的 Breaking）；改正后再次冻结，仍不得继续叠升 |
| 合并之后 | 下一次新 MR 再以**新的** main 版本为基线重新按表计算 |

示例：`main` 为 `1.0.0`，分支 `feat/rust-sdk`：

1. 首次按方案定版：Bump → CHANGELOG / Cargo 均为 `1.0.1`，标题 `#### Bump Version:`。
2. 同分支后续提交只更新条目文案 → 仍写 `1.0.1`，**不**改成 `1.0.2`。
3. 若中途变成 Breaking → 改为 `1.1.0` + `#### Breaking Change:` 一次；之后仍保持该目标。

Agent：**未合并 MR 内更新 changelog 时，默认只改条目、不改版本号**；仅在「尚未相对 main 定过版」或「需从 Bump 上调为 Breaking」时改 `Cargo.toml` + 版本标题/版本号。

## 文档骨架

```markdown
# CHANGELOG

## YYYYMMDD

### <crate-name>
- <变更说明>
    1. <可选细节>
    2. <可选细节>

#### Bump Version:
<x.y.z>

### <another-crate>
- <破坏性变更说明>

#### Breaking Change:
<a.b.0>
```

### 真实格式对照（摘自现有文件）

```markdown
# CHANGELOG

## 20261023

### core-proto-compiler
- 增加 core-proto-compiler 模块，支持通过 `ProtoCompiler` 加载指定 protobuf url …
    1. 文件类型 url: file://*.tar.gz
    2. 路径类型 url: file://path/to/protobuf/
    3. http 类型 url: http(s)://host:port/*.tar.gz

#### Bump Version:
0.1.0

### core-proto
- 增加 core-proto 模块，支持通过读取 `PROTO_URL` …

#### Bump Version:
0.1.0
```

## 何时写、写什么

| 场景 | 动作 |
|------|------|
| 新增 crate / 模块 | 在**当天**日期节下新增 `### <name>` + 条目 + 对应版本标题（本 MR 目标版） |
| 已有 crate 行为/API 变更 | 在当天日期节对应模块下追加 `-` 条目；版本遵循「MR 未合并只定一次」 |
| 仅文档/注释/格式、无用户可见或包行为变化 | **默认不写**；用户明确要求时再记 |
| 同一 MR / 同一天多次更新同一 crate | **合并进同一** `###` 节，追加 bullet；**不叠升** 版本号 |
| 跨天（仍同一未合并 MR） | 可新建 `## YYYYMMDD`，但各 crate 的版本行仍用**本 MR 已定目标版**，勿按天再 bump |
| 跨天且已合并后再开新 MR | 新日期节 + 相对**新** main 重新定版 |

## Agent 操作流程

1. **读**现有 `CHANGELOG.md`，确认标题为 `# CHANGELOG`。
2. **取今日日期** `YYYYMMDD`（以用户环境/会话日期为准）。
3. **定位日期节**：
   - 已有 `## YYYYMMDD` → 在其下按 crate 追加或合并；
   - 没有 → 在 `# CHANGELOG` 后插入新的 `## YYYYMMDD` 块（保持空行风格与现文件一致）。
4. **确定涉及的 crate**：以实际改动的 `crates/<name>` / `[package].name` 为准；一节一个 crate。
5. **写条目**：
   - 用中文完整句子或短语，说明能力/行为变化（可含关键标识符、路径、环境变量名）；
   - 多点细节用缩进编号列表（4 空格 + `1.`）；
   - 避免空洞词（「更新代码」「微调」）；不写无关文件清单。
6. **版本（见「版本号规则」+「MR 未合并」）**：
   - 查 main（或合入目标）上该 crate 的 `version` 作为基线；
   - 若本 MR **尚未**定过目标版 → 按 Bump / Breaking 算一次，写入 `Cargo.toml` 与对应版本标题；
   - 若本 MR **已有**目标版 → **保持不变**，只改条目；仅当需从 Bump **上调**为 Breaking 时改一次（同时把标题改为 `Breaking Change`）；
   - 格式固定为（兼容 / 破坏性二选一）：

     ```markdown
     #### Bump Version:
     1.0.1
     ```

     ```markdown
     #### Breaking Change:
     1.1.0
     ```

     （`####` 标题与版本号分行；版本号单独一行，无引号、无 `v` 前缀。）
7. **不要**擅自改历史日期节里其他 crate 的表述；**不要**删除旧日期节。

## 文风与命名

- **模块标题**：优先用 Cargo package name（如 `core-proto-compiler`）。若历史节用了简称，新节仍用正式 package name，不必回溯改名除非用户要求统一。
- **语言**：与现有 CHANGELOG 一致，默认中文；专有名词、路径、URL scheme、环境变量保持原文（如 `PROTO_URL`、`file://`）。
- **一条 bullet 一件事**；同一 crate 多变更用多条 `-`，或一条总述 + 编号细节。

## 检查清单

- [ ] 版本相对 **main / 合入目标** 计算，且本 MR **未叠升**（无 `1.0.1`→`1.0.2` 这类二次 bump）
- [ ] 仅在首次定版或 Bump→Breaking 上调时改了 `Cargo.toml`；否则版本行未动
- [ ] 对应 crate 的 `Cargo.toml` `version` 与 CHANGELOG 版本行一致
- [ ] 兼容用 `#### Bump Version:`，破坏性用 `#### Breaking Change:`（二者勿混用）
- [ ] 日期为 `## YYYYMMDD`，新日期在文件上部
- [ ] `###` 对应真实 crate / package name
- [ ] 每个有版本含义的模块节含正确版本标题 + 下一行 semver
- [ ] 条目可读、与本次改动一致；无密钥/本机绝对路径
- [ ] 未改写无关历史节

## 与其它 skill

- **版本号**在对应 crate 的 `Cargo.toml` 修改，CHANGELOG 必须同步同一数字；未合并 MR 内遵循「只定一次」。
- **提交信息**仍按用户/仓库 git 约定；不要把整段 CHANGELOG 粘进 commit message，除非用户要求。
