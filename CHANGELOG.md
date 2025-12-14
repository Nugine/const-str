# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - Unreleased

### 🎉 Stable Release

This is the first stable release of `const-str`! The API is now considered stable and will follow semantic versioning guarantees.

#### Added
- **API Review Documentation** (`API_REVIEW_V1.0.0.md`)
  - Comprehensive review of all 42 public macros
  - Stability and compatibility analysis
  - Security and performance assessment
  - Recommendations for long-term maintenance

- **Stability Policy** (`STABILITY_POLICY.md`)
  - Clear semantic versioning commitment
  - MSRV update policy
  - Deprecation and breaking change guidelines
  - User-facing stability guarantees

- **Changelog** (`CHANGELOG.md`)
  - Track all changes going forward
  - Following Keep a Changelog format

#### Stability Guarantees

Starting with v1.0.0, we guarantee:

- ✅ **Backward Compatibility**: No breaking changes within major versions
- ✅ **Semantic Versioning**: Strict adherence to semver
- ✅ **MSRV Policy**: Clear policy for Minimum Supported Rust Version updates
- ✅ **Deprecation Process**: 3 minor version warning period before removal
- ✅ **API Stability**: All documented macros and functions are stable

#### Confirmed Stable APIs

**Core String Operations (13 macros)**
- `concat!`, `join!`, `repeat!`, `replace!`
- `split!`, `split_inclusive!`, `split_ascii_whitespace!`, `split_lines!`
- `squish!`, `trim_ascii!`, `trim_ascii_start!`, `trim_ascii_end!`
- `from_utf8!`

**Comparison & Search (9 macros)**
- `equal!`, `compare!`, `contains!`
- `starts_with!`, `ends_with!`
- `strip_prefix!`, `strip_suffix!`
- `eq_ignore_ascii_case!`, `is_ascii!`

**Conversion & Encoding (8 macros)**
- `to_str!`, `to_byte_array!`, `to_char_array!`
- `encode!`, `encode_z!`, `hex!`
- `parse!`, `unwrap!`

**Format & Case (4 macros)**
- `format!` (requires `proc` feature)
- `convert_ascii_case!`
- `convert_case!` (requires `case` feature)
- `cstr!`, `raw_cstr!`

**Advanced Features (8 macros)**
- `chain!`, `sorted!`, `ip_addr!`, `concat_bytes!`
- `verified_header_name!` (requires `http` feature)
- `verified_regex!` (requires `regex` feature)
- `regex_assert_match!` (requires `regex` feature)

**Feature Flags**
- `default`: Minimal functionality
- `std`: Standard library support
- `proc`: Procedural macro support
- `regex`: Regex pattern validation
- `http`: HTTP header validation
- `case`: Case conversion utilities
- `all`: Enable all features
- `unstable`: Experimental features (NOT stable)

**Public Modules**
- `ascii`: ASCII utility functions
- `bytes`: Byte slice operations
- `str`: String operations
- `slice`: Slice utilities
- `utf8`: UTF-8 encoding/decoding
- `utf16`: UTF-16 encoding
- `printable`: Printable character detection

#### Technical Details

- **MSRV**: Rust 1.77.0
- **Tests**: 108 unit tests + 50 doc tests (all passing)
- **Dependencies**: All stable and widely-used crates
- **Security**: No known vulnerabilities
- **Platform**: Works on all Rust targets, including `no_std`

#### Migration from v0.7.1

**No breaking changes!** 

All code that works with v0.7.1 will continue to work with v1.0.0. Simply update your `Cargo.toml`:

```toml
# Before
const-str = "0.7.1"

# After
const-str = "1.0"
```

#### Notes

- The `__ctfe` and `__proc` modules remain hidden from documentation and are not part of the stability guarantee
- The `unstable` feature flag is experimental and not covered by stability guarantees
- All dependency versions are current and maintained

#### For Library Authors

If you're building a library that depends on `const-str`, we recommend:

```toml
[dependencies]
const-str = { version = "1.0", default-features = false }
```

This ensures your library works in `no_std` environments and only enables features you need.

#### Breaking Changes

**None** - This is a stability commitment release, not a breaking change release.

#### Acknowledgments

Thank you to all contributors and users who have helped shape `const-str` into a stable, reliable library. Special thanks to issue reporters and documentation contributors.

---

## [0.7.1] - Prior Release

This was the last pre-1.0 release. For changes in v0.7.1 and earlier, see git history.

### Summary of 0.x Development

The 0.x series established the core functionality:
- Compile-time string operations
- Macro-based API design
- Feature flag architecture
- `no_std` support
- Comprehensive test coverage

**MSRV History:**
- v0.7.0: Rust 1.77.0
- v0.6.0: Rust 1.77.0
- v0.5.7: Rust 1.65.0
- v0.5.0: Rust 1.64.0
- v0.4.0: Rust 1.61.0

---

## Release Process

Releases follow these steps:

1. **Version Bump**: Update version in all `Cargo.toml` files
2. **Update Changelog**: Document all changes
3. **Run Tests**: `just dev` (format, lint, test, miri)
4. **Tag Release**: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
5. **Publish**: `cargo publish -p const-str-proc-macro && cargo publish -p const-str`
6. **GitHub Release**: Create release with changelog excerpt

---

## Versioning Policy

- **MAJOR** (X.0.0): Breaking changes to public API
- **MINOR** (0.X.0): New features, MSRV updates, backward-compatible changes  
- **PATCH** (0.0.X): Bug fixes, documentation, internal improvements

See [STABILITY_POLICY.md](./STABILITY_POLICY.md) for detailed versioning guarantees.

---

[Unreleased]: https://github.com/Nugine/const-str/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/Nugine/const-str/releases/tag/v1.0.0
[0.7.1]: https://github.com/Nugine/const-str/releases/tag/v0.7.1
