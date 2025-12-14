# Release Checklist for v1.0.0

This checklist guides the process of releasing const-str v1.0.0.

## Pre-Release Review

- [x] API review completed (see API_REVIEW_V1.0.0.md)
- [x] Stability policy documented (see STABILITY_POLICY.md)
- [x] Changelog prepared (see CHANGELOG.md)
- [x] All tests passing (158 tests)
- [x] No security vulnerabilities
- [x] Documentation complete

## Version Updates

When ready to release, update version numbers:

### 1. Update Cargo.toml Files

#### crates/const-str-proc-macro/Cargo.toml
```toml
[package]
version = "1.0.0"  # Update from "0.7.1"
```

#### crates/const-str/Cargo.toml
```toml
[package]
version = "1.0.0"  # Update from "0.7.1"

[dependencies.const-str-proc-macro]
version = "1.0.0"  # Update from "0.7.1"
```

### 2. Update justfile

```just
sync-version:
    cargo set-version -p const-str-proc-macro   '1.0.0'
    cargo set-version -p const-str              '1.0.0'
```

### 3. Update CHANGELOG.md

Change the release date:
```markdown
## [1.0.0] - YYYY-MM-DD  # Replace "Unreleased" with actual date
```

## Pre-Release Testing

- [ ] Run `just fmt --check`
- [ ] Run `just lint -- -D warnings`
- [ ] Run `just test`
- [ ] Run `just unstable-test` (if nightly available)
- [ ] Run `just miri` (if nightly available)
- [ ] Test with minimal features: `cargo test --no-default-features`
- [ ] Test with all features: `cargo test --all-features`
- [ ] Build documentation: `cargo doc --no-deps --all-features`
- [ ] Verify documentation looks correct

## Git Operations

- [ ] Commit version updates: `git commit -m "Release v1.0.0"`
- [ ] Create git tag: `git tag -a v1.0.0 -m "Release v1.0.0"`
- [ ] Push commits: `git push`
- [ ] Push tag: `git push origin v1.0.0`

## Crates.io Publication

### Order of Publication (Important!)

1. **First**: Publish proc-macro crate
   ```bash
   cd crates/const-str-proc-macro
   cargo publish --dry-run  # Verify first
   cargo publish
   ```

2. **Wait**: Allow crates.io to process (usually a few minutes)

3. **Second**: Publish main crate
   ```bash
   cd crates/const-str
   cargo publish --dry-run  # Verify first
   cargo publish
   ```

## GitHub Release

- [ ] Go to https://github.com/Nugine/const-str/releases/new
- [ ] Select tag: v1.0.0
- [ ] Release title: "v1.0.0 - Stable Release"
- [ ] Description: Use excerpt from CHANGELOG.md
- [ ] Attach stability documents (optional):
  - API_REVIEW_V1.0.0.md
  - STABILITY_POLICY.md
- [ ] Mark as "Latest release"
- [ ] Publish release

## Post-Release

### Documentation
- [ ] Verify docs.rs built correctly: https://docs.rs/const-str/1.0.0
- [ ] Check crates.io page: https://crates.io/crates/const-str

### Communication
- [ ] Update README.md badges if needed
- [ ] Announce on relevant forums/channels (optional):
  - Reddit: r/rust
  - Twitter/X
  - This Week in Rust (submit PR)
  - Rust Users Forum

### Repository Maintenance
- [ ] Close related issues
- [ ] Update any related projects or dependencies
- [ ] Monitor issues for any v1.0.0-related problems

## Rollback Plan (If Needed)

If critical issues are discovered after release:

1. **Yank the release** (only if absolutely necessary):
   ```bash
   cargo yank --vers 1.0.0 const-str
   cargo yank --vers 1.0.0 const-str-proc-macro
   ```

2. **Fix the issue** in a new release (1.0.1)

3. **Document** in CHANGELOG.md what went wrong

## Success Criteria

v1.0.0 is successfully released when:

- [x] Both crates published to crates.io
- [x] Git tag created and pushed
- [x] GitHub release created
- [x] Documentation built on docs.rs
- [ ] No immediate critical issues reported
- [ ] Release announcement published (if planned)

## Notes

### Why This Order Matters

1. **Proc-macro first**: The main crate depends on the proc-macro crate, so the proc-macro must be published first
2. **Wait between publications**: Crates.io needs time to index the proc-macro crate before the main crate can depend on it
3. **Dry-run first**: Always test with `--dry-run` to catch issues before publishing
4. **No un-publish**: Once published, a version cannot be unpublished, only yanked

### Version Constraints

After v1.0.0, follow semantic versioning:
- **Patch** (1.0.x): Bug fixes only
- **Minor** (1.x.0): New features, MSRV updates, backward compatible
- **Major** (x.0.0): Breaking changes only

### MSRV Updates

Current MSRV: Rust 1.77.0
- Can be updated in **minor** versions (1.x.0)
- Document in CHANGELOG.md
- Announce prominently in release notes

## Resources

- [Cargo Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [API Review](./API_REVIEW_V1.0.0.md)
- [Stability Policy](./STABILITY_POLICY.md)

## Contact

For questions about the release process:
- Open an issue on GitHub
- Contact the maintainer: @Nugine

---

**Last Updated:** 2025-12-14  
**For Release:** v1.0.0  
**Status:** Ready for release when maintainer decides
