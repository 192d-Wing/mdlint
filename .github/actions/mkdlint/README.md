# mkdlint GitHub Action

Fast Markdown linter with auto-fix support and native GitHub Code Scanning integration.

## Features

- **Fast Binary Downloads**: Pre-built binaries for Linux, macOS, and Windows (x86_64 and aarch64)
- **Automatic Caching**: Caches downloaded binaries for 10-100x faster subsequent runs
- **SARIF Integration**: Native GitHub Code Scanning support with automatic SARIF upload
- **Auto-Fix Support**: Apply fixes automatically with the `fix: true` option
- **Flexible Configuration**: Support for `.markdownlint.json`, `.markdownlint.yaml`, and other config formats
- **Cargo Fallback**: Automatically builds from source if binary download fails
- **Comprehensive Options**: Full control over linting rules, output formats, and behavior

## Quick Start

### Basic Usage

Lint all Markdown files in your repository and upload results to Code Scanning:

```yaml
name: Lint Markdown

on: [push, pull_request]

jobs:
  lint:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      security-events: write  # Required for SARIF upload

    steps:
      - uses: actions/checkout@v4

      - uses: 192d-Wing/mkdlint/.github/actions/mkdlint@main
        with:
          files: '.'
```

### Auto-Fix and Commit

Automatically fix issues and commit the changes:

```yaml
name: Auto-fix Markdown

on: [push]

jobs:
  fix:
    runs-on: ubuntu-latest
    permissions:
      contents: write

    steps:
      - uses: actions/checkout@v4

      - uses: 192d-Wing/mkdlint/.github/actions/mkdlint@main
        with:
          files: '.'
          fix: true
          fail-on-error: false

      - uses: stefanzweifel/git-auto-commit-action@v4
        with:
          commit_message: 'fix(docs): auto-fix markdown issues'
```

### Custom Configuration

Use a custom configuration file and selective rules:

```yaml
- uses: 192d-Wing/mkdlint/.github/actions/mkdlint@main
  with:
    config: '.markdownlint.json'
    disable: 'MD013 MD033'  # Disable line length and HTML rules
    ignore: '**/node_modules/** **/vendor/**'
```

## Inputs

### Core Options

| Input | Description | Default |
|-------|-------------|---------|
| `files` | Files or directories to lint (space-separated) | `.` |
| `version` | mkdlint version to use (e.g., `v0.3.2` or `latest`) | `latest` |
| `use-binary` | Use pre-built binary (`true`) or build from source (`false`) | `true` |
| `working-directory` | Working directory for running mkdlint | `.` |

### Configuration

| Input | Description | Default |
|-------|-------------|---------|
| `config` | Path to configuration file | _(auto-discover)_ |
| `enable` | Rules to enable (space-separated, e.g., `MD001 MD002`) | _(all enabled rules)_ |
| `disable` | Rules to disable (space-separated, e.g., `MD013 MD033`) | _(none)_ |
| `ignore` | Glob patterns to ignore (space-separated) | _(none)_ |

### Output Options

| Input | Description | Default |
|-------|-------------|---------|
| `output-format` | Output format: `text`, `json`, or `sarif` | `sarif` |
| `sarif-file` | Path to SARIF output file | `mkdlint.sarif` |
| `upload-sarif` | Upload SARIF results to Code Scanning | `true` |
| `no-color` | Disable colored output | `false` |
| `verbose` | Enable verbose output | `false` |
| `quiet` | Suppress output except errors | `false` |

### Behavior

| Input | Description | Default |
|-------|-------------|---------|
| `fix` | Apply auto-fixes to files | `false` |
| `fail-on-error` | Fail workflow if linting errors are found | `true` |
| `cache-binary` | Cache downloaded binary for faster runs | `true` |
| `custom-args` | Additional arguments to pass to mkdlint | _(none)_ |

### Authentication

| Input | Description | Default |
|-------|-------------|---------|
| `github-token` | GitHub token for SARIF upload | `${{ github.token }}` |

## Outputs

| Output | Description |
|--------|-------------|
| `exit-code` | Exit code from mkdlint execution |
| `error-count` | Number of linting errors found |
| `file-count` | Number of files with errors |
| `sarif-file` | Path to generated SARIF file |
| `binary-path` | Path to the mkdlint binary used |
| `cache-hit` | Whether the binary was retrieved from cache |

## Examples

### Lint Only Changed Files

Optimize performance by linting only files changed in the PR:

```yaml
- uses: tj-actions/changed-files@v41
  id: changed
  with:
    files: '**/*.md'

- uses: 192d-Wing/mkdlint/.github/actions/mkdlint@main
  if: steps.changed.outputs.any_changed == 'true'
  with:
    files: ${{ steps.changed.outputs.all_changed_files }}
```

### Multiple Platforms

Test across different platforms:

