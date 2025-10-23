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