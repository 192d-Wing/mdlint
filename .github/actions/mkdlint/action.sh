#!/usr/bin/env bash
# mkdlint GitHub Action - Setup and Execution Script
set -euo pipefail

# Color output helpers
if [ "${INPUT_NO_COLOR:-false}" = "true" ] || [ ! -t 1 ]; then
    RED=""
    GREEN=""
    YELLOW=""
    BLUE=""
    RESET=""
else
    RED="\033[0;31m"
    GREEN="\033[0;32m"
    YELLOW="\033[0;33m"
    BLUE="\033[0;34m"
    RESET="\033[0m"
fi

# Logging helpers
log_info() {
    echo -e "${BLUE}ℹ${RESET} $*"
}

log_success() {
    echo -e "${GREEN}✓${RESET} $*"
}

log_warning() {
    echo -e "${YELLOW}⚠${RESET} $*"
}

log_error() {
    echo -e "${RED}✗${RESET} $*" >&2
}

# Resolve version (convert "latest" to actual version)
resolve_version() {
    local version="${INPUT_VERSION:-latest}"

    if [ "$version" = "latest" ]; then
        log_info "Resolving latest version..."

        # Try GitHub API
        if command -v curl >/dev/null 2>&1; then
            local latest_version
            latest_version=$(curl -sSL \
                -H "Accept: application/vnd.github+json" \
                -H "Authorization: Bearer ${GITHUB_TOKEN:-}" \
                "https://api.github.com/repos/192d-Wing/mkdlint/releases/latest" \
                | grep -o '"tag_name": *"[^"]*"' \
                | sed 's/"tag_name": *"\(.*\)"/\1/' \
                || echo "")

            if [ -n "$latest_version" ]; then
                version="$latest_version"
                log_success "Latest version: $version"
            else
                log_warning "Could not fetch latest version from GitHub API, using v0.3.2"
                version="v0.3.2"
            fi
        else
            log_warning "curl not available, using v0.3.2"
            version="v0.3.2"
        fi
    fi

    # Ensure version has "v" prefix
    if [[ ! "$version" =~ ^v ]]; then
        version="v${version}"
    fi

    echo "$version"
}

# Download and extract binary
download_binary() {
    local version="$1"
    local platform="$2"
    local temp_dir="${TEMP_DIR}/mkdlint-bin"

    # Check if already cached
    if [ "${CACHE_HIT:-false}" = "true" ] && [ -f "${temp_dir}/mkdlint" ]; then
        log_success "Using cached binary"
        echo "${temp_dir}/mkdlint"
        return 0
    fi

    log_info "Downloading mkdlint ${version} for ${platform}..."

    # Create temp directory
    mkdir -p "$temp_dir"

    # Determine archive name and extension
    local archive_name="mkdlint-${platform}"
    local archive_ext
    local binary_name="mkdlint"

    if [[ "$platform" == windows-* ]]; then
        archive_ext="zip"
        binary_name="mkdlint.exe"
    else
        archive_ext="tar.gz"
    fi

    local download_url="https://github.com/192d-Wing/mkdlint/releases/download/${version}/${archive_name}.${archive_ext}"

    # Download archive
    local archive_path="${temp_dir}/${archive_name}.${archive_ext}"

    if command -v curl >/dev/null 2>&1; then
        if ! curl -sSL -f -o "$archive_path" "$download_url"; then
            log_error "Failed to download from: $download_url"
            return 1
        fi
    elif command -v wget >/dev/null 2>&1; then
        if ! wget -q -O "$archive_path" "$download_url"; then
            log_error "Failed to download from: $download_url"
            return 1
        fi
    else
        log_error "Neither curl nor wget is available"
        return 1
    fi

    log_success "Downloaded archive"

    # Extract archive
    log_info "Extracting binary..."

    if [[ "$archive_ext" == "tar.gz" ]]; then
        if ! tar -xzf "$archive_path" -C "$temp_dir"; then
            log_error "Failed to extract tar.gz archive"
            return 1
        fi
    elif [[ "$archive_ext" == "zip" ]]; then
        if ! unzip -q -o "$archive_path" -d "$temp_dir"; then
            log_error "Failed to extract zip archive"
            return 1
        fi
    fi

    # Clean up archive
    rm -f "$archive_path"

    # Make binary executable
    chmod +x "${temp_dir}/${binary_name}"

    # Verify binary
    if ! "${temp_dir}/${binary_name}" --version >/dev/null 2>&1; then
        log_error "Binary verification failed"
        return 1
    fi

    log_success "Binary ready: ${temp_dir}/${binary_name}"

    # For Windows, return with .exe extension
    if [[ "$platform" == windows-* ]]; then
        echo "${temp_dir}/mkdlint.exe"
    else
        echo "${temp_dir}/mkdlint"
    fi
}

