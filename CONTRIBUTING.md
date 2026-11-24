# Development Guide

> AI agents: see [AGENTS.md](./AGENTS.md) for agent-specific instructions.

## Requirements

|                          Toolchain                          | Version |
| :---------------------------------------------------------: | :-----: |
|                 [Rust](https://rustup.rs/)                  | ^1.77.0 |
|            Rust nightly with miri and rust-src             | latest (for `just dev` / `just miri`) |
|            [just](https://github.com/casey/just)            | ^1.36.0 |
| [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) | ^0.6.21 |
| [cargo-audit](https://github.com/rustsec/rustsec/tree/main/cargo-audit) | latest (for `just audit`) |
| [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) | latest (for `just audit`) |
| [cargo-edit](https://github.com/killercup/cargo-edit) | latest (for `just sync-version`) |

## Workflow

### Download source code

```bash
git clone https://github.com/Nugine/const-str.git
cd const-str
```

### Run basic checks and tests

```bash
just dev
```

### Open documentation

```bash
just doc
```

### Generate test coverage

```bash
just coverage
```

This will generate an HTML coverage report in `target/llvm-cov/html/index.html`.

### Run CI checks locally

```bash
just ci
```

`just ci` runs fmt, clippy, tests, and miri. Audit/deny checks are separate:

### Run audit and deny checks

```bash
just audit
```

This requires `cargo-audit` and `cargo-deny`.

## Release

Publishing uses crates.io trusted publishing; no long-lived crates.io token is stored in the repository. Only maintainers publish.

1. Bump the workspace and crate versions: `just sync-version X.Y.Z`.
2. Run `cargo test --all-features --locked` and `just audit`.
3. Commit `release vX.Y.Z`.
4. Push a `vX.Y.Z` tag. This triggers the Publish workflow; creating a GitHub Release alone does not publish.
5. The workflow validates the tag against both crate versions, runs tests, and publishes `const-str-proc-macro` then `const-str`.

Both crates must have a crates.io trusted publisher configured for this repository, workflow `publish.yml`, and environment `crates.io`.

## Git

### Commit Message

We follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification.

### Pull Request

Before creating or updating a pull request, please make sure to run the following commands:

```bash
just dev
```

It's recommended to resolve any warnings or errors before submitting.

Note that some lints may be too restrict. You can allow some of them if you think they are not helpful to the code quality.
