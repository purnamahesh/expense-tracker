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

Would you like some tips on how to structure the code or which concepts to focus on first?

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

To require tests to pass before merging:
1. Go to repository Settings → Branches
2. Add branch protection rule for `main`/`master`
3. Enable "Require status checks to pass before merging"
4. Select "test-summary" from the list
5. Enable "Require branches to be up to date before merging" (recommended)

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

## **Version Management**

This project includes automated version bumping using semantic versioning (semver).

### **Available Tools:**

#### 1. **GitHub Actions** (`.github/workflows/release.yml`)
Automatically bumps version and publishes to crates.io when pushing to `main` or `master` branch.

**How it works:**
- **Check for Changes:**
  - Skips release if no `.rs` files changed since last tag
  - Displays skip message in workflow logs
- **Job 1 (Version Bump):**
  - Analyzes commit messages since the last git tag
  - Determines version bump type based on Conventional Commits
  - Commits changes and creates a git tag
  - Pushes everything back to the repository
- **Job 2 (Publish):**
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
./bump-version.sh [major|minor|patch]
```

**Examples:**
```bash
./bump-version.sh patch   # 0.1.0 → 0.1.1 (bug fixes)
./bump-version.sh minor   # 0.1.0 → 0.2.0 (new features)
./bump-version.sh major   # 0.1.0 → 1.0.0 (breaking changes)
```

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
- **Check for Changes:**
  - Skips pre-release if no `.rs` files changed in the PR
  - Adds a skip message comment to the PR
- Bumps version with pre-release suffix (e.g., `0.1.1a0`, `1.2.3b2`)
- Publishes to crates.io
- Adds a comment to the PR with version info
- Reacts with 👍 on success or 👎 on failure

**Usage:**
1. Open a pull request
2. Add a comment: `/pre-release --type=alpha` or `/pre-release --type=beta`
3. Workflow automatically:
   - Bumps version with suffix (e.g., `0.1.0` → `0.1.1a0` or `0.1.1a0` → `0.1.1a1`)
   - Updates Cargo.toml and Cargo.lock
   - Commits and tags the version
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

#### 6. **Git Hook** (`.git/hooks/pre-merge-commit`)
Automatically bumps version when merging into `main` or `master`.

**How it works:**
- Runs before a merge commit is created
- Analyzes commits being merged
- Determines version bump based on Conventional Commits
- Automatically stages Cargo.toml and Cargo.lock

**Example workflow:**
```bash
git checkout main
git merge feature/add-categories
# Hook automatically bumps version during merge
```

### **Requirements:**

Install `cargo-bump` for all methods:
```bash
cargo install cargo-bump
```

### **Best Practices:**

1. **Use Conventional Commits** for automatic version detection
2. **Create git tags** for each release to track version history
3. **Push tags** along with commits: `git push origin --tags`
4. Choose the method that fits your workflow:
   - **GitHub Actions**: Best for team projects and CI/CD
   - **Manual Script**: Best for controlled releases
   - **Git Hook**: Best for automatic local version bumping