# Build from source using cargo
build_from_source() {
    local version="$1"

    log_info "Building mkdlint from source..."

    # Check for cargo
    if ! command -v cargo >/dev/null 2>&1; then
        log_error "cargo not found - cannot build from source"
        log_error "Please install Rust from https://rustup.rs/"
        return 1
    fi

    # Install via cargo
    local version_arg="${version#v}" # Remove 'v' prefix for cargo

    if ! cargo install mkdlint --version "$version_arg" --quiet; then
        log_error "Failed to build mkdlint ${version} from source"
        return 1
    fi

    local binary_path
    binary_path="$(command -v mkdlint)"

    if [ -z "$binary_path" ]; then
        log_error "mkdlint binary not found after cargo install"
        return 1
    fi

    log_success "Built from source: $binary_path"
    echo "$binary_path"
}

# Setup mkdlint (main setup function)
setup_mkdlint() {
    local version
    version=$(resolve_version)

    local binary_path=""

    # Try binary download first if requested
    if [ "${INPUT_USE_BINARY:-true}" = "true" ]; then
        if binary_path=$(download_binary "$version" "$PLATFORM"); then
            log_success "Using pre-built binary"
        else
            log_warning "Binary download failed, falling back to cargo build"
            binary_path=""
        fi
    fi

    # Fall back to cargo build if binary download failed or not requested
    if [ -z "$binary_path" ]; then
        if ! binary_path=$(build_from_source "$version"); then
            log_error "Failed to setup mkdlint"
            exit 1
        fi
    fi

    # Verify final binary
    local mkdlint_version
    mkdlint_version=$("$binary_path" --version 2>&1 || echo "unknown")
    log_success "mkdlint ready: $mkdlint_version"

    # Output binary path
    echo "binary-path=$binary_path" >> "$GITHUB_OUTPUT"
}

