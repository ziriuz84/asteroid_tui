#!/bin/bash

# release.sh - Automates the asteroid-tui release workflow
# Usage: ./release.sh 1.1.3
# Options: ./release.sh --help

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

success() {
    echo -e "${GREEN}✅ $1${NC}"
}

error() {
    echo -e "${RED}❌ $1${NC}"
    exit 1
}

warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# Banner
print_banner() {
    cat << "EOF"

  ╔═══════════════════════════════════════╗
  ║   asteroid-tui Release Manager 🚀    ║
  ╚═══════════════════════════════════════╝

EOF
}

# Help
show_help() {
    cat << EOF
Usage: ./release.sh [VERSION] [OPTIONS]

Examples:
  ./release.sh 1.1.3               # Release version 1.1.3
  ./release.sh --patch             # Auto-increment patch (1.1.2 → 1.1.3)
  ./release.sh --minor             # Auto-increment minor (1.1.0 → 1.2.0)
  ./release.sh --major             # Auto-increment major (1.0.0 → 2.0.0)
  ./release.sh --dry-run 1.1.3     # Preview changes without committing

Options:
  --help           Show this help message
  --dry-run        Show what would be done without making changes
  --no-tag         Create release but don't push the tag
  --push-only      Only push existing tag (for recovery)

Files modified:
  - Cargo.toml (package version)
  - CHANGELOG.md (new section prepended): built from commits after the tag for the version in Cargo.toml prior to bump (vX.Y.Z..HEAD); feat/fix/docs map to Features/Bug Fixes/etc.; merges, chore: release*, Cargo.toml version bumps ignored.

Requirements:
  - git
  - grep/sed
  - cargo (optional; useful for local validation before release)
  - Current branch: main (or master)
  - Cargo.lock tracked in git (Jenkins uses cargo publish --locked)

EOF
}

# Validate semantic version format
validate_version() {
    local version=$1
    if [[ ! $version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        error "Invalid version format: $version (use X.Y.Z format)"
    fi
}

# Read current version from Cargo.toml
get_current_version() {
    grep -E '^version[[:space:]]*=' Cargo.toml | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1
}

# Increment version (major/minor/patch)
increment_version() {
    local current=$1
    local type=$2
    
    IFS='.' read -r major minor patch <<< "$current"
    
    case $type in
        patch)
            patch=$((patch + 1))
            ;;
        minor)
            minor=$((minor + 1))
            patch=0
            ;;
        major)
            major=$((major + 1))
            minor=0
            patch=0
            ;;
        *)
            error "Unknown increment type: $type"
            ;;
    esac
    
    echo "$major.$minor.$patch"
}

# True when subject looks like a Cargo.toml version bump commit
is_version_bump_commit() {
    local s="$1"
    [[ "$s" =~ Cargo\.toml ]] || [[ "$s" =~ version[[:space:]]*= ]]
}

# Validate repo is ready for release
validate_repo() {
    local dry_run=$1
    info "Validating repository state..."
    
    # Ensure we're in a git repository
    if ! git rev-parse --git-dir > /dev/null 2>&1; then
        error "Not a git repository"
    fi
    
    # Warn if not on default branch
    current_branch=$(git rev-parse --abbrev-ref HEAD)
    if [ "$current_branch" != "main" ] && [ "$current_branch" != "master" ]; then
        if [ "$dry_run" = "true" ]; then
            warning "Current branch is '$current_branch' (expected 'main' or 'master'); continuing dry-run"
        else
            warning "Current branch is '$current_branch' (expected 'main' or 'master')"
            read -p "Continue anyway? (y/n) " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                error "Aborted by user"
            fi
        fi
    fi
    
    # Check working tree (skipped in dry-run so preview works with local changes)
    if ! git diff-index --quiet HEAD --; then
        if [ "$dry_run" = "true" ]; then
            warning "Working directory has uncommitted changes; dry-run will not commit"
        else
            error "Working directory has uncommitted changes. Please commit first."
        fi
    fi
    
    # Check remote
    if ! git remote | grep -q 'origin'; then
        error "No 'origin' remote configured"
    fi
    
    success "Repository validation passed"
}

