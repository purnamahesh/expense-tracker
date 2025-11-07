## **Personal Expense Tracker (CLI)**

> **Current Status:** Active development on `feature/clap` branch
> **Version:** 0.5.2
> **Latest Update:** Migrating to `clap` for CLI argument parsing

**Project Goal:** Build a command-line application that helps track personal expenses.

**Core Features:**
- Add a new expense with description, amount, and category (food, transport, entertainment, etc.)
- List all expenses
- Calculate total spending
- Filter expenses by category
- Save/load data to a text file (using simple format like PSV or JSON)

**What You'll Learn:**
- Basic Rust syntax and ownership concepts
- Working with `struct` and `enum` types
- Using `Vec` for storing data
- Pattern matching with `match`
- File I/O operations
- Error handling with `Result`
- ✅ **Parsing command-line arguments using `clap` crate** (implemented with derive macros)

**Example Usage:**
```bash
# Add expense with tags
$ expense-tracker add --amount 12.50 --category food --description "Lunch" --tag meal --tag restaurant

# List all expenses
$ expense-tracker list

# Calculate total spending
$ expense-tracker total

# Filter expenses by category
$ expense-tracker filter --category food

# Filter by amount
$ expense-tracker filter --amount 50.0

# Filter by tags
$ expense-tracker filter --tag food --tag coffee
```

**Current Implementation (v0.5.2):**
- ✅ **Clap-based CLI** - Modern argument parsing with derive macros (`src/cli.rs`)
- ✅ **Subcommands** - `add`, `list`, `total`, `filter`
- ✅ **Type-safe arguments** - Validated by clap at compile time
- ✅ **DateTime tracking** - Automatic timestamp for each expense
- ✅ **Multi-tag support** - Multiple tags per expense
- ✅ **PSV storage** - Pipe-separated values format
- 🚧 **SQLite migration** - Planned future enhancement

**Project Structure:**
```
src/
├── main.rs           # Entry point
├── cli.rs            # Clap CLI definitions (NEW!)
├── config.rs         # Configuration constants
├── utils.rs          # Module declarations
└── utils/
    ├── expense.rs    # Expense struct and logic
    └── file_parser.rs # File I/O operations
```

This project is perfect because it's:
- Practical and useful
- Small enough to complete in a few days
- Covers fundamental Rust concepts
- Can be extended with more features as you learn (like date tracking, budgets, statistics)

---

## **Future Enhancements - Command Reference**

### **Export to Multiple Formats**
```bash
# Export to JSON
expense-tracker export --format json --output expenses.json

# Export to CSV
expense-tracker export --format csv --output expenses.csv

# Export to HTML report
expense-tracker export --format html --output report.html

# Export filtered data
expense-tracker export --format json --category food --output food-expenses.json
```

### **Database Encryption**
```bash
# Initialize encrypted database
expense-tracker init --encrypt --password

# Open with password
expense-tracker --password="my-secret" list

# Change encryption password
expense-tracker encrypt --change-password

# Export decrypted backup
expense-tracker backup --output backup.db --decrypt
```

### **Enhanced UI - Tables & Coloring**
```bash
# Pretty table output with colors
expense-tracker list --style fancy

# Compact table
expense-tracker list --style compact

# ASCII borders only
expense-tracker list --style ascii

# Custom columns
expense-tracker list --columns id,amount,category,date

# Sort options
expense-tracker list --sort-by amount --order desc

# Show totals in table footer
expense-tracker list --show-totals
```

### **Error Handling & Logging**
```bash
# Enable debug logging
expense-tracker --log-level debug add --amount 50 --category food

# Log to file
expense-tracker --log-file ~/.expense-tracker/app.log list

# Verbose error messages
expense-tracker -v add --amount -10 --category food
# Error: Invalid amount: must be positive
#   at src/utils/expense.rs:45
#   Suggestion: Use positive numbers for amounts

# JSON error output (for scripting)
expense-tracker --error-format json add --amount invalid
```

### **Multi-User Support**
```bash
# Create user
expense-tracker user create --username john --email john@example.com

# Login
expense-tracker login --username john

# Switch user
expense-tracker --user alice list

# List all users
expense-tracker user list

# User-specific database
expense-tracker --user john add --amount 25 --category food

# Shared expenses
expense-tracker add --amount 100 --category rent --shared-with alice,bob

# User statistics
expense-tracker stats --user john --period monthly
```