# Run mkdlint with configured options
run_mkdlint() {
    local binary_path="${BINARY_PATH}"

    if [ ! -f "$binary_path" ]; then
        log_error "mkdlint binary not found at: $binary_path"
        exit 1
    fi

    # Build command array
    local -a cmd=("$binary_path")

    # Add files/directories
    local files="${INPUT_FILES:-.}"
    for file in $files; do
        cmd+=("$file")
    done

    # Add config file
    if [ -n "${INPUT_CONFIG:-}" ]; then
        cmd+=("--config" "$INPUT_CONFIG")
    fi

    # Add output format
    local output_format="${INPUT_OUTPUT_FORMAT:-sarif}"
    cmd+=("--output-format" "$output_format")

    # Add SARIF file path
    local sarif_file="${INPUT_SARIF_FILE:-mkdlint.sarif}"
    if [ "$output_format" = "sarif" ]; then
        cmd+=("--output" "$sarif_file")
    fi

    # Add fix flag
    if [ "${INPUT_FIX:-false}" = "true" ]; then
        cmd+=("--fix")
    fi

    # Add ignore patterns
    if [ -n "${INPUT_IGNORE:-}" ]; then
        for pattern in $INPUT_IGNORE; do
            cmd+=("--ignore" "$pattern")
        done
    fi

    # Add enable rules
    if [ -n "${INPUT_ENABLE:-}" ]; then
        for rule in $INPUT_ENABLE; do
            cmd+=("--enable" "$rule")
        done
    fi

    # Add disable rules
    if [ -n "${INPUT_DISABLE:-}" ]; then
        for rule in $INPUT_DISABLE; do
            cmd+=("--disable" "$rule")
        done
    fi

    # Add color flag
    if [ "${INPUT_NO_COLOR:-false}" = "true" ]; then
        cmd+=("--no-color")
    fi

    # Add verbose flag
    if [ "${INPUT_VERBOSE:-false}" = "true" ]; then
        cmd+=("--verbose")
    fi

    # Add quiet flag
    if [ "${INPUT_QUIET:-false}" = "true" ]; then
        cmd+=("--quiet")
    fi

    # Add custom arguments
    if [ -n "${INPUT_CUSTOM_ARGS:-}" ]; then
        # shellcheck disable=SC2206
        cmd+=($INPUT_CUSTOM_ARGS)
    fi

    # Log command (for debugging)
    if [ "${INPUT_VERBOSE:-false}" = "true" ]; then
        log_info "Running: ${cmd[*]}"
    fi

    # Run mkdlint and capture exit code
    local exit_code=0
    "${cmd[@]}" || exit_code=$?

    # Parse results
    local error_count=0
    local file_count=0

    if [ -f "$sarif_file" ] && [ "$output_format" = "sarif" ]; then
        # Parse SARIF output if jq is available
        if command -v jq >/dev/null 2>&1; then
            error_count=$(jq '[.runs[].results[]] | length' "$sarif_file" 2>/dev/null || echo "0")
            file_count=$(jq '[.runs[].results[].locations[].physicalLocation.artifactLocation.uri] | unique | length' "$sarif_file" 2>/dev/null || echo "0")
        else
            log_warning "jq not available - cannot parse SARIF results"
        fi

        log_info "SARIF results written to: $sarif_file"
    fi

    # Set outputs
    echo "exit-code=$exit_code" >> "$GITHUB_OUTPUT"
    echo "error-count=$error_count" >> "$GITHUB_OUTPUT"
    echo "file-count=$file_count" >> "$GITHUB_OUTPUT"

    if [ "$output_format" = "sarif" ]; then
        echo "sarif-file=$sarif_file" >> "$GITHUB_OUTPUT"
    else
        echo "sarif-file=" >> "$GITHUB_OUTPUT"
    fi

    # Generate step summary
    {
        echo "## mkdlint Results"
        echo ""
        if [ "$exit_code" = "0" ] && [ "$error_count" = "0" ]; then
            echo "✅ **No issues found**"
        else
            echo "❌ **Found $error_count error(s) in $file_count file(s)**"
        fi
        echo ""
        echo "- Exit Code: \`$exit_code\`"
        echo "- Errors: \`$error_count\`"
        echo "- Files: \`$file_count\`"
        echo "- Format: \`$output_format\`"

        if [ "$output_format" = "sarif" ] && [ "${INPUT_UPLOAD_SARIF:-true}" = "true" ]; then
            echo "- SARIF Upload: ✅ Enabled"
        fi
    } >> "$GITHUB_STEP_SUMMARY"

    # Return original exit code (but don't fail here - let the action.yml check step handle it)
    return 0
}

# Main execution dispatcher
main() {
    local action="${1:-}"

    case "$action" in
        setup)
            setup_mkdlint
            ;;
        run)
            run_mkdlint
            ;;
        *)
            log_error "Unknown action: $action"
            log_error "Usage: $0 {setup|run}"
            exit 1
            ;;
    esac
}

# Only run main if this script is executed directly (not sourced)
if [ "${BASH_SOURCE[0]}" = "${0}" ]; then
    main "$@"
fi
