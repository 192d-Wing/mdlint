# GitHub Action Usage

The mkdlint GitHub Action allows you to automatically lint Markdown files in your repository on every commit, pull request, or scheduled run.

## Quick Start

Create `.github/workflows/markdown-lint.yml` in your repository:

```yaml
name: Markdown Lint

on:
  push:
    branches: [main]
  pull_request:

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: 192d-Wing/mkdlint@v0.10
        with:
          files: '**/*.md'
```

## Inputs

| Input | Description | Required | Default |
|-------|-------------|----------|---------|
| `files` | Files or glob patterns to lint (space-separated) | No | `**/*.md` |
| `config` | Path to configuration file | No | Auto-discovered |
| `fix` | Automatically fix errors where possible | No | `false` |
| `fail-on-error` | Fail the action if errors are found | No | `true` |
| `output-format` | Output format (`default`, `json`, `sarif`, `github`) | No | `github` |
| `version` | mkdlint version to use | No | `latest` |

## Outputs

| Output | Description |
|--------|-------------|
| `errors-found` | Number of errors found |
| `files-checked` | Number of files checked |

## Usage Examples

### Basic Linting

Lint all Markdown files with default configuration:

```yaml
- uses: 192d-Wing/mkdlint@v0.10
```

### Custom File Patterns

Lint only documentation files:

```yaml
- uses: 192d-Wing/mkdlint@v0.10
  with:
    files: 'docs/**/*.md README.md'
```

### With Custom Configuration

Use a custom configuration file:

```yaml
- uses: 192d-Wing/mkdlint@v0.10
  with:
    config: .markdownlint.json
```

### Auto-fix Errors

Automatically fix errors and commit changes:

```yaml
- uses: 192d-Wing/mkdlint@v0.10
  with:
    fix: true

- name: Commit fixes
  if: always()
  run: |
    git config --local user.email "action@github.com"
    git config --local user.name "GitHub Action"
    git diff --quiet || git commit -am "docs: auto-fix markdown issues"
    git push
```

### Continue on Error

Check for errors but don't fail the build:

```yaml
- uses: 192d-Wing/mkdlint@v0.10
  with:
    fail-on-error: false
```

### SARIF Upload (Security Tab)

Upload results to GitHub Security tab:

```yaml
- uses: 192d-Wing/mkdlint@v0.10
  with:
    output-format: sarif
  continue-on-error: true

- name: Upload SARIF
  uses: github/codeql-action/upload-sarif@v2
  with:
    sarif_file: mkdlint.sarif
```

### Specific Version

Pin to a specific version:

```yaml
- uses: 192d-Wing/mkdlint@v0.10
  with:
    version: '0.10.2'
```

### Use Outputs

Use the action outputs in subsequent steps:

```yaml
- uses: 192d-Wing/mkdlint@v0.10
  id: lint

- name: Comment on PR
  if: failure() && github.event_name == 'pull_request'
  uses: actions/github-script@v6
  with:
    script: |
      github.rest.issues.createComment({
        issue_number: context.issue.number,
        owner: context.repo.owner,
        repo: context.repo.repo,
        body: `❌ Markdown linting found ${{ steps.lint.outputs.errors-found }} errors in ${{ steps.lint.outputs.files-checked }} files.`
      })
```

### Multiple Jobs

Run linting separately from other checks:

```yaml
name: CI

on: [push, pull_request]

jobs:
  markdown-lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: 192d-Wing/mkdlint@v0.10

  tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run tests
        run: npm test

  deploy:
    needs: [markdown-lint, tests]
    runs-on: ubuntu-latest
    steps:
      - name: Deploy
        run: echo "Deploying..."
```

### Scheduled Linting

Run linting on a schedule:

```yaml
name: Weekly Markdown Lint

on:
  schedule:
    - cron: '0 0 * * 0'  # Every Sunday at midnight

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: 192d-Wing/mkdlint@v0.10
```

### Matrix Strategy

Test across multiple platforms:

```yaml
jobs:
  lint:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: 192d-Wing/mkdlint@v0.10
```

### Changed Files Only

Lint only changed files in a PR:

```yaml
- name: Get changed files
  id: changed-files
  uses: tj-actions/changed-files@v40
  with:
    files: |
      **.md

- uses: 192d-Wing/mkdlint@v0.10
  if: steps.changed-files.outputs.any_changed == 'true'
  with:
    files: ${{ steps.changed-files.outputs.all_changed_files }}
```

## Configuration File

Place a `.markdownlint.json` file in your repository root:

```json
{
  "default": true,
  "MD013": false,
  "MD033": {
    "allowed_elements": ["br", "img"]
  }
}
```

See [USER_GUIDE.md](USER_GUIDE.md) for full configuration options.

## Troubleshooting

### Action fails with "command not found"

The action downloads and installs mkdlint automatically. Ensure:
1. You're using a supported runner (ubuntu-latest, macos-latest, windows-latest)
2. The runner has internet access to download releases
3. The runner has `sudo` permissions (for Linux/macOS)

### No files found

Check your glob pattern matches files:

```yaml
- name: Debug - List files
  run: ls -la **/*.md

- uses: 192d-Wing/mkdlint@v0.10
  with:
    files: '**/*.md'
```

### Config file not found

Ensure the config path is relative to repository root:

```yaml
# Wrong
config: '${{ github.workspace }}/.markdownlint.json'

# Correct
config: '.markdownlint.json'
```

### Annotations not showing

Ensure you're using the `github` output format (default):

```yaml
- uses: 192d-Wing/mkdlint@v0.10
  with:
    output-format: github  # This is the default
```

## Performance

The action is highly optimized:
- **Small repos** (~10 files): ~5 seconds total (download + lint)
- **Medium repos** (~100 files): ~10 seconds total
- **Large repos** (~1000 files): ~30 seconds total

Downloads are cached by GitHub Actions, so subsequent runs are faster.

## Contributing

Found an issue with the action? Please report it at:
https://github.com/192d-Wing/mkdlint/issues
