Here's a great beginner Rust project for you:

## **Personal Expense Tracker (CLI)**

**Project Goal:** Build a command-line application that helps track personal expenses.

**Core Features:**
- Add a new expense with description, amount, and category (food, transport, entertainment, etc.)
- List all expenses
- Calculate total spending
- Filter expenses by category
- Save/load data to a text file (using simple format like CSV or JSON)

**What You'll Learn:**
- Basic Rust syntax and ownership concepts
- Working with `struct` and `enum` types
- Using `Vec` for storing data
- Pattern matching with `match`
- File I/O operations
- Error handling with `Result`
- Parsing command-line arguments (you can use the `clap` crate or just `std::env::args()`)

**Example Usage:**
```
$ expense-tracker add --description "Lunch" --amount 12.50 --category food
$ expense-tracker list
$ expense-tracker total
$ expense-tracker filter --category food
```

This project is perfect because it's:
- Practical and useful
- Small enough to complete in a few days
- Covers fundamental Rust concepts
- Can be extended with more features as you learn (like date tracking, budgets, statistics)

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
git commit -m "feat: add CSV export functionality"

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
./pre-release.sh alpha   # Create alpha pre-release (e.g., 1.0.1a0)
./pre-release.sh beta    # Create beta pre-release (e.g., 1.0.1b0)
```

**Features:**
- ✓ Checks all prerequisites
- ✓ Detects if on feature branch vs base branch
- ✓ Checks for Rust file changes compared to base branch
- ✓ Smart version calculation:
  - Increments if same type (1.0.1a0 → 1.0.1a1)
  - Resets if switching type (1.0.1a2 → 1.0.1b0)
  - Bumps patch if stable (1.0.0 → 1.0.1a0)
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
Publish alpha/beta versions directly from pull requests using PR comments.

**How it works:**
- Triggered by commenting on a PR with `/pre-release --type=alpha` or `/pre-release --type=beta`
- **Job 1 (Check for Changes):**
  - Skips pre-release if no `.rs` files changed in the PR
  - Adds a skip message comment to the PR
- **Job 2 (Version Bump):**
  - Uses the reusable `bump-version.yml` workflow
  - Creates a new branch with pre-release version changes
  - Creates a PR with title: `chore: bump version X.X.X -> Y.Y.YaZ` (or `bZ` for beta)
  - Automatically merges the PR
- **Job 3 (Publish):**
  - Waits for the version bump PR to be merged
  - Creates a git tag for the new pre-release version
  - Publishes to crates.io
  - Adds a comment to the PR with version info
  - Reacts with 👍 on success or 👎 on failure

**Usage:**
1. Open a pull request
2. Add a comment: `/pre-release --type=alpha` or `/pre-release --type=beta`
3. Workflow automatically:
   - Bumps version with suffix (e.g., `0.1.0` → `0.1.1a0` or `0.1.1a0` → `0.1.1a1`)
   - Creates a version bump PR and auto-merges it
   - Updates Cargo.toml and Cargo.lock
   - Creates a git tag for the new version
   - Publishes to crates.io
   - Reports back in PR comments

**Examples:**
```bash
# In a PR comment:
/pre-release --type=alpha    # Creates version like 0.1.1a0, 0.1.1a1, etc.
/pre-release --type=beta     # Creates version like 0.1.1b0, 0.1.1b1, etc.
```

**Version Format:**
- `1.0.0` → `/pre-release --type=alpha` → `1.0.1a0`
- `1.0.1a0` → `/pre-release --type=alpha` → `1.0.1a1`
- `1.0.1a1` → `/pre-release --type=beta` → `1.0.1b0` (switches type)
- `1.0.1b0` → `/pre-release --type=beta` → `1.0.1b1`

**Requirements:**
- Must be run from a pull request
- Requires `CARGO_REGISTRY_TOKEN` secret
- Comment must be exactly `/pre-release --type=alpha` or `/pre-release --type=beta`

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
- ✓ Automatically merges the PR
- ✓ Supports both semantic versioning (major/minor/patch) and pre-releases (alpha/beta)
- ✓ Updates both Cargo.toml and Cargo.lock
- ✓ Returns version information and PR number for downstream jobs

**Benefits:**
- **DRY Principle**: Version bumping logic is centralized in one place
- **Code Review**: PRs allow for review and CI checks before version bump is applied
- **Audit Trail**: Clear history of version changes through PR records
- **Consistency**: Both release and pre-release workflows use the same logic

### **Best Practices:**

1. **Use Conventional Commits** for automatic version detection
2. **Version changes create PRs** instead of direct commits for better traceability
3. **Git tags are created** after PR merge to track version history
4. Choose the method that fits your workflow:
   - **GitHub Actions**: Best for team projects and CI/CD (creates PRs for review)
   - **Manual Script**: Best for controlled releases (direct local control)