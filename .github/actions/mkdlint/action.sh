#!/usr/bin/env bash
set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Logging functions
info() {
    echo -e "${CYAN}ℹ${NC} $*"
}

success() {
    echo -e "${GREEN}✓${NC} $*"
}

warn() {
    echo -e "${YELLOW}⚠${NC} $*"
}

error() {
    echo -e "${RED}✗${NC} $*" >&2
}

# Detect platform
detect_platform() {
    local os arch
    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Linux*)
            case "$arch" in
                x86_64) echo "linux-x86_64" ;;
                aarch64|arm64) echo "linux-aarch64" ;;
                *) error "Unsupported architecture: $arch"; exit 1 ;;
            esac
            ;;
        Darwin*)
            case "$arch" in
                x86_64) echo "macos-x86_64" ;;
                arm64) echo "macos-aarch64" ;;
                *) error "Unsupported architecture: $arch"; exit 1 ;;
            esac
            ;;
        MINGW*|MSYS*|CYGWIN*)
            echo "windows-x86_64"
            ;;
        *)
            error "Unsupported OS: $os"
            exit 1
            ;;
    esac
}

# Resolve version
resolve_version() {
    local version="$1"
    
    if [ "$version" = "latest" ]; then
        info "Fetching latest version from GitHub..."
        local latest
        latest=$(curl -sSf https://api.github.com/repos/192d-Wing/mkdlint/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
        if [ -z "$latest" ]; then
            error "Failed to fetch latest version"
            exit 1
        fi
        echo "$latest"
    else
        # Ensure version has 'v' prefix
        if [[ ! "$version" =~ ^v ]]; then
            echo "v$version"
        else
            echo "$version"
        fi
    fi
}

# Download binary
download_binary() {
    local version="$1"
    local platform="$2"
    local cache_dir="$HOME/.mkdlint-bin"
    
    mkdir -p "$cache_dir"
    
    local archive_name
    local binary_name="mkdlint"
    
    if [[ "$platform" == windows-* ]]; then
        archive_name="mkdlint-${platform}.exe.zip"
        binary_name="mkdlint.exe"
    else
        archive_name="mkdlint-${platform}.tar.gz"
    fi
    
    local url="https://github.com/192d-Wing/mkdlint/releases/download/${version}/${archive_name}"
    local download_path="${cache_dir}/${archive_name}"
    
    info "Downloading mkdlint ${version} for ${platform}..."
    if ! curl -sSfL "$url" -o "$download_path"; then
        error "Failed to download from $url"
        return 1
    fi
    
    info "Extracting binary..."
    cd "$cache_dir"
    if [[ "$archive_name" == *.tar.gz ]]; then
        tar -xzf "$archive_name"
    else
        unzip -q -o "$archive_name"
    fi
    
    if [ ! -f "$cache_dir/$binary_name" ]; then
        error "Binary not found after extraction"
        return 1
    fi
    
    chmod +x "$cache_dir/$binary_name"
    
    # Verify binary works
    if "$cache_dir/$binary_name" --version >/dev/null 2>&1; then
        success "Binary downloaded and verified"
        echo "$cache_dir/$binary_name"
        return 0
    else
        error "Binary verification failed"
        return 1
    fi
}

# Build from source
build_from_source() {
    local version="$1"
    
    info "Building mkdlint from source..."
    
    if ! command -v cargo &> /dev/null; then
        error "cargo not found. Please install Rust or use use-binary: true"
        exit 1
    fi
    
    # Install specific version or latest
    if [ "$version" = "latest" ] || [ "$version" = "vlatest" ]; then
        cargo install mkdlint --features cli
    else
        # Remove 'v' prefix for cargo
        local cargo_version="${version#v}"
        cargo install mkdlint --version "$cargo_version" --features cli
    fi
    
    local binary_path
    binary_path="$(command -v mkdlint)"
    
    if [ -z "$binary_path" ]; then
        error "mkdlint not found after installation"
        exit 1
    fi
    
    success "Built from source successfully"
    echo "$binary_path"
}

# Setup mkdlint
cmd_setup() {
    local version="$1"
    local use_binary="$2"
    local cache_hit="${3:-false}"
    
    local resolved_version
    resolved_version=$(resolve_version "$version")
    
    local binary_path
    
    if [ "$use_binary" = "true" ]; then
        # Check if binary already in cache
        local cache_dir="$HOME/.mkdlint-bin"
        local binary_name="mkdlint"
        [[ "$(detect_platform)" == windows-* ]] && binary_name="mkdlint.exe"
        
        if [ "$cache_hit" = "true" ] && [ -f "$cache_dir/$binary_name" ]; then
            success "Using cached binary"
            binary_path="$cache_dir/$binary_name"
        else
            # Download binary
            local platform
            platform=$(detect_platform)
            
            if ! binary_path=$(download_binary "$resolved_version" "$platform"); then
                warn "Binary download failed, falling back to source build"
                binary_path=$(build_from_source "$resolved_version")
            fi
        fi
    else
        # Build from source
        binary_path=$(build_from_source "$resolved_version")
    fi
    
    # Output to GitHub Actions
    echo "binary-path=$binary_path" >> "$GITHUB_OUTPUT"
    
    # Verify and show version
    local actual_version
    actual_version=$("$binary_path" --version 2>&1 | head -n1)
    success "mkdlint ready: $actual_version"
    info "Binary path: $binary_path"
}

# Run mkdlint
cmd_run() {
    local binary_path="$1"
    local files="$2"
    local config="$3"
    local output_format="$4"
    local sarif_file="$5"
    local fix="$6"
    local ignore="$7"
    local enable="$8"
    local disable="$9"
    local no_color="${10}"
    local verbose="${11}"
    local quiet="${12}"
    
    # Build command
    local cmd=("$binary_path")
    
    # Add flags
    [ "$fix" = "true" ] && cmd+=(--fix)
    [ "$no_color" = "true" ] && cmd+=(--no-color)
    [ "$verbose" = "true" ] && cmd+=(--verbose)
    [ "$quiet" = "true" ] && cmd+=(--quiet)
    
    # Add config
    [ -n "$config" ] && cmd+=(--config "$config")
    
    # Add output format
    cmd+=(--output-format "$output_format")
    
    # Add ignore patterns
    if [ -n "$ignore" ]; then
        IFS=',' read -ra PATTERNS <<< "$ignore"
        for pattern in "${PATTERNS[@]}"; do
            cmd+=(--ignore "$pattern")
        done
    fi
    
    # Add enable rules
    if [ -n "$enable" ]; then
        IFS=',' read -ra RULES <<< "$enable"
        for rule in "${RULES[@]}"; do
            cmd+=(--enable "$rule")
        done
    fi
    
    # Add disable rules
    if [ -n "$disable" ]; then
        IFS=',' read -ra RULES <<< "$disable"
        for rule in "${RULES[@]}"; do
            cmd+=(--disable "$rule")
        done
    fi
    
    # Add files
    cmd+=($files)
    
    info "Running: ${cmd[*]}"
    
    # Run and capture output
    local exit_code=0
    local output_file
    output_file=$(mktemp)
    
    if [ "$output_format" = "sarif" ]; then
        "${cmd[@]}" > "$sarif_file" 2>&1 || exit_code=$?
        cp "$sarif_file" "$output_file"
    else
        "${cmd[@]}" > "$output_file" 2>&1 || exit_code=$?
        cat "$output_file"
    fi
    
    # Parse results
    local error_count=0
    local file_count=0
    
    if [ "$output_format" = "sarif" ] && command -v jq &> /dev/null; then
        # Parse SARIF with jq if available
        if [ -f "$sarif_file" ]; then
            error_count=$(jq '[.runs[].results | length] | add // 0' "$sarif_file" 2>/dev/null || echo "0")
            file_count=$(jq '[.runs[].results[].locations[].physicalLocation.artifactLocation.uri] | unique | length' "$sarif_file" 2>/dev/null || echo "0")
        fi
    elif [ "$output_format" = "json" ] && command -v jq &> /dev/null; then
        # Parse JSON
        if [ -f "$output_file" ]; then
            error_count=$(jq '[.[] | length] | add // 0' "$output_file" 2>/dev/null || echo "0")
            file_count=$(jq 'keys | length' "$output_file" 2>/dev/null || echo "0")
        fi
    fi
    
    # Output to GitHub Actions
    echo "exit-code=$exit_code" >> "$GITHUB_OUTPUT"
    echo "error-count=$error_count" >> "$GITHUB_OUTPUT"
    echo "file-count=$file_count" >> "$GITHUB_OUTPUT"
    echo "sarif-file=$sarif_file" >> "$GITHUB_OUTPUT"
    
    # Summary
    if [ "$exit_code" -eq 0 ]; then
        success "No errors found!"
    else
        warn "Found $error_count error(s) in $file_count file(s)"
    fi
    
    rm -f "$output_file"
    return "$exit_code"
}

# Main
main() {
    local command="${1:-}"
    shift || true
    
    case "$command" in
        setup)
            cmd_setup "$@"
            ;;
        run)
            cmd_run "$@"
            ;;
        *)
            error "Unknown command: $command"
            echo "Usage: $0 {setup|run} [args...]"
            exit 1
            ;;
    esac
}

main "$@"