# Bump version in tracked files
update_version() {
    local new_version=$1
    local dry_run=$2
    
    info "Updating version to $new_version..."
    
    if [ "$dry_run" = "true" ]; then
        sed -E -e 's/^version[[:space:]]*=.*/version = "'"$new_version"'"/' Cargo.toml | head -5
        echo "Would run: cargo update -p <crate> --precise $new_version"
    else
        sed -i -E 's/^version[[:space:]]*=.*/version = "'"$new_version"'"/' Cargo.toml
        local pkg
        pkg=$(grep -E '^name[[:space:]]*=' Cargo.toml | head -1 | sed -E 's/^name[[:space:]]*=[[:space:]]*"([^"]+)".*/\1/')
        cargo update -p "$pkg" --precise "$new_version"
    fi
    
    success "Version updated to $new_version"
}

# GitHub https base URL from origin (https://github.com/owner/repo), or empty
get_github_https_base() {
    local remote
    remote=$(git remote get-url origin 2>/dev/null || echo "")
    if [[ "$remote" =~ github\.com[:/]([^/]+)/([^/.]+)(\.git)?$ ]]; then
        echo "https://github.com/${BASH_REMATCH[1]}/${BASH_REMATCH[2]%.git}"
    else
        echo ""
    fi
}

# Maps conventional commit subjects to changelog sections (added/changed/fixed/...)
classify_commit_subject() {
    local s="$1"
    if [[ "$s" != *:* ]]; then
        local lc
        lc=$(printf '%s\n' "$s" | tr '[:upper:]' '[:lower:]')
        if [[ "$lc" == bump* ]] && is_version_bump_commit "$s"; then
            echo "skip"
        else
            echo "changed"
        fi
        return
    fi

    local prefix="${s%%:*}"
    local body="${s#*:}"
    local body_trim="${body#"${body%%[![:space:]]*}"}"
    local body_lower
    body_lower=$(printf '%s\n' "$body_trim" | tr '[:upper:]' '[:lower:]')

    case "$prefix" in
        Merge*)
            echo "skip"
            return
            ;;
    esac

    local raw_type="${prefix%%(*}"
    local type="$raw_type"
    type="${type%\!}"

    case "$type" in
        feat | Feat | feature | Feature) echo added ;;
        fix | Fix) echo fixed ;;
        docs | Docs) echo documentation ;;
        test | tests | Test | Tests) echo tests ;;
        chore | Chore)
            if [[ "$body_lower" == release* ]] || { [[ "$body_lower" == bump* ]] && is_version_bump_commit "$s"; }; then
                echo skip
            else
                echo chores
            fi
            ;;
        style | Style | refactor | Refactor | perf | Perf | ci | CI | build | Build | revert | Revert) echo changed ;;
        *) echo changed ;;
    esac
}