### **Combined Examples**
```bash
# Export encrypted data for specific user with logging
expense-tracker --user john --log-level info \
  export --format json --decrypt --output john-backup.json

# TUI with pretty colors and debug mode
expense-tracker --log-level debug tui --style fancy

# Multi-user report with HTML export
expense-tracker stats --all-users --period yearly \
  export --format html --output annual-report.html

# Encrypted backup with compression
expense-tracker backup --encrypt --compress \
  --output ~/.backups/expenses-$(date +%Y%m%d).db.enc.gz
```

---

## **Getting Started**

### **1. Clone the Repository**
```bash
git clone <repository-url>
cd expense-tracker
```

### **2. Build the Project**
```bash
cargo build --release
```

### **3. Install Git Hooks**
This project uses git hooks to enforce commit message standards and automate version management:
```bash
./install-hooks.sh
```

This installs two hooks:

#### **commit-msg Hook**
Validates commit messages follow [Conventional Commits](https://www.conventionalcommits.org/) format.

**What it does:**
- ✓ Enforces commit message prefix (feat:, fix:, chore:, docs:, etc.)
- ✓ Suggests adding `[skip tests]` suffix if not present
- ✓ Blocks commits with invalid format
- ✓ Shows helpful error messages with examples

#### **pre-merge-commit Hook**
Automatically bumps version when merging into main/master branch.

**What it does:**
- ✓ Detects merge commits to main/master
- ✓ Analyzes merged commits for version bump type
- ✓ Automatically updates Cargo.toml and Cargo.lock
- ✓ Stages version changes with the merge commit
- ✓ Uses conventional commit format to determine bump type:
  - `feat!:`, `BREAKING CHANGE` → **MAJOR** bump (1.0.0 → 2.0.0)
  - `feat:` → **MINOR** bump (1.0.0 → 1.1.0)
  - `fix:`, `patch:` → **PATCH** bump (1.0.0 → 1.0.1)

**Requirement:** Install `cargo-bump` for automatic version bumping:
```bash
cargo install cargo-bump
```

**Valid commit message format:**
```bash
<type>: <description>
```

**Valid prefixes:**
- `feat:` - New feature (MINOR version bump)
- `feat!:` - Breaking change feature (MAJOR version bump)
- `fix:` - Bug fix (PATCH version bump)
- `patch:` - Small patch (PATCH version bump)
- `chore:` - Routine tasks, maintenance
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `test:` - Adding or updating tests
- `build:` - Build system or dependency changes
- `ci:` - CI/CD configuration changes
- `revert:` - Reverting a previous commit

Add `!` before `:` for breaking changes (e.g., `feat!:`, `fix!:`)

**Examples:**
```bash
✅ git commit -m "feat: add user authentication"
✅ git commit -m "fix: correct date parsing [skip tests]"
✅ git commit -m "docs: update README [skip tests]"
✅ git commit -m "feat!: redesign API (breaking change)"

❌ git commit -m "Add new feature"  # Missing prefix
❌ git commit -m "Feature: add auth"  # Wrong prefix
❌ git commit -m "feat add auth"  # Missing colon
```

**Bypassing hooks (not recommended):**
```bash
git commit -m "message" --no-verify
```

**Example Workflow with Hooks:**
```bash
# Create a feature branch
git checkout -b feature/add-export

# Make changes and commit (commit-msg hook validates)
git commit -m "feat: add PSV export functionality"

# Switch to main and merge (pre-merge-commit hook bumps version)
git checkout main
git merge feature/add-export
# → Hook automatically bumps version from 1.0.0 to 1.1.0
# → Cargo.toml and Cargo.lock are updated and staged

# Push changes
git push origin main
```

**Note:** The git hooks are versioned in the `hooks/` directory. If hooks are updated, run `./install-hooks.sh` again to update your local `.git/hooks/`.

---

## **Testing**

This project includes comprehensive testing infrastructure for unit tests, integration tests, and code coverage.

### **GitHub Actions Workflow** (`.github/workflows/test.yml`)

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

**Re-enable:**
1. Go to repository Settings → Secrets and variables → Actions → Variables
2. Find `DISABLE_TESTS`
3. Click "Delete" or change value to `false`

**Matrix Strategy:**
| OS | Rust Stable | Rust Beta |
|----|-------------|-----------|
| Ubuntu | ✅ | ✅ |
| macOS | ✅ | ⏭️ |
| Windows | ✅ | ⏭️ |

### **Local Test Script** (`test.sh`)

Interactive script for running tests locally with various options.

**Usage:**
```bash
./test.sh [OPTIONS]
```

**Options:**
- `--unit` - Run only unit tests
- `--integration` - Run only integration tests
- `--doc` - Run only doc tests
- `--coverage` - Generate code coverage report
- `--watch` - Run tests in watch mode (auto-rerun on changes)
- `--verbose` - Show verbose output
- `--release` - Run tests in release mode
- `--help` - Show help message

**Examples:**
```bash
# Run all tests (default)
./test.sh

# Run only unit tests
./test.sh --unit

# Run tests in watch mode (auto-rerun on file changes)
./test.sh --watch --unit

# Generate coverage report
./test.sh --coverage

# Run integration tests with verbose output
./test.sh --integration --verbose

# Run tests in release mode
./test.sh --release
```

**Features:**
- ✓ Prerequisite checks (cargo, cargo-watch, cargo-llvm-cov)
- ✓ Offers to install missing tools
- ✓ Shows uncommitted Rust file changes
- ✓ Runs formatting check (`cargo fmt`)
- ✓ Runs clippy linter
- ✓ Builds project before tests
- ✓ Color-coded output
- ✓ Clear pass/fail summary

**Example Output:**
```bash
$ ./test.sh --unit
========================================
  Test Suite - Local Execution
========================================

Checking prerequisites...
✓ Cargo.toml found
✓ cargo installed

ℹ Uncommitted Rust file changes detected:
  src/main.rs
  src/utils/expense.rs

Checking code formatting...
✓ Code formatting is correct

Running clippy (linter)...
✓ Clippy checks passed

Building project...
✓ Build successful

Running Unit Tests...
Command: cargo test --lib

✓ Unit Tests passed

========================================
  ✓ All tests passed!
========================================
```

**Coverage Report:**
```bash
# Generate and view coverage
./test.sh --coverage

# Generate HTML report
cargo llvm-cov --all-features --workspace --html
open target/llvm-cov/html/index.html

# Generate lcov.info for external tools (Codecov, Coveralls)
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
```

---

## **Pull Request Requirements**

### **PR Title Format** (`.github/workflows/pr-title-check.yml`)

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

#### 1. **GitHub Actions** (`.github/workflows/release.yml`)
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

#### 2. **Release Script** (`release.sh`)
Interactive script that replicates the full GitHub Actions release workflow locally.

**Usage:**
```bash
./release.sh
```

**Features:**
- ✓ Checks all prerequisites (cargo-bump, git, CARGO_REGISTRY_TOKEN, etc.)
- ✓ Detects Rust file changes since last tag
- ✓ Analyzes commits for automatic version bump type (major/minor/patch)
- ✓ Updates both Cargo.toml and Cargo.lock
- ✓ Interactive y/n prompts for each major action:
  - Version bump
  - Commit and tag
  - Push to remote
  - Publish to crates.io (with dry-run first)
- ✓ Color-coded output for better readability

**Example session:**
```bash
$ ./release.sh
========================================
  Release Script - Local Workflow
========================================

Checking prerequisites...
✓ Cargo.toml found
✓ Git repository
✓ On main branch
✓ No uncommitted changes
✓ cargo-bump installed
✓ CARGO_REGISTRY_TOKEN is set

✓ Rust files changed since v0.1.3:
  src/main.rs
  src/utils/expense.rs

Analyzing commits for version bump type...
Recent commits:
  feat: add new expense filtering
  fix: correct date parsing

Detected: MINOR bump (new feature)

Proceed with minor version bump? (y/N):
```

#### 3. **Pre-Release Script** (`pre-release.sh`)
Interactive script that replicates the GitHub Actions pre-release workflow locally.

**Usage:**
```bash
./pre-release.sh [alpha|beta]
```

**Examples:**
```bash
./pre-release.sh alpha   # Create alpha pre-release (e.g., 1.0.1-alpha.0)
./pre-release.sh beta    # Create beta pre-release (e.g., 1.0.1-beta.0)
```

**Features:**
- ✓ Checks all prerequisites
- ✓ Detects if on feature branch vs base branch
- ✓ Checks for Rust file changes compared to base branch
- ✓ Smart version calculation:
  - Increments if same type (1.0.1-alpha.0 → 1.0.1-alpha.1)
  - Resets if switching type (1.0.1-alpha.2 → 1.0.1-beta.0)
  - Bumps patch if stable (1.0.0 → 1.0.1-alpha.0)
- ✓ Interactive y/n prompts for each action
- ✓ Portable (works on macOS and Linux)

#### 4. **Manual Version Bump Script** (`bump-version.sh`)
Simple interactive script for quick version bumping.

**Usage:**
```bash
./bump-version.sh <major|minor|patch>
```

**Examples:**
```bash
./bump-version.sh patch   # 0.1.0 → 0.1.1 (bug fixes)
./bump-version.sh minor   # 0.1.0 → 0.2.0 (new features)
./bump-version.sh major   # 0.1.0 → 1.0.0 (breaking changes)
```

**Note:** Bump type argument is **required** (no longer defaults to patch).

**Features:**
- Validates bump type
- Shows current and new version
- Updates Cargo.lock
- Optionally commits and tags the version
- Provides helpful instructions for pushing
- Shows command to publish to crates.io

#### 5. **Pre-Release Workflow** (`.github/workflows/pre-release.yml`)
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

### **Requirements:**

Install `cargo-bump` for all methods:
```bash
cargo install cargo-bump
```

### **Reusable Version Bump Workflow** (`.github/workflows/bump-version.yml`)

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

### **Best Practices:**

1. **Use Conventional Commits** for automatic version detection
2. **Version changes create PRs** instead of direct commits for better traceability
3. **Git tags are created** after PR merge to track version history
4. **`[skip ci]` is automatic** - Version bump workflows add it, you don't need to
5. Choose the method that fits your workflow:
   - **GitHub Actions**: Best for team projects and CI/CD (creates PRs for review)
   - **Manual Script**: Best for controlled releases (direct local control)

---

## **Documentation**

This project includes automated documentation generation using Rust's built-in `rustdoc` tool.

### **GitHub Actions Workflow** (`.github/workflows/docs.yml`)

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

**Setup GitHub Pages:**
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

### **Local Documentation Script** (`generate-docs.sh`)

Interactive script for generating documentation locally.

**Usage:**
```bash
./generate-docs.sh
```

**Options:**
1. **Generate documentation (HTML)** - Basic doc generation
2. **Generate and open in browser** - Build and open automatically
3. **Generate with private items** - Include private functions/structs
4. **Check for broken doc links** - Validate all documentation links
5. **Run doc tests** - Execute code examples in documentation
6. **Generate JSON documentation** - Machine-readable format (requires nightly)
7. **Clean documentation** - Remove generated docs
8. **Exit**

**Quick Commands:**
```bash
# Generate and open docs
cargo doc --open

# Generate with private items
cargo doc --document-private-items

# Check for broken links
cargo rustdoc -- -D warnings

# Run doc tests
cargo test --doc

# Clean docs
cargo clean --doc
```

### **Documentation Template** (`docs-template.md`)

Reference guide for writing documentation comments in Rust.

**Example Module Documentation:**
```rust
//! Module for expense tracking.
//!
//! This module provides functionality for managing expenses
//! including creation, filtering, and persistence.

/// Represents a single expense entry.
///
/// # Fields
///
/// * `amount` - The expense amount (must be positive)
/// * `category` - Expense category (e.g., "food", "transport")
///
/// # Examples
///
/// ```
/// use expense_tracker::Expense;
///
/// let expense = Expense::new(
///     50.0,
///     Some("Lunch".to_string()),
///     "food".to_string(),
///     vec![]
/// );
/// ```
pub struct Expense {
    pub amount: f64,
    pub category: String,
    // ...
}
```

**Documentation Guidelines:**
- Use `///` for item documentation (functions, structs, enums)
- Use `//!` for module documentation (top of files)
- Include `# Examples` sections with code
- Document public APIs thoroughly
- Cross-reference with `[Type]` or `[function_name]`

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

---

## **GitHub Actions Workflows**

This project includes a comprehensive set of GitHub Actions workflows for automation.

### **Command Dispatch** (`.github/workflows/command-dispatch.yml`)

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

**Example Flow:**
```
PR Comment: /pre-release beta
     ↓
command-dispatch.yml parses command
     ↓
Triggers pre-release.yml with input: prerelease-type=beta
     ↓
pre-release.yml executes
     ↓
Comments back to PR with results
```

See [.github/COMMAND_DISPATCH_GUIDE.md](.github/COMMAND_DISPATCH_GUIDE.md) for detailed documentation.

---

### **Docker Release** (`.github/workflows/release-docker.yml`)

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

### **Workflow Summary**

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

**Recent Changes (Uncommitted):**
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
README.md                      # This file - comprehensive updates
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