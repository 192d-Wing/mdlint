# Release Checklist

This document outlines the steps to release a new version of mkdlint.

## Pre-Release Checklist

- [ ] All tests pass locally (`cargo test --all-features`)
- [ ] Clippy passes with no warnings (`cargo clippy --all-targets --all-features -- -D warnings`)
- [ ] Documentation builds successfully (`cargo doc --no-deps`)
- [ ] Benchmarks run without errors (`cargo bench`)
- [ ] README.md is up to date with current features
- [ ] CHANGELOG.md is updated with all changes since last release
- [ ] Version number is updated in `Cargo.toml`
- [ ] Git status is clean (all changes committed)

## Version Numbering

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (x.0.0): Breaking API changes
- **MINOR** (0.x.0): New features, backwards compatible
- **PATCH** (0.0.x): Bug fixes, backwards compatible

## Release Process

### 1. Update Version

```bash
# Edit Cargo.toml to bump version
# Edit CHANGELOG.md to set release date
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to X.Y.Z"
```

### 2. Create Git Tag

```bash
# Create annotated tag with release notes
git tag -a vX.Y.Z -m "Release X.Y.Z"
```

### 3. Push to GitHub

```bash
# Push commits and tags
git push origin main
git push origin vX.Y.Z
```

### 4. Automated Release

GitHub Actions will automatically:

1. **Create GitHub Release**
   - Extract changelog for this version
   - Create release with changelog as description
   - Upload binary artifacts for 5 platforms

2. **Publish to crates.io**
   - Verify version matches tag
   - Run full test suite
   - Run clippy checks
   - Publish to crates.io

### 5. Verify Release

- [ ] GitHub release created: https://github.com/192d-Wing/mkdlint/releases
- [ ] Binary artifacts uploaded (5 platforms)
- [ ] crates.io updated: https://crates.io/crates/mkdlint
- [ ] Documentation updated: https://docs.rs/mkdlint
- [ ] CI badges showing passing status

## Post-Release

- [ ] Test installation: `cargo install mkdlint`
- [ ] Verify downloaded binary works
- [ ] Announce release (if applicable):
  - GitHub Discussions
  - Reddit r/rust
  - Twitter/X
  - This Week in Rust

## Hotfix Process

For critical bugs requiring immediate patch release:

1. Create hotfix branch from release tag:
   ```bash
   git checkout -b hotfix/X.Y.Z+1 vX.Y.Z
   ```

2. Apply fix and test thoroughly

3. Update version to patch increment (X.Y.Z+1)

4. Follow normal release process

## Rollback

If a release has critical issues:

1. **Yank the crates.io version** (doesn't delete, prevents new downloads):
   ```bash
   cargo yank --vers X.Y.Z
   ```

2. Delete the GitHub release (if needed)

3. Create hotfix release with fix

## Release Schedule

- **Patch releases**: As needed for bug fixes
- **Minor releases**: Monthly (first Monday of the month)
- **Major releases**: As needed for breaking changes

## Current Version

**v0.2.0** - Scheduled for 2026-02-15

### Highlights

- 14 new auto-fix rules (total 24+ fixable)
- Rich error display with source context
- GitHub Actions CI/CD pipeline
- Code coverage tracking
- Security auditing
- Multi-platform binary releases
- Performance improvements (19-22% faster)