```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest, windows-latest]

runs-on: ${{ matrix.os }}

steps:
  - uses: actions/checkout@v4

  - uses: 192d-Wing/mkdlint/.github/actions/mkdlint@main
    with:
      files: 'docs/'
```

### JSON Output

Get structured JSON output for custom processing:

```yaml
- uses: 192d-Wing/mkdlint/.github/actions/mkdlint@main
  id: lint
  with:
    output-format: json
    upload-sarif: false

- name: Process results
  run: |
    echo "Errors found: ${{ steps.lint.outputs.error-count }}"
    echo "Files affected: ${{ steps.lint.outputs.file-count }}"
```

### Disable Specific Rules

Disable rules that don't fit your workflow:

```yaml
- uses: 192d-Wing/mkdlint/.github/actions/mkdlint@main
  with:
    disable: 'MD013 MD033 MD041'
    # MD013: Line length
    # MD033: Inline HTML
    # MD041: First line in file should be top-level heading
```

### Build from Source

Force building from source (useful for testing or unsupported platforms):

```yaml
- uses: 192d-Wing/mkdlint/.github/actions/mkdlint@main
  with:
    use-binary: false
    version: 'v0.3.2'
```

### Custom Configuration Path

Use a configuration file in a non-standard location:

```yaml
- uses: 192d-Wing/mkdlint/.github/actions/mkdlint@main
  with:
    config: 'config/linting/.markdownlint.yaml'
    files: 'docs/**/*.md'
```

### Ignore Patterns

Exclude specific directories or files:

```yaml
- uses: 192d-Wing/mkdlint/.github/actions/mkdlint@main
  with:
    ignore: '**/node_modules/** **/vendor/** **/CHANGELOG.md'
```

### Verbose Output

Enable detailed logging for troubleshooting:

```yaml
- uses: 192d-Wing/mkdlint/.github/actions/mkdlint@main
  with:
    verbose: true
    no-color: false
```

## Required Permissions

For SARIF upload to Code Scanning, your workflow needs:

```yaml
permissions:
  contents: read
  security-events: write
```

For auto-fix with commit:

```yaml
permissions:
  contents: write
```

## Performance Notes

### Binary Caching

The action automatically caches downloaded binaries. Typical performance:

- **First run**: 5-10 seconds (download + extract + cache)
- **Cached runs**: 1-2 seconds (cache hit)
- **Cargo build**: 60-90 seconds (fallback only)

### Incremental Linting

For best performance on large repositories, combine with changed file detection:

```yaml
- uses: tj-actions/changed-files@v41
  id: changed
  with:
    files: '**/*.md'

- uses: 192d-Wing/mkdlint/.github/actions/mkdlint@main
  if: steps.changed.outputs.any_changed == 'true'
  with:
    files: ${{ steps.changed.outputs.all_changed_files }}
```

This can reduce linting time from minutes to seconds on large repositories.

## Troubleshooting

### Binary Download Fails

If binary download fails, the action automatically falls back to building from source using cargo. To force cargo build:

```yaml
- uses: 192d-Wing/mkdlint/.github/actions/mkdlint@main
  with:
    use-binary: false
```

### SARIF Upload Fails

Ensure your workflow has the required permissions:

```yaml
permissions:
  contents: read
  security-events: write
```

To disable SARIF upload:

```yaml
- uses: 192d-Wing/mkdlint/.github/actions/mkdlint@main
  with:
    upload-sarif: false
```

### No Config File Found

mkdlint auto-discovers configuration files in this order:
1. `.markdownlint.jsonc`
2. `.markdownlint.json`
3. `.markdownlint.yaml`
4. `.markdownlint.yml`
5. `.markdownlintrc`

To specify a custom config:

```yaml
- uses: 192d-Wing/mkdlint/.github/actions/mkdlint@main
  with:
    config: 'path/to/config.json'
```

### Workflow Fails Unexpectedly

Enable verbose output and check the logs:

```yaml
- uses: 192d-Wing/mkdlint/.github/actions/mkdlint@main
  with:
    verbose: true
```

Set `fail-on-error: false` to prevent failing the workflow:

```yaml
- uses: 192d-Wing/mkdlint/.github/actions/mkdlint@main
  with:
    fail-on-error: false
```

## Supported Platforms

| Platform | Architecture | Status |
|----------|--------------|--------|
| Linux | x86_64 | ✅ Supported |
| Linux | aarch64 | ✅ Supported |
| macOS | x86_64 | ✅ Supported |
| macOS | aarch64 (Apple Silicon) | ✅ Supported |
| Windows | x86_64 | ✅ Supported |

All platforms fall back to cargo build if binary download fails.

## License

Apache-2.0

## Related Links

- [mkdlint Repository](https://github.com/192d-Wing/mkdlint)
- [mkdlint Documentation](https://github.com/192d-Wing/mkdlint#readme)
- [GitHub Code Scanning](https://docs.github.com/en/code-security/code-scanning)
- [SARIF Format](https://sarifweb.azurewebsites.net/)
