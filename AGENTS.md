# Agent Guide

This is the canonical instruction file for AI agents working on `const-str`.
`.github/copilot-instructions.md` is a symlink to this file; edit only `AGENTS.md`.

## Read first

- `CONTRIBUTING.md`: prerequisites, local checks, coverage, commit/PR, and release/publishing process. Keep that information there; this file does not duplicate it.
- `justfile`: available recipes.
- `.github/workflows/`: CI, Audit, and Publish definitions.

## Repository map

- `crates/const-str/`: main library crate.
- `crates/const-str/src/__ctfe/`: const-evaluation types backing exported macros.
- `crates/const-str/src/__proc/` and `crates/const-str-proc-macro/`: proc-macro implementations.
- `crates/const-str/src/{str,bytes,slice,utf8,utf16,...}.rs`: shared string/byte helpers.
- The crate `README.md` files are symlinks to the root `README.md`; edit the root file only.

## Working rules

- Run the checks described in `CONTRIBUTING.md` before proposing changes; see `justfile` for exact recipes.
- Keep MSRV 1.77.0 and no_std compatibility. Check the MSRV CI result when touching APIs with newer stabilization requirements.
- Follow the existing macro pattern: `__ctfe` struct + const-eval implementation + `macro_export` + docs, with unit tests.
- Add tests for new macro behavior. For APIs that claim std parity, prefer differential tests against std.
- Run miri when changing unsafe or const-eval code.
- Codecov patch coverage is enforced; project coverage is informational. Add tests rather than lowering coverage gates.
- Do not run `cargo publish`, `cargo release`, or push `v*` tags. Publishing is limited to maintainers/CI via trusted publishing; follow `CONTRIBUTING.md`'s release section.
- Remote writes (pushing branches, creating/updating PRs, deleting branches, etc.) require explicit user approval.
- Use `just sync-version X.Y.Z` for version bumps; do not hand-edit versions.
- Document intentional differences from std and cover them with tests.
