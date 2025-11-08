# **Git Hooks & Bash Scripts**

[ê Back to Main README](README.md) | [VCS & CI/CD](VCS-README.md) | [Roadmap](ROADMAP.md)

---

This document covers git hooks, commit conventions, and all bash automation scripts for the Expense Tracker project.

---

## **Table of Contents**

- [Git Hooks](#git-hooks)
  - [Installation](#installation)
  - [commit-msg Hook](#commit-msg-hook)
  - [pre-merge-commit Hook](#pre-merge-commit-hook)
  - [Valid Commit Prefixes](#valid-commit-prefixes)
  - [Example Workflow](#example-workflow)
- [Bash Scripts](#bash-scripts)
  - [Test Script (test.sh)](#test-script-testsh)
  - [Release Script (release.sh)](#release-script-releasesh)
  - [Pre-Release Script (pre-release.sh)](#pre-release-script-pre-releasesh)
  - [Manual Version Bump (bump-version.sh)](#manual-version-bump-bump-versionsh)
  - [Documentation Script (generate-docs.sh)](#documentation-script-generate-docssh)

---

## **Git Hooks**

This project uses git hooks to enforce commit message standards and automate version management.

### **Installation**

Run the installation script to set up git hooks:

```bash
./install-hooks.sh
```

This installs two hooks:

### **commit-msg Hook**

**File:** `hooks/commit-msg`

Validates commit messages follow [Conventional Commits](https://www.conventionalcommits.org/) format.

**What it does:**
-  Enforces commit message prefix (feat:, fix:, chore:, docs:, etc.)
-  Suggests adding `[skip tests]` suffix if not present
-  Blocks commits with invalid format
-  Shows helpful error messages with examples

**Valid commit message format:**
```bash
<type>: <description>
```

**Examples:**
```bash
 git commit -m "feat: add user authentication"
 git commit -m "fix: correct date parsing [skip tests]"
 git commit -m "docs: update README [skip tests]"
 git commit -m "feat!: redesign API (breaking change)"

L git commit -m "Add new feature"  # Missing prefix
L git commit -m "Feature: add auth"  # Wrong prefix
L git commit -m "feat add auth"  # Missing colon
```

### **pre-merge-commit Hook**

**File:** `hooks/pre-merge-commit`

Automatically bumps version when merging into main/master branch.

**What it does:**
-  Detects merge commits to main/master
-  Analyzes merged commits for version bump type
-  Automatically updates Cargo.toml and Cargo.lock
-  Stages version changes with the merge commit
-  Uses conventional commit format to determine bump type:
  - `feat!:`, `BREAKING CHANGE` í **MAJOR** bump (1.0.0 í 2.0.0)
  - `feat:` í **MINOR** bump (1.0.0 í 1.1.0)
  - `fix:`, `patch:` í **PATCH** bump (1.0.0 í 1.0.1)

**Requirement:** Install `cargo-bump` for automatic version bumping:
```bash
cargo install cargo-bump
```

### **Valid Commit Prefixes**

| Prefix | Description | Version Bump | Example |
|--------|-------------|--------------|---------|
| `feat:` | New feature | MINOR (0.1.0 í 0.2.0) | `feat: add user authentication` |
| `feat!:` | Breaking change | MAJOR (0.1.0 í 1.0.0) | `feat!: redesign API` |
| `fix:` | Bug fix | PATCH (0.1.0 í 0.1.1) | `fix: resolve login crash` |
| `patch:` | Small patch | PATCH (0.1.0 í 0.1.1) | `patch: fix typo` |
| `docs:` | Documentation | No bump | `docs: update README [skip tests]` |
| `chore:` | Maintenance | No bump | `chore: update dependencies` |
| `style:` | Code style | No bump | `style: format code` |
| `refactor:` | Code refactoring | No bump | `refactor: simplify logic` |
| `perf:` | Performance | PATCH | `perf: optimize queries` |
| `test:` | Testing | No bump | `test: add unit tests` |
| `build:` | Build system | No bump | `build: update deps` |
| `ci:` | CI/CD config | No bump | `ci: update workflow` |
| `revert:` | Revert commit | No bump | `revert: revert commit abc123` |

Add `!` before `:` for breaking changes (e.g., `feat!:`, `fix!:`)

### **Bypassing Hooks**

Not recommended, but possible:
```bash
git commit -m "message" --no-verify
```

### **Example Workflow**

```bash
# Create a feature branch
git checkout -b feature/add-export

# Make changes and commit (commit-msg hook validates)
git commit -m "feat: add PSV export functionality"

# Switch to main and merge (pre-merge-commit hook bumps version)
git checkout main
git merge feature/add-export
# í Hook automatically bumps version from 1.0.0 to 1.1.0
# í Cargo.toml and Cargo.lock are updated and staged

# Push changes
git push origin main
```

**Note:** The git hooks are versioned in the `hooks/` directory. If hooks are updated, run `./install-hooks.sh` again to update your local `.git/hooks/`.

---

## **Bash Scripts**

### **Test Script (test.sh)**

**File:** `test.sh`

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
-  Prerequisite checks (cargo, cargo-watch, cargo-llvm-cov)
-  Offers to install missing tools
-  Shows uncommitted Rust file changes
-  Runs formatting check (`cargo fmt`)
-  Runs clippy linter
-  Builds project before tests
-  Color-coded output
-  Clear pass/fail summary

**Example Output:**
```bash
$ ./test.sh --unit
========================================
  Test Suite - Local Execution
========================================

Checking prerequisites...
 Cargo.toml found
 cargo installed

9 Uncommitted Rust file changes detected:
  src/main.rs
  src/utils/expense.rs

Checking code formatting...
 Code formatting is correct

Running clippy (linter)...
 Clippy checks passed

Building project...
 Build successful

Running Unit Tests...
Command: cargo test --lib

 Unit Tests passed

========================================
   All tests passed!
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

### **Release Script (release.sh)**

**File:** `release.sh`

Interactive script that replicates the full GitHub Actions release workflow locally.

**Usage:**
```bash
./release.sh
```

**Features:**
-  Checks all prerequisites (cargo-bump, git, CARGO_REGISTRY_TOKEN, etc.)
-  Detects Rust file changes since last tag
-  Analyzes commits for automatic version bump type (major/minor/patch)
-  Updates both Cargo.toml and Cargo.lock
-  Interactive y/n prompts for each major action:
  - Version bump
  - Commit and tag
  - Push to remote
  - Publish to crates.io (with dry-run first)
-  Color-coded output for better readability

**Example session:**
```bash
$ ./release.sh
========================================
  Release Script - Local Workflow
========================================

Checking prerequisites...
 Cargo.toml found
 Git repository
 On main branch
 No uncommitted changes
 cargo-bump installed
 CARGO_REGISTRY_TOKEN is set

 Rust files changed since v0.1.3:
  src/main.rs
  src/utils/expense.rs

Analyzing commits for version bump type...
Recent commits:
  feat: add new expense filtering
  fix: correct date parsing

Detected: MINOR bump (new feature)

Proceed with minor version bump? (y/N):
```

**Requirements:**
```bash
# Install cargo-bump
cargo install cargo-bump

# Set CARGO_REGISTRY_TOKEN environment variable
export CARGO_REGISTRY_TOKEN=<your-token>
```

---

### **Pre-Release Script (pre-release.sh)**

**File:** `pre-release.sh`

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
-  Checks all prerequisites
-  Detects if on feature branch vs base branch
-  Checks for Rust file changes compared to base branch
-  Smart version calculation:
  - Increments if same type (1.0.1-alpha.0 í 1.0.1-alpha.1)
  - Resets if switching type (1.0.1-alpha.2 í 1.0.1-beta.0)
  - Bumps patch if stable (1.0.0 í 1.0.1-alpha.0)
-  Interactive y/n prompts for each action
-  Portable (works on macOS and Linux)

**Version Format:**
- `1.0.0` í `./pre-release.sh alpha` í `1.0.1-alpha.0`
- `1.0.1-alpha.0` í `./pre-release.sh alpha` í `1.0.1-alpha.1`
- `1.0.1-alpha.1` í `./pre-release.sh beta` í `1.0.1-beta.0` (switches type)
- `1.0.1-beta.0` í `./pre-release.sh beta` í `1.0.1-beta.1`

---

### **Manual Version Bump (bump-version.sh)**

**File:** `bump-version.sh`

Simple interactive script for quick version bumping.

**Usage:**
```bash
./bump-version.sh <major|minor|patch>
```

**Examples:**
```bash
./bump-version.sh patch   # 0.1.0 í 0.1.1 (bug fixes)
./bump-version.sh minor   # 0.1.0 í 0.2.0 (new features)
./bump-version.sh major   # 0.1.0 í 1.0.0 (breaking changes)
```

**Note:** Bump type argument is **required** (no longer defaults to patch).

**Features:**
- Validates bump type
- Shows current and new version
- Updates Cargo.lock
- Optionally commits and tags the version
- Provides helpful instructions for pushing
- Shows command to publish to crates.io

**Example Session:**
```bash
$ ./bump-version.sh minor

========================================
  Version Bump Script
========================================

Current version: 0.1.0
New version: 0.2.0

Bumping version...
 Version bumped successfully

Commit and tag this version? (y/N): y

 Changes committed
 Tag v0.2.0 created

Next steps:
  git push origin main
  git push origin v0.2.0

To publish to crates.io:
  cargo publish
```

**Requirements:**
```bash
cargo install cargo-bump
```

---

### **Documentation Script (generate-docs.sh)**

**File:** `generate-docs.sh`

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

**Documentation Guidelines:**
- Use `///` for item documentation (functions, structs, enums)
- Use `//!` for module documentation (top of files)
- Include `# Examples` sections with code
- Document public APIs thoroughly
- Cross-reference with `[Type]` or `[function_name]`

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

See [docs-template.md](docs-template.md) for a comprehensive documentation writing guide.

---

## **Best Practices**

### **For Git Hooks:**
1. **Always use conventional commits** - Enables automation
2. **Install hooks after clone** - Run `./install-hooks.sh`
3. **Update hooks when changed** - Re-run installation script
4. **Use `[skip tests]` for docs** - Speeds up CI for documentation changes
5. **Don't bypass hooks** - They ensure consistency

### **For Bash Scripts:**
1. **Run tests before committing** - Use `./test.sh`
2. **Test locally before pushing** - Use `./release.sh` for dry-runs
3. **Use pre-releases for testing** - Try `./pre-release.sh alpha` first
4. **Generate docs regularly** - Keep documentation up-to-date
5. **Check prerequisites** - Scripts will guide you if tools are missing

### **For Version Management:**
1. **Follow semantic versioning** - MAJOR.MINOR.PATCH
2. **Use conventional commits** - Automates bump type detection
3. **Test before release** - Use pre-release versions
4. **Tag releases properly** - Hooks/scripts handle this automatically
5. **Document breaking changes** - Use `feat!:` or `BREAKING CHANGE:`

---

## **Troubleshooting**

### **Hooks not working?**

```bash
# Reinstall hooks
./install-hooks.sh

# Check if hooks are executable
ls -la .git/hooks/

# Make hooks executable if needed
chmod +x .git/hooks/commit-msg
chmod +x .git/hooks/pre-merge-commit
```

### **cargo-bump not found?**

```bash
# Install cargo-bump
cargo install cargo-bump

# Verify installation
cargo bump --version
```

### **Script permission denied?**

```bash
# Make script executable
chmod +x test.sh
chmod +x release.sh
chmod +x pre-release.sh
chmod +x bump-version.sh
chmod +x generate-docs.sh
chmod +x install-hooks.sh
```

### **Version bump failed?**

Check:
- `cargo-bump` is installed
- No uncommitted changes
- On correct branch
- Cargo.toml has valid version

### **Tests failing locally?**

```bash
# Check formatting
cargo fmt --check

# Fix formatting
cargo fmt

# Check lints
cargo clippy

# Fix lints automatically
cargo clippy --fix

# Run specific test
cargo test test_name
```

---

## **Related Documentation**

- [Main README](README.md) - Application usage and installation
- [VCS & CI/CD](VCS-README.md) - GitHub workflows and automation
- [Roadmap](ROADMAP.md) - Future features and enhancements
- [docs-template.md](docs-template.md) - Documentation writing guide

---

[ê Back to Main README](README.md) | [VCS & CI/CD](VCS-README.md) | [Roadmap](ROADMAP.md)
