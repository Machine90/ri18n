# Ri18n

[English](README.md) | [中文](README.zh-CN.md)

**Ri18n** turns i18n resource files into typed source code for your app.

You keep translations in JSON / YAML / TOML / `.properties`. Ri18n emits a single Java, Rust, TypeScript, or Python file with constants, a supported-language list, and `translate(locale, key)` lookup — so call sites use compile-time keys instead of raw strings.

| | |
|---|---|
| **Inputs** | `.json` · `.yaml` / `.yml` · `.toml` · `.properties` |
| **Targets** | Java · Rust · TypeScript · Python |
| **Version** | 0.2.0 |

---

## Quick start

```shell
# Java
ri18n -s path/to/i18n -t out/ -n Ri18n -p com.example.i18n java

# Rust
ri18n -s path/to/i18n -t out/ -n ri18n rust

# Rust with formatx macro for formatted strings
ri18n -s path/to/i18n -t out/ -n ri18n rust --formatx-enable

# TypeScript
ri18n -s path/to/i18n -t out/ -n Ri18n typescript

# Python
ri18n -s path/to/i18n -t out/ -n ri18n python

# Constants / language list only
ri18n -s path/to/i18n -t out/ -n Ri18nConstantsOnly -c java
```

`-n` / `--name` is used **as-is** for the type name and file stem (e.g. Java → `Ri18n.java`, Rust → `ri18n.rs`, TypeScript → `Ri18n.ts`, Python → `ri18n.py`). Choose a name that matches the target language’s conventions yourself.

---

## CLI reference

| Flag | Description |
|------|-------------|
| `-s`, `--src-dir` | Directory or single file of i18n resources (**required**) |
| `-t`, `--target-dir` | Output directory (default: `./`) |
| `-n`, `--name` | Generated type / file stem (default: `Ri18n`) |
| `-p`, `--package` | Optional package / module path: `com.a.b`, `com/a/b`, or `com::a::b` |
| `-c`, `--constants-only` | Emit constants and support-language list only (no `translate` APIs) |
| subcommand | `java` · `rust [--formatx-enable]` · `typescript` · `python` |

Environment variables with the same names as long flags are also accepted (via clap `env`).

---

## Input files

### Formats & keys

Nested maps are flattened to dotted keys:

```yaml
# i18n_en.yaml
animal:
  cat:
    name: "Tiger"
    desc: "Name is {s}, age is {int}"
```

→ keys `animal.cat.name`, `animal.cat.desc`.

### Locale detection

1. **Filename hint** — e.g. `i18n_zh.toml`, `messages-de.properties` → ISO 639-1  
2. **Content fallback** — `whatlang` when the filename has no valid code  

Files with an undetectable locale are skipped with a warning. If nothing loads, the CLI exits with an error.

Same key with **different** values in one file is an error; identical duplicates are ignored.

---

## Generated API (shape)

Typical output includes:

| Piece | Role |
|-------|------|
| `SupportLanguage` + language list | Enumerate / validate supported locales |
| `Iso639Codes` / `keys` constants | Typed locale codes and translation keys |
| `translate(locale, key)` | Lookup template / string |
| Formatted variant | Java / TypeScript: `translateWithArgs(...)` · Python: `translate_with_args(...)` · Rust: `translate_with_args!` (needs `--formatx-enable` + [`formatx`](https://crates.io/crates/formatx)) |

### Placeholders

Values may use typed placeholders; Ri18n rewrites them per target:

| Placeholder examples | Meaning |
|----------------------|---------|
| `{s}` `{str}` `{string}` `{text}` | string |
| `{int}` `{integer}` `{u32}` `{i64}` … | integer |
| `{float}` `{double}` `{f32}` `{f64}` | float |
| `{bool}` `{boolean}` | boolean |
| `{c}` `{char}` | char |
| `{hex}` `{exp}` | hex / exponent |

Java → `String.formatted` style (`%s`, `%d`, …). TypeScript → runtime `formatTemplate`. Python → `str.format` with `{}`. Rust + formatx → `{}`.

---

## Logging

Warnings go to stderr through `tracing` (default level `warn`).

```shell
RUST_LOG=debug ri18n -s ./i18n -t ./out -n Ri18n java
```

---

## Development

```shell
cargo nextest run
UPDATE_GOLDEN=1 cargo nextest run -E 'test(codegen)'   # refresh codegen snapshots
```

Requires [Task](https://taskfile.dev/) (`brew install go-task` on macOS).

---

## Cross-compile

Release binaries for macOS / Linux / Windows via `Taskfile.yml` (artifacts under `dist/<triple>/`):

```shell
task setup        # rustup targets + host tools (zig / mingw on macOS, cross elsewhere)
task build:all    # macOS + Linux musl + Windows gnu
task build:macos  # Apple Silicon + Intel
task build:linux  # x86_64 / aarch64 musl
task build:windows
```

On **macOS**, Linux targets use [`cargo-zigbuild`](https://github.com/rust-cross/cargo-zigbuild) and Windows uses `mingw-w64` (no Docker required). On **Linux / Windows** hosts, non-native targets still use [`cross`](https://github.com/cross-rs/cross) + Docker; see `Cross.toml` for Apple image notes.
