# Stability Policy

**Version:** 1.0  
**Effective Date:** v1.0.0 Release  
**Last Updated:** 2025-12-14

---

## Overview

This document defines the stability guarantees and policies for the `const-str` crate starting from version 1.0.0. We are committed to providing a stable, reliable library while maintaining the flexibility to evolve and improve.

---

## Semantic Versioning

The `const-str` crate follows [Semantic Versioning 2.0.0](https://semver.org/):

- **MAJOR** (X.0.0): Breaking changes to public API
- **MINOR** (0.X.0): New features, MSRV updates, backward-compatible changes
- **PATCH** (0.0.X): Bug fixes, documentation updates, internal improvements

---

## Stability Guarantees (v1.0.0+)

### What is Stable ✅

#### 1. Public Macros
All documented macros are stable and will not be removed or have breaking signature changes:

- Core string operations: `concat!`, `join!`, `repeat!`, `replace!`, `split!`, etc.
- Comparison & search: `equal!`, `compare!`, `contains!`, `starts_with!`, etc.
- Conversion & encoding: `to_str!`, `to_byte_array!`, `parse!`, `hex!`, etc.
- Format & case: `format!`, `convert_case!`, `convert_ascii_case!`, `cstr!`
- Advanced features: `sorted!`, `ip_addr!`, `concat_bytes!`, feature-gated macros

**Guarantee:** These macros will maintain backward compatibility. New parameters may be added as optional, but existing usage patterns will continue to work.

#### 2. Public Functions
All `pub const fn` functions in the following modules are stable:

- `ascii` - ASCII utilities
- `bytes` - Byte slice operations  
- `str` - String operations
- `slice` - Slice utilities
- `utf8` - UTF-8 encoding
- `utf16` - UTF-16 encoding
- `printable` - Printable character detection

**Guarantee:** Function signatures and behavior will not change in backward-incompatible ways.

#### 3. Feature Flags
All documented feature flags are stable:

- `std` - Standard library support
- `proc` - Procedural macro support
- `regex` - Regex validation
- `http` - HTTP header validation
- `case` - Case conversion
- `all` - Enable all features

**Guarantee:** Existing features will not be removed. New features may be added. Feature behavior will remain backward compatible.

#### 4. Core Behavior
- Compile-time evaluation semantics
- Const-context vs const-fn distinction
- Error behavior (panics for invalid inputs)
- UTF-8 validation and safety

**Guarantee:** These fundamental behaviors will not change.

### What is NOT Stable ⚠️

#### 1. Hidden Modules
Modules marked with `#[doc(hidden)]` are internal implementation details:

- `__ctfe` - Compile-time function evaluation internals
- `__proc` - Procedural macro internals

**Warning:** Do not depend on these modules. They may change without notice.

#### 2. Unstable Features
Features marked `unstable` in Cargo.toml:

- `unstable` - Experimental Rust features

**Warning:** These features may change or be removed at any time.

#### 3. Internal Types
Public types that are only visible for macro implementation:

- Implementation structs in `__ctfe`
- Helper types not documented in public API

**Warning:** These exist for technical reasons but are not part of the public API contract.

---

## MSRV Policy

### Current MSRV: Rust 1.77.0

#### Update Policy
- **MINOR version**: MSRV may be increased
- **PATCH version**: MSRV will NOT be increased
- **Advance notice**: MSRV increases will be announced in release notes

#### Rationale
Const evaluation features in Rust are rapidly evolving. We may need to increase MSRV to:
- Leverage new const-fn capabilities
- Improve compile-time performance
- Add new functionality

#### Commitment
- We will always document MSRV changes clearly
- We will maintain MSRV history in documentation
- We will consider user impact before MSRV increases

---

## Deprecation Policy

### How We Deprecate

When we need to phase out functionality:

1. **Announce in Release Notes**: Clearly state what is deprecated and why
2. **Use `#[deprecated]` Attribute**: Provide compiler warnings with guidance
3. **Maintain for 3 Minor Versions**: Keep deprecated items for at least 3 minor releases
4. **Provide Migration Path**: Always offer alternatives and migration guide
5. **Remove in Next Major**: Only remove in the next major version

### Example Timeline

```
v1.0.0 - Feature X is stable
v1.2.0 - Feature Y is added (better alternative to X)
v1.3.0 - Feature X is deprecated
         #[deprecated(since = "1.3.0", note = "Use feature Y instead")]
v1.4.0 - Feature X still available but deprecated
v1.5.0 - Feature X still available but deprecated
v2.0.0 - Feature X may be removed
```

### Current Status
**No deprecations planned for v1.0.0**

---

## Breaking Change Policy

### When We Make Breaking Changes

Breaking changes will ONLY occur in major version updates (2.0.0, 3.0.0, etc.).

#### What Constitutes a Breaking Change

- Removing a public macro
- Changing macro signatures in incompatible ways
- Removing a public function
- Changing function signatures
- Changing const evaluation behavior
- Removing a feature flag
- Changing error conditions (panic vs success)

#### What is NOT a Breaking Change

- Adding new macros
- Adding new optional parameters to macros
- Adding new functions
- Adding new feature flags
- Improving error messages
- Internal implementation changes
- Documentation updates
- Performance improvements

### Major Version Cadence

We aim to:
- Minimize major version releases (target: 1-2 years between majors)
- Batch breaking changes together
- Provide comprehensive migration guides
- Maintain previous major version with critical bug fixes for 6 months

---

## Feature Addition Policy

### Adding New Features

New features can be added in MINOR versions if they:

1. Are backward compatible
2. Don't break existing code
3. Are properly feature-gated if they add dependencies
4. Include tests and documentation

### Feature Flag Guidelines

New features should be:
- **Optional**: Behind feature flags if they add dependencies
- **Documented**: Clear documentation of what they enable
- **Tested**: Comprehensive test coverage
- **Composable**: Work well with existing features

---

## API Evolution Guidelines

### What We Encourage

- **Additive changes**: New macros, functions, and features
- **Improvements**: Better error messages, performance, documentation
- **Extensions**: New parameters (if optional and backward compatible)

### What We Avoid

- **Removing functionality**: Deprecate first, remove only in major versions
- **Breaking changes**: Only in major versions with clear migration path
- **Silent behavior changes**: Changes that could break user code unexpectedly

---

## Commitment to Users

### We Promise To

1. ✅ **Maintain backward compatibility** within major versions
2. ✅ **Follow semantic versioning** strictly
3. ✅ **Document all changes** in release notes
4. ✅ **Provide migration guides** for breaking changes
5. ✅ **Be transparent** about stability and deprecations
6. ✅ **Test thoroughly** before releases
7. ✅ **Respond to issues** and security concerns promptly

### We Ask Users To

1. 📌 **Specify version constraints** appropriately in Cargo.toml
2. 📖 **Read release notes** when upgrading
3. 🐛 **Report issues** on GitHub
4. 💡 **Share feedback** on API design and features
5. ⚠️ **Avoid depending on** hidden modules and unstable features

---

## Version Specification Guidelines

### Recommended Cargo.toml Specifications

#### For Applications
```toml
# Allow all compatible updates (recommended)
const-str = "1.0"

# Or be more conservative
const-str = "1.0.0"
```

#### For Libraries
```toml
# Allow minor and patch updates (recommended)
const-str = { version = "1.0", default-features = false }

# With specific features
const-str = { version = "1.0", features = ["proc", "regex"] }

# Or specify exact version if needed
const-str = "=1.0.0"
```

---

## Security Policy

### Security Updates

Security fixes will be:
- Released as PATCH versions
- Backported to previous MINOR version if actively used
- Announced through GitHub Security Advisories
- Documented in CHANGELOG

### Reporting Security Issues

Please report security vulnerabilities through GitHub Security Advisories or by contacting the maintainer directly.

---

## Documentation Standards

### What We Document

- ✅ All public macros with examples
- ✅ All public functions with behavior description
- ✅ Feature requirements and effects
- ✅ MSRV and compatibility information
- ✅ Migration guides for breaking changes
- ✅ Troubleshooting common issues

---

## Release Process

### Before Each Release

1. ✅ Run full test suite (`just dev`)
2. ✅ Update documentation
3. ✅ Update CHANGELOG.md
4. ✅ Review for breaking changes
5. ✅ Test with different feature combinations
6. ✅ Verify MSRV compliance

### Release Communication

- GitHub Release with detailed notes
- Crates.io publication
- Documentation update on docs.rs
- Announcement in relevant channels

---

## Questions and Feedback

For questions about this policy or stability concerns:

- Open a GitHub Issue
- Start a GitHub Discussion
- Contact the maintainer

We value community feedback and will refine this policy as needed.

---

**Policy Version:** 1.0  
**Applies to:** const-str v1.0.0 and later  
**Review Schedule:** Annually or as needed  
**Last Reviewed:** 2025-12-14
