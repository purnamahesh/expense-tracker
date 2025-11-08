# **Version Control & CI/CD**

[← Back to Main README](README.md) | [Hooks & Scripts](HOOKS-README.md) | [Roadmap](ROADMAP.md)

---

This document covers GitHub workflows, CI/CD pipelines, version management, testing, and documentation generation for the Expense Tracker project.

---

## **Table of Contents**

- [Testing](#testing)
  - [GitHub Actions Workflow](#github-actions-workflow)
  - [Temporarily Disable Tests](#temporarily-disable-tests)
- [Pull Request Requirements](#pull-request-requirements)
- [Version Management](#version-management)
  - [GitHub Actions Release](#1-github-actions)
  - [Pre-Release Workflow](#2-pre-release-workflow)
  - [Reusable Version Bump Workflow](#reusable-version-bump-workflow)
  - [Understanding [skip ci]](#understanding-skip-ci-in-workflows)
- [Documentation](#documentation)
  - [GitHub Actions Workflow](#github-actions-workflow-1)
  - [Setup GitHub Pages](#setup-github-pages)
  - [Documentation Structure](#documentation-structure)
- [GitHub Actions Workflows Summary](#github-actions-workflows-summary)
- [Repository Status](#repository-status-summary)

---

## **Testing**

This project includes comprehensive testing infrastructure for unit tests, integration tests, and code coverage.

### **GitHub Actions Workflow**

**File:** `.github/workflows/test.yml`

**Triggers:**
- Push to any branch (if Rust files changed)
- Pull requests to main/master

**Features:**
- ✓ **Smart change detection** - Only runs if `.rs` files changed
- ✓ **Multi-OS testing** - Ubuntu, macOS, Windows
- ✓ **Multi-Rust version** - Stable and Beta
- ✓ **Comprehensive checks**:
  - Code formatting (`cargo fmt`)
  - Linting (`cargo clippy`)
  - Unit tests
  - Integration tests
  - Doc tests
  - All features tests
- ✓ **Code coverage** - Generates coverage report with cargo-llvm-cov
- ✓ **Dependency caching** - Faster CI runs
- ✓ **Required status check** - Blocks PR merge if tests fail
- ✓ **PR comments** - Posts detailed test results as PR comments
- ✓ **Test summary** - Clear pass/fail status

**PR Comments:**

The workflow automatically posts test results to PRs:

**Success:**
```markdown
## 🧪 Test Results

✅ **All Tests Passed**

| Check | Status |
|-------|--------|
| Code Formatting | ✅ Passed |
| Clippy Lints | ✅ Passed |
| Unit Tests | ✅ Passed |
| Integration Tests | ✅ Passed |
| Doc Tests | ✅ Passed |
| Code Coverage | ✅ Generated |

**Tested on:** Ubuntu (stable, beta), macOS (stable), Windows (stable)
```

**Failure:**
```markdown
## 🧪 Test Results

❌ **Tests Failed**

Some tests did not pass. Please check the workflow run for details.

**Common fixes:**
- Run `cargo fmt` to fix formatting issues
- Run `cargo clippy --fix` to fix linting issues
- Run `cargo test` locally to reproduce failures
```

**Branch Protection:**

To require tests and PR title checks to pass before merging:
1. Go to repository Settings → Branches
2. Add branch protection rule for `main`/`master`
3. Enable "Require status checks to pass before merging"
4. Select these status checks:
   - `test-summary` - All tests must pass
   - `validate-title` - PR title must follow conventional commits
5. Enable "Require branches to be up to date before merging" (recommended)

---

### **Temporarily Disable Tests**

Multiple ways to skip tests when needed:

#### **1. Commit Message Override** (Per-commit)
Add `[skip tests]` or `[no tests]` to your commit message:
```bash
git commit -m "docs: update README [skip tests]"
git push
```
Tests will be skipped for that specific commit.

#### **2. Workflow Dispatch** (Manual trigger)
Manually run the workflow with custom options:
1. Go to Actions → Test workflow
2. Click "Run workflow"
3. Select options:
   - ✅ **Skip all tests** - Emergency override (bypasses all tests)
   - ✅ **Run code coverage** - Enable/disable coverage generation

**Use cases:**
- Emergency deployments
- CI maintenance/debugging
- Skip expensive coverage generation for faster feedback

#### **3. Repository Variable** (Global disable)
Disable tests for all pushes/PRs (emergency use):
1. Go to repository Settings → Secrets and variables → Actions → Variables
2. Click "New repository variable"
3. Name: `DISABLE_TESTS`
4. Value: `true`
5. Click "Add variable"

Tests will be disabled globally until you:
- Delete the variable, OR
- Change value to `false`

**⚠️ Warning:** This bypasses all tests including PR checks. Use only in emergencies.

**Matrix Strategy:**
| OS | Rust Stable | Rust Beta |
|----|-------------|-----------|
| Ubuntu | ✅ | ✅ |
| macOS | ✅ | ⏭️ |
| Windows | ✅ | ⏭️ |

---

## **Pull Request Requirements**

### **PR Title Format**

**File:** `.github/workflows/pr-title-check.yml`

All PRs must have titles that follow [Conventional Commits](https://www.conventionalcommits.org/) format. This enables automatic version bumping when merged.

**Required Format:**
```
<type>: <description>
```

**Valid Prefixes:**

| Prefix | Description | Version Bump | Example |
|--------|-------------|--------------|---------|
| `feat:` | New feature | MINOR (0.1.0 → 0.2.0) | `feat: add user authentication` |
| `feat!:` | Breaking change feature | MAJOR (0.1.0 → 1.0.0) | `feat!: redesign API` |
| `fix:` | Bug fix | PATCH (0.1.0 → 0.1.1) | `fix: resolve login crash` |
| `docs:` | Documentation | PATCH (0.1.0 → 0.1.1) | `docs: update README` |
| `chore:` | Maintenance | PATCH (0.1.0 → 0.1.1) | `chore: update dependencies` |
| `patch:` | Small patch | PATCH (0.1.0 → 0.1.1) | `patch: fix typo` |

**Examples:**
```
✅ feat: add expense export feature
✅ fix: correct date calculation
✅ docs: add API documentation
✅ chore: update Rust to 1.70
✅ feat!: change expense data structure (breaking)

❌ Add export feature (missing prefix)
❌ Feature: add export (wrong prefix)
❌ feat add export (missing colon)
```

**How the Check Works:**
- Runs on PR opened, edited, reopened, or synchronized
- **Blocks merge** if title is invalid
- Posts helpful comment with examples if validation fails
- Shows which version bump will be triggered
- Automatically updates comments when title is fixed

**Fix Invalid Title:**
```bash
# Via GitHub CLI
gh pr edit 123 --title "feat: add user authentication"

# Or edit directly in GitHub UI (top of PR page)
```

**Relationship to Version Bumping:**
When a PR is merged to `main`/`master`, the release workflow analyzes the PR title (from the merge commit message) to determine the version bump type. This is why the correct format is required!

---

## **Version Management**

This project includes automated version bumping using semantic versioning (semver).

### **Available Tools:**

#### 1. **GitHub Actions**

**File:** `.github/workflows/release.yml`

Automatically bumps version and publishes to crates.io when pushing to `main` or `master` branch.

**How it works:**
- **Check for Changes:**
  - Skips release if no `.rs` files changed since last tag
  - Displays skip message in workflow logs
- **Job 1 (Check Changes & Determine Bump Type):**
  - Analyzes commit messages since the last git tag
  - Determines version bump type based on Conventional Commits
- **Job 2 (Version Bump):**
  - Uses the reusable `bump-version.yml` workflow
  - Creates a new branch with version changes
  - Creates a PR with title: `chore: bump version X.X.X -> Y.Y.Y`
  - Automatically approves the PR
  - Automatically merges the PR
- **Job 3 (Publish):**
  - Waits for the version bump PR to be merged
  - Creates a git tag for the new version
  - Automatically publishes the new version to crates.io

**Setup:**
To enable automatic publishing to crates.io, add `CARGO_REGISTRY_TOKEN` to your GitHub repository secrets:
1. Get your token from https://crates.io/settings/tokens
2. Go to repository Settings → Secrets and variables → Actions
3. Add new secret: `CARGO_REGISTRY_TOKEN`

**Conventional Commit Format:**
- `feat!:` or `BREAKING CHANGE:` → **MAJOR** bump (0.1.0 → 1.0.0)
- `feat:` → **MINOR** bump (0.1.0 → 0.2.0)
- `fix:` → **PATCH** bump (0.1.0 → 0.1.1)
- Other commits → **PATCH** bump (default)

**Examples:**
```bash
git commit -m "feat: add expense filtering by date range"
git push origin main
# → Version bumped from 0.1.0 to 0.2.0

git commit -m "fix: correct total calculation for negative amounts"
git push origin main
# → Version bumped from 0.2.0 to 0.2.1

git commit -m "feat!: redesign expense data structure"
git push origin main
# → Version bumped from 0.2.1 to 1.0.0
```

#### 2. **Pre-Release Workflow**

**File:** `.github/workflows/pre-release.yml`

Publish alpha/beta/rc versions directly from pull requests using PR comments.

**How it works:**
- Triggered by commenting on a PR with `/pre-release alpha`, `/pre-release beta`, or `/pre-release rc`
- **Job 1 (Check for Changes):**
  - Skips pre-release if no `.rs` files changed in the PR
  - Adds a skip message comment to the PR
- **Job 2 (Bump and Publish):**
  - Bumps version with pre-release suffix directly on the PR branch
  - Commits the version bump to the PR branch
  - Creates a git tag for the new pre-release version
  - Publishes to crates.io
- **Job 3 (Comment and React):**
  - Adds a success comment to the PR with version info
  - Reacts with 👍 on success or 👎 on failure

**Important:** This workflow does NOT merge to main/master. It only:
- Commits version changes to your PR branch
- Publishes a pre-release version to crates.io
- Allows you to test the pre-release before merging the PR

**Usage:**
1. Open a pull request
2. Add a comment: `/pre-release alpha`, `/pre-release beta`, or `/pre-release rc`
3. Workflow automatically:
   - Bumps version with suffix (e.g., `0.1.0` → `0.1.1-alpha.0` or `0.1.1-alpha.0` → `0.1.1-alpha.1`)
   - Commits changes to your PR branch
   - Creates a git tag for the new version
   - Publishes to crates.io
   - Reports back in PR comments

**Examples:**
```bash
# In a PR comment:
/pre-release alpha    # Creates version like 0.1.1-alpha.0, 0.1.1-alpha.1, etc.
/pre-release beta     # Creates version like 0.1.1-beta.0, 0.1.1-beta.1, etc.
/pre-release rc       # Creates version like 0.1.1-rc.0, 0.1.1-rc.1, etc.
/pre-release          # Defaults to alpha
```

**Version Format:**
- `1.0.0` → `/pre-release alpha` → `1.0.1-alpha.0`
- `1.0.1-alpha.0` → `/pre-release alpha` → `1.0.1-alpha.1`
- `1.0.1-alpha.1` → `/pre-release beta` → `1.0.1-beta.0` (switches type)
- `1.0.1-beta.0` → `/pre-release beta` → `1.0.1-beta.1`

**Requirements:**
- Must be run from a pull request
- Requires `CARGO_REGISTRY_TOKEN` secret
- Comment format: `/pre-release [alpha|beta|rc]`

See [HOOKS-README.md](HOOKS-README.md) for local version management scripts.

---

### **Reusable Version Bump Workflow**

**File:** `.github/workflows/bump-version.yml`

A shared workflow used by both `release.yml` and `pre-release.yml` to handle version bumping consistently.

**Features:**
- ✓ Creates a new branch for version changes
- ✓ Creates a PR with clear title showing the version change
- ✓ Automatically approves the PR
- ✓ Automatically merges the PR
- ✓ Supports both semantic versioning (major/minor/patch) and pre-releases (alpha/beta)
- ✓ Updates both Cargo.toml and Cargo.lock
- ✓ Returns version information and PR number for downstream jobs
- ✓ Includes `[skip ci]` in commit messages to prevent infinite release loops

**Benefits:**
- **DRY Principle**: Version bumping logic is centralized in one place
- **Code Review**: PRs allow for review and CI checks before version bump is applied
- **Audit Trail**: Clear history of version changes through PR records
- **Consistency**: Both release and pre-release workflows use the same logic
- **No Infinite Loops**: Version bump commits skip CI to prevent triggering another release

---

### **Understanding `[skip ci]` in Workflows**

The release workflows use `[skip ci]` to prevent infinite loops:

**Syntax in `.github/workflows/release.yml`:**
```yaml
if: "!contains(github.event.head_commit.message, '[skip ci]')"
```

**How it works:**
- `contains(X, Y)` - Returns `true` if X contains Y
- `!` - Negation operator (NOT)
- **Quoted** to prevent YAML parsing issues with `!`

**What it prevents:**
```
Push → Trigger Release → Bump Version → Commit [skip ci] → Push
                            ↑                                  |
                            └──────────────────────────────────┘
                            (This loop is prevented!)
```

**Example commit that gets skipped:**
```bash
chore: bump version 0.3.0 -> 0.4.0 [skip ci]
```

**Alternative Keywords:**
- `[skip ci]`
- `[ci skip]`
- `[no ci]`
- `[skip actions]`
- `[actions skip]`

All of these tell GitHub Actions to skip workflow runs for that commit.

**Where it's used in this project:**
1. **Version bump commits** - Prevents re-triggering release workflow
2. **Documentation updates** - Can manually add to skip tests
3. **CI configuration changes** - When you don't want to trigger full pipeline

---

## **Documentation**

This project includes automated documentation generation using Rust's built-in `rustdoc` tool.

### **GitHub Actions Workflow**

**File:** `.github/workflows/docs.yml`

**Triggers:**
- ~~Push to main/master~~ (currently disabled - commented out in workflow)
- Pull requests to main/master (for doc validation)
- Manual workflow dispatch

**Features:**
- ✓ **Auto-generates documentation** from code comments
- ✓ **Commits to `docs/` directory** automatically (when enabled)
- ✓ **Publishes to GitHub Pages** from `docs/` folder
- ✓ **Checks broken links** in pull requests
- ✓ **Documents private items** for comprehensive coverage
- ✓ **PR comments** with documentation build status
- ✓ **Includes `[skip ci]`** to prevent infinite loops

> **Note:** Push triggers are currently commented out. The workflow only runs on:
> 1. Pull requests (validation only)
> 2. Manual dispatch

### **Setup GitHub Pages**

1. Go to repository Settings → Pages
2. Source: "Deploy from a branch"
3. Branch: `main` (or `master`) and `/docs` folder
4. Click "Save"
5. Documentation will be available at: `https://[username].github.io/[repo-name]/`

**How it works:**
```
Push to main → Build docs → Copy to docs/ → Commit [skip ci] → Push
                                                                    ↓
                                                          GitHub Pages serves docs/
```

**Pull Request Checks:**

When you open a PR, the workflow automatically:
- Builds documentation to check for errors
- Validates intra-doc links
- Posts a comment with the status

### **Documentation Structure**

```
docs/                        # Committed to repository
├── expense_tracker/          # Main crate documentation
│   ├── index.html           # Crate overview
│   ├── utils/               # Utils module docs
│   │   ├── expense/         # Expense module
│   │   ├── args_parser/     # CLI parser
│   │   └── file_parser/     # File I/O
│   └── config/              # Config module
├── search-index.js          # Search functionality
├── index.html              # Redirect to crate docs
└── .nojekyll               # Tells GitHub Pages not to use Jekyll

target/doc/                  # Local build (not committed)
└── (same structure as docs/)
```

**Viewing Documentation:**
```bash
# Local (after running cargo doc)
open target/doc/expense_tracker/index.html

# Or use cargo
cargo doc --open

# Local (view committed docs)
open docs/index.html

# GitHub Pages (after setup)
# https://[username].github.io/expense-tracker/
```

**Note:** The `docs/` directory will be automatically generated and committed by the workflow when push triggers are enabled. Do not manually edit files in `docs/` as they will be overwritten.

**To Generate Docs Manually:**
```bash
# Generate locally
cargo doc --no-deps --document-private-items --all-features

# Copy to docs/ directory
rm -rf docs
cp -r target/doc docs
touch docs/.nojekyll

# Commit to repository
git add docs/
git commit -m "docs: update documentation [skip ci]"
git push
```

See [HOOKS-README.md](HOOKS-README.md) for the local documentation generation script.

---

## **GitHub Actions Workflows Summary**

This project includes a comprehensive set of GitHub Actions workflows for automation.

### **Command Dispatch**

**File:** `.github/workflows/command-dispatch.yml`

Trigger workflows via slash commands in PR comments.

**Available Commands:**

| Command | Description | Example |
|---------|-------------|---------|
| `/pre-release [TYPE]` | Create pre-release | `/pre-release alpha` |
| `/test` | Run test suite | `/test` |
| `/docs` | Generate documentation | `/docs` |

**Usage Example:**
```bash
# In a PR comment:
/pre-release beta
/test
/docs
```

**Features:**
- ✓ Permission checking (requires write/maintain/admin)
- ✓ Argument parsing (named and unnamed)
- ✓ Real-time feedback with reactions and comments
- ✓ Triggers workflow_dispatch on target workflows

**How it works:**
1. User comments `/command` on a PR
2. Workflow validates user permissions
3. Parses command and arguments
4. Triggers the appropriate workflow via `workflow_dispatch`
5. Comments back with status and link to workflow run

See [.github/COMMAND_DISPATCH_GUIDE.md](.github/COMMAND_DISPATCH_GUIDE.md) for detailed documentation.

---

### **Docker Release**

**File:** `.github/workflows/release-docker.yml`

Reusable workflow for building and pushing Docker images.

**Features:**
- 🐳 Multi-arch builds (linux/amd64, linux/arm64)
- 🏷️ Smart tagging strategy (pre-release vs stable)
- 📦 GitHub Container Registry (ghcr.io) or Docker Hub support
- 🔒 Security scanning with Trivy
- 📋 SBOM and provenance generation
- ⚡ Build caching for faster builds

**Usage:**
```yaml
jobs:
  docker:
    uses: ./.github/workflows/release-docker.yml
    secrets: inherit
    with:
      image-name: expense-tracker
      version: 1.0.0
      platforms: linux/amd64,linux/arm64
      docker-registry: ghcr.io
```

**Tagging Strategy:**
- **Pre-release** (e.g., `1.0.0-alpha.1`, `1.0.0-beta.2`, `1.0.0-rc.3`): Only exact version tag
- **Stable** (e.g., `1.0.0`): Version + major.minor + major + optionally "latest"

**Example Tags:**
```bash
# Pre-release 1.0.0-alpha.1:
ghcr.io/username/expense-tracker:1.0.0-alpha.1

# Stable 1.2.3:
ghcr.io/username/expense-tracker:1.2.3
ghcr.io/username/expense-tracker:1.2
ghcr.io/username/expense-tracker:1
ghcr.io/username/expense-tracker:latest
```

**Required Secrets:**
- `GITHUB_TOKEN` (automatic) for ghcr.io
- OR `DOCKER_PASSWORD` for Docker Hub

---

### **All Workflows**

All workflows use modern GitHub Actions and are optimized for Rust projects:

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `command-dispatch.yml` | PR comment | Dispatch commands from PRs |
| `test.yml` | PR, Push | Run tests and coverage |
| `pre-release.yml` | PR comment, Manual | Publish pre-releases |
| `release.yml` | Push to main/master | Auto-release stable versions |
| `release-docker.yml` | Workflow call | Build Docker images |
| `release-github.yml` | Workflow call | Create GitHub releases |
| `docs.yml` | PR, Manual | Generate documentation |
| `pr-title-check.yml` | PR | Validate PR titles |
| `bump-version.yml` | Workflow call | Reusable version bumping |

**Action Versions (All Modern):**
- ✅ `actions/checkout@v4`
- ✅ `actions/cache@v4`
- ✅ `actions/github-script@v7`
- ✅ `docker/build-push-action@v5`
- ✅ `docker/metadata-action@v5`
- ✅ `actions/download-artifact@v5`
- ✅ `softprops/action-gh-release@v2`

---

## **Repository Status Summary**

### **Current Branch:** `feature/clap`

**Recent Changes:**
- ✅ Migrated to `clap` for CLI argument parsing
- ✅ Added `src/cli.rs` with derive-based CLI definitions
- ✅ Removed old `src/utils/args_parser.rs`
- ✅ Updated dependencies: `clap = "4.5.51"`
- ✅ Version bumped to 0.5.2
- ✅ Created documentation generation workflow
- ✅ Created `generate-docs.sh` script
- ✅ Created `docs-template.md` reference guide

**New Files Added:**
```
.github/workflows/docs.yml     # Documentation generation workflow
generate-docs.sh               # Interactive doc generation script
docs-template.md               # Documentation writing guide
src/cli.rs                     # Clap CLI definitions
```

**Files Modified:**
```
Cargo.toml                     # Added clap dependency
src/main.rs                    # Updated to use clap
src/utils/expense.rs          # Updated for clap integration
src/utils/file_parser.rs      # Various improvements
```

**Files Removed:**
```
src/utils/args_parser.rs      # Replaced by src/cli.rs (clap-based)
```

**Dependencies:**
- `chrono = "0.4.42"` - DateTime handling
- `clap = { version = "4.5.51", features = ["derive"] }` - CLI parsing ✨ NEW

**Next Steps:**
1. Test the clap implementation thoroughly
2. Commit changes to `feature/clap` branch
3. Create PR to merge into `master`
4. Consider enabling docs workflow push triggers
5. Plan SQLite migration (future enhancement)

---

## **Best Practices**

1. **Use Conventional Commits** for automatic version detection
2. **Version changes create PRs** instead of direct commits for better traceability
3. **Git tags are created** after PR merge to track version history
4. **`[skip ci]` is automatic** - Version bump workflows add it, you don't need to
5. **Branch protection recommended** - Require PR reviews and status checks
6. **Test locally first** - Use scripts in [HOOKS-README.md](HOOKS-README.md)

---

[← Back to Main README](README.md) | [Hooks & Scripts](HOOKS-README.md) | [Roadmap](ROADMAP.md)
