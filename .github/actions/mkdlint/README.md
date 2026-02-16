# mkdlint GitHub Action

Fast Markdown linting with auto-fix and SARIF Code Scanning support.

## Features

- ⚡ **Fast binary caching** - 10-100x faster than building from source
- 🔧 **Auto-fix** - Automatically fix 44/54 rules (81.5% coverage)
- 📊 **SARIF Support** - Native GitHub Code Scanning integration
- 🎯 **Zero config** - Works out of the box
- 🔄 **Flexible** - Lint, fix, or just report
- 📝 **Rich output** - Text, JSON, or SARIF formats

## Quick Start

```yaml
- uses: 192d-Wing/mkdlint/.github/actions/mkdlint@v0.8.0
  with:
    files: '.'
```

That's it! This will:
1. Lint all Markdown files in your repository
2. Upload results to GitHub Code Scanning
3. Fail the workflow if errors are found

## Usage Examples

### Basic Linting

```yaml
name: Lint Markdown
on: [push, pull_request]

jobs:
  markdown:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: 192d-Wing/mkdlint/.github/actions/mkdlint@v0.8.0
        with:
          files: '.'
```

### Auto-Fix and Commit

```yaml
name: Auto-Fix Markdown
on: [push]

jobs:
  fix:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - uses: 192d-Wing/mkdlint/.github/actions/mkdlint@v0.8.0
        with:
          files: '.'
          fix: true
          fail-on-error: false
      
      - uses: stefanzweifel/git-auto-commit-action@v5
        with:
          commit_message: 'docs: auto-fix markdown issues'
```

### Custom Configuration

```yaml
- uses: 192d-Wing/mkdlint/.github/actions/mkdlint@v0.8.0
  with:
    files: 'docs/ README.md'
    config: '.markdownlint.json'
    ignore: '**/node_modules/**,vendor/**'
    disable: 'MD013,MD033'
```

### Multiple Output Formats

```yaml
- uses: 192d-Wing/mkdlint/.github/actions/mkdlint@v0.8.0
  with:
    files: '.'
    output-format: 'json'
    upload-sarif: false
```

### Specific Version

```yaml
- uses: 192d-Wing/mkdlint/.github/actions/mkdlint@v0.8.0
  with:
    files: '.'
    version: '0.8.0'  # Pin to specific version
```

## Inputs

| Input | Description | Default |
|-------|-------------|---------|
| `files` | Files or directories to lint | `.` |
| `version` | mkdlint version (`latest` or specific like `0.8.0`) | `latest` |
| `use-binary` | Use pre-built binary (much faster) | `true` |
| `config` | Path to configuration file | `` |
| `output-format` | Output format: `text`, `json`, or `sarif` | `sarif` |
| `sarif-file` | Path to SARIF output file | `mkdlint.sarif` |
| `fix` | Auto-fix violations | `false` |
| `ignore` | Comma-separated glob patterns to ignore | `` |
| `enable` | Comma-separated rules to enable | `` |
| `disable` | Comma-separated rules to disable | `` |
| `no-color` | Disable colored output | `false` |
| `verbose` | Verbose output | `false` |
| `quiet` | Quiet mode | `false` |
| `fail-on-error` | Fail if errors found | `true` |
| `upload-sarif` | Upload to Code Scanning | `true` |
| `working-directory` | Working directory | `.` |

## Outputs

| Output | Description |
|--------|-------------|
| `exit-code` | Exit code from mkdlint |
| `error-count` | Number of errors found |
| `file-count` | Number of files with errors |
| `sarif-file` | Path to SARIF file |
| `binary-path` | Path to mkdlint binary |
| `cache-hit` | Whether binary was cached |

## Advanced Examples

### Matrix Testing

```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest, windows-latest]
runs-on: ${{ matrix.os }}
steps:
  - uses: actions/checkout@v4
  - uses: 192d-Wing/mkdlint/.github/actions/mkdlint@v0.8.0
```

### Conditional Execution

```yaml
- uses: 192d-Wing/mkdlint/.github/actions/mkdlint@v0.8.0
  if: github.event_name == 'pull_request'
  with:
    files: ${{ github.event.pull_request.changed_files }}
```

### Use Outputs

```yaml
- id: lint
  uses: 192d-Wing/mkdlint/.github/actions/mkdlint@v0.8.0
  with:
    fail-on-error: false

- name: Comment on PR
  if: steps.lint.outputs.error-count > 0
  run: |
    echo "Found ${{ steps.lint.outputs.error-count }} errors"
```

## Performance

**Binary caching makes this action 10-100x faster than alternatives:**

- With cache hit: ~1-2 seconds
- Without cache (first run): ~5-10 seconds
- Building from source: ~60-90 seconds

Cache is keyed by OS, architecture, and version, so it persists across runs.

## Permissions

Minimal permissions required:

```yaml
permissions:
  contents: read           # To checkout code
  security-events: write   # To upload SARIF (if using Code Scanning)
```

## Troubleshooting

### SARIF upload fails

Ensure you have the correct permissions:

```yaml
permissions:
  security-events: write
```

### Binary download fails

The action automatically falls back to building from source. To force source build:

```yaml
- uses: 192d-Wing/mkdlint/.github/actions/mkdlint@v0.8.0
  with:
    use-binary: false
```

### Errors in node_modules

Use the `ignore` input:

```yaml
with:
  ignore: '**/node_modules/**,**/vendor/**'
```

## Comparison with Alternatives

| Feature | mkdlint | markdownlint-cli | markdownlint-cli2 |
|---------|---------|------------------|-------------------|
| Speed | ⚡ Rust (parallel) | Node.js | Node.js |
| Auto-fix | 81.5% (44/54) | Limited | Limited |
| SARIF | ✅ Native | ❌ | ✅ Via plugin |
| Binary caching | ✅ Yes | ❌ | ❌ |
| Watch mode | ✅ Yes | ❌ | ❌ |
| Config wizard | ✅ Yes | ❌ | ❌ |

## License

Apache-2.0

## Links

- [mkdlint Repository](https://github.com/192d-Wing/mkdlint)
- [Documentation](https://github.com/192d-Wing/mkdlint/blob/main/docs/USER_GUIDE.md)
- [Rules Reference](https://github.com/DavidAnson/markdownlint/tree/main/doc)
- [Report Issues](https://github.com/192d-Wing/mkdlint/issues)