# Build markdown changelog body from git (v<previous_version>..HEAD unless fallback)
build_changelog_notes() {
    local previous_version=$1
    local github_base=$2
    local log_range=""
    local tmpdir
    tmpdir=$(mktemp -d)

    if git rev-parse "v${previous_version}^{commit}" >/dev/null 2>&1; then
        log_range="v${previous_version}..HEAD"
    elif last=$(git describe --tags --abbrev=0 --match 'v*' 2>/dev/null); then
        warning "Tag v${previous_version} not found; using revision range ${last}..HEAD"
        log_range="${last}..HEAD"
    else
        warning "No v* tag found; falling back to the last 30 commits"
        log_range="-n 30"
    fi

    : >"$tmpdir/added"
    : >"$tmpdir/changed"
    : >"$tmpdir/fixed"
    : >"$tmpdir/documentation"
    : >"$tmpdir/tests"
    : >"$tmpdir/chores"

    local -a log_cmd
    if [[ "$log_range" == "-n 30" ]]; then
        log_cmd=(git log -n 30 --no-merges --pretty=format:'%H|%s')
    else
        log_cmd=(git log "$log_range" --no-merges --pretty=format:'%H|%s')
    fi

    while IFS= read -r line; do
        [[ -z "$line" ]] && continue
        local hash="${line%%|*}"
        local subject="${line#*|}"
        local section
        section=$(classify_commit_subject "$subject")
        [[ "$section" == "skip" ]] && continue
        local short
        short=$(git rev-parse --short "$hash" 2>/dev/null || echo "${hash:0:7}")
        local link_suffix=""
        if [[ -n "$github_base" ]]; then
            link_suffix=" ([${short}](${github_base}/commit/${hash}))"
        fi
        echo "- ${subject}${link_suffix}" >>"$tmpdir/$section"
    done < <("${log_cmd[@]}")

    append_section() {
        local title=$1
        local file=$2
        if [[ -s "$file" ]]; then
            printf '%s\n\n%s\n\n' "### $title" "$(cat "$file")" >>"$tmpdir/outbuf"
        fi
    }

    : >"$tmpdir/outbuf"
    append_section "Features" "$tmpdir/added"
    append_section "Bug Fixes" "$tmpdir/fixed"
    append_section "Code Refactoring" "$tmpdir/changed"
    append_section "Documentation" "$tmpdir/documentation"
    append_section "Tests" "$tmpdir/tests"
    append_section "Chores" "$tmpdir/chores"

    if [[ ! -s "$tmpdir/outbuf" ]]; then
        echo "### Code Refactoring"
        echo ""
        echo "- No commits left after filtering (merge/release/Cargo.toml version bump only). Check tags or edit this entry manually."
        echo ""
        rm -rf "$tmpdir"
    else
        cat "$tmpdir/outbuf"
        rm -rf "$tmpdir"
    fi
}

