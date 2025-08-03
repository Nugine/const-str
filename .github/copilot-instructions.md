# Copilot Instructions for const-str

## Project Overview

`const-str` is a Rust library that provides compile-time string operations. It's designed to perform string manipulations at compile time using const generics and const evaluation.

### Repository Structure

- **Workspace**: This is a Rust workspace with two main crates:
  - `const-str/`: The main library crate
  - `const-str-proc-macro/`: Procedural macro support crate
- **Build Tool**: Uses `justfile` for development commands
- **MSRV**: Minimum Supported Rust Version is 1.77.0
- **Nightly Features**: Some features require nightly Rust (miri, documentation)

## Development Workflow

### Essential Commands

Use `just` command runner for development tasks:

```bash
# Development cycle
just dev              # Format, lint, test, unstable test, and miri

# Individual tasks
just fmt              # Format code with cargo fmt
just lint             # Run clippy linter
just test             # Run standard tests with multiple configurations
just unstable-test    # Run tests with all features
just miri             # Run miri tests (requires nightly)
just ci               # Run CI checks locally
```

### Testing Strategy

The project has comprehensive testing:
- Standard unit tests
- Doc tests (42 doc tests in the main crate)
- Feature-gated tests (`--features all`)
- Release mode tests
- Miri tests for unsafe code validation
- Multiple Rust version testing (MSRV 1.77.0 and stable)

### Code Style and Guidelines

1. **Const Evaluation**: Focus on compile-time string operations
2. **Safety**: Use miri for unsafe code validation
3. **Documentation**: Extensive doc tests - maintain them when adding new functions
4. **Features**: Support both minimal and full feature sets
5. **Performance**: Consider both compile-time and runtime performance

## Important Files

- `justfile`: Contains all development commands
- `Cargo.toml`: Workspace configuration
- `.github/workflows/ci.yml`: CI pipeline with multiple Rust versions
- `const-str/src/lib.rs`: Main library entry point
- Individual modules in `const-str/src/` for different string operations

## Dependencies and Tools

### Required Tools
- Rust toolchain (stable and nightly)
- `just` command runner
- `cargo-audit` for security auditing

### Key Dependencies
- `proc-macro2`, `quote`, `syn`: For procedural macros
- Various optional dependencies for extended functionality

## When Making Changes

1. **Always run `just dev`** before submitting changes
2. **Test with both stable and nightly** if using advanced features
3. **Update documentation** if adding new const functions
4. **Consider MSRV compatibility** when adding new language features
5. **Add doc tests** for new public functions
6. **Run miri tests** if working with unsafe code

## Copilot-Specific Notes

- This crate heavily uses const generics and const evaluation
- Pay attention to const fn requirements and limitations
- When suggesting code changes, ensure they work at compile time
- Be aware of Rust edition differences and MSRV constraints
- Test suggestions should include both regular tests and doc tests
- Consider feature flags when suggesting new functionality