# Prepend generated release section to CHANGELOG.md
update_changelog() {
    local previous_version=$1
    local new_version=$2
    local dry_run=$3
    local date
    date=$(date +%Y-%m-%d)

    info "Updating CHANGELOG.md..."

    local github_base
    github_base=$(get_github_https_base)
    local heading
    if [[ -n "$github_base" ]]; then
        heading="## [$new_version](${github_base}/releases/tag/v${new_version}) (${date})"
    else
        heading="## [$new_version] (${date})"
    fi

    local body
    body=$(build_changelog_notes "$previous_version" "$github_base")
    body="${body%"${body##*[![:space:]]}"}"$'\n'

    local changelog_entry="${heading}"$'\n\n'"${body}"$'---'$'\n\n'

    if [ "$dry_run" = "true" ]; then
        echo "CHANGELOG.md would be updated with:"
        echo "$changelog_entry"
    else
        temp=$(mktemp)
        {
            echo "$changelog_entry"
            cat CHANGELOG.md
        } >"$temp"
        mv "$temp" CHANGELOG.md
    fi

    success "CHANGELOG.md updated"
}

# Stage and commit version + changelog
create_commit() {
    local version=$1
    local dry_run=$2
    
    info "Creating release commit..."
    
    if [ "$dry_run" = "true" ]; then
        echo "Would run:"
        echo "  git add Cargo.toml Cargo.lock CHANGELOG.md"
        echo "  git commit -m 'chore: release v$version'"
    else
        git add Cargo.toml Cargo.lock CHANGELOG.md
        git commit -m "chore: release v$version"
        success "Commit created"
    fi
}

# Create annotated git tag
create_tag() {
    local version=$1
    local dry_run=$2
    
    info "Creating git tag..."
    
    if [ "$dry_run" = "true" ]; then
        echo "Would run:"
        echo "  git tag -a v$version -m 'Release version $version'"
        echo "  git push origin main v$version"
    else
        # Annotated tag points at release commit (Cargo.toml version)
        git tag -a "v$version" -m "Release version $version"
        success "Tag created: v$version"
    fi
}

# Push
push_changes() {
    local version=$1
    local dry_run=$2
    local no_tag=$3
    
    info "Pushing changes to origin..."
    
    if [ "$dry_run" = "true" ]; then
        echo "Would run:"
        echo "  git push origin main"
        if [ "$no_tag" != "true" ]; then
            echo "  git push origin v$version"
        fi
    else
        git push origin main
        if [ "$no_tag" != "true" ]; then
            git push origin "v$version"
        fi
        success "Changes pushed to origin"
    fi
}

# Main
main() {
    print_banner
    
    local new_version=""
    local dry_run=false
    local no_tag=false
    local push_only=false
    
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --help)
                show_help
                exit 0
                ;;
            --dry-run)
                dry_run=true
                shift
                ;;
            --no-tag)
                no_tag=true
                shift
                ;;
            --push-only)
                push_only=true
                shift
                ;;
            --patch|--minor|--major)
                current_version=$(get_current_version)
                increment_type=${1#--}
                new_version=$(increment_version "$current_version" "$increment_type")
                shift
                ;;
            *)
                new_version=$1
                shift
                ;;
        esac
    done
    
    # Require explicit version (--patch|--minor|--major handled above)
    if [ -z "$new_version" ]; then
        current=$(get_current_version)
        error "No version specified. Current version: $current\nRun with --help for usage"
    fi
    
    # Semantic version sanity check
    validate_version "$new_version"
    
    # Info
    current_version=$(get_current_version)
    info "Current version: $current_version"
    info "New version: $new_version"
    
    if [ "$dry_run" = "true" ]; then
        warning "DRY RUN MODE - no changes will be made"
        if [ ! -f Cargo.lock ] || ! git ls-files --error-unmatch Cargo.lock >/dev/null 2>&1; then
            warning "Cargo.lock is not tracked in git; commit it before a real release (Jenkins uses cargo publish --locked)"
        fi
    fi
    
    # Push-only: skip changelog/version steps
    if [ "$push_only" = "true" ]; then
        info "Push-only mode: skipping validation and updates"
        push_changes "$new_version" "$dry_run" "$no_tag"
        success "Tag pushed"
        return 0
    fi
    
    # Prerequisites
    validate_repo "$dry_run"

    local previous_version
    previous_version=$(get_current_version)

    # Version file, changelog, commit
    update_version "$new_version" "$dry_run"
    update_changelog "$previous_version" "$new_version" "$dry_run"
    create_commit "$new_version" "$dry_run"

    # Tag must attach to the commit that touches Cargo.toml (CI expects this)
    if [ "$dry_run" != "true" ]; then
        if ! git show --pretty="" --name-only HEAD | grep -qx 'Cargo.toml'; then
            error "Release commit missing Cargo.toml — refusing to tag (fix release.sh)"
        fi
        committed_ver=$(
            git show "HEAD:Cargo.toml" | grep -E '^version[[:space:]]*=' |
                grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1
        )
        if [ "$committed_ver" != "$new_version" ]; then
            error "Committed Cargo.toml version ($committed_ver) != release target ($new_version)"
        fi
    fi

    create_tag "$new_version" "$dry_run"
    push_changes "$new_version" "$dry_run" "$no_tag"
    
    # Summary
    echo ""
    if [ "$dry_run" = "true" ]; then
        warning "DRY RUN completed. No changes were made."
        info "Run again without --dry-run to make changes"
    else
        success "Release v$new_version created successfully!"
        echo ""
        echo "Next steps:"
        echo "  1. Jenkins will detect the tag and run tests, build, packaging, GitHub release, and crates.io publish"
        echo "  2. Monitor the Jenkins pipeline until it completes successfully"
        echo ""
        info "Release process initiated! 🚀"
    fi
}

# Entry point
main "$@"
