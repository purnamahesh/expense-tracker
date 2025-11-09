## **Personal Expense Tracker (CLI)**

[![Crates.io](https://img.shields.io/crates/v/expense-tracker.svg)](https://crates.io/crates/expense-tracker)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A command-line application for tracking personal expenses with categories, tags, and filtering capabilities. Built with Rust for performance and reliability.

---

## **Documentation Index**

- **[VCS-README.md](VCS-README.md)** - GitHub workflows, CI/CD, version management, and testing
- **[HOOKS-README.md](HOOKS-README.md)** - Git hooks and bash automation scripts
- **[ROADMAP.md](ROADMAP.md)** - Future features and enhancement plans

---

## **Features**

### **Core Functionality**
- ✅ Add expenses with amount, category, description, and multiple tags
- ✅ List all expenses with formatted output
- ✅ Calculate total spending
- ✅ Filter expenses by category, amount, or tags
- ✅ Automatic DateTime tracking for each expense
- ✅ PSV (Pipe-Separated Values) storage format
- ✅ Custom file paths with `~` expansion support

### **Development & Quality**
- ✅ Modern CLI with `clap` derive macros
- ✅ Comprehensive test suite (unit, integration, doc tests)
- ✅ Idiomatic error handling with `Result<>`
- ✅ CI/CD with GitHub Actions
- ✅ Automated version management and releases
- ✅ Code coverage reporting
- 🚧 SQLite migration (planned)


---

## **Installation**

### **Option 1: Install from crates.io** (Recommended)

```bash
cargo install expense-tracker
```

The binary will be installed to `~/.cargo/bin/` (make sure it's in your PATH).

### **Option 2: Build from Source**

#### **1. Clone the Repository**
```bash
git clone <repository-url>
cd expense-tracker
```

#### **2. Build and Install**
```bash
cargo install --path .
```

Or just build without installing:
```bash
cargo build --release
# Binary will be at: target/release/expense-tracker
```

#### **3. Install Git Hooks** (Optional - for contributors)
```bash
./install-hooks.sh
```
See [HOOKS-README.md](HOOKS-README.md) for details on commit message validation and automatic version bumping.

---

## **Usage**

### **Basic Commands**

```bash
# Add an expense
expense-tracker add --amount 12.50 --category food --description "Lunch"

# Add expense with tags
expense-tracker add -a 12.50 -c food -d "Lunch" --tag meal --tag restaurant

# List all expenses
expense-tracker list

# Calculate total spending
expense-tracker total

# Filter by category
expense-tracker filter --category food

# Filter by amount
expense-tracker filter --amount 50.0

# Filter by tags
expense-tracker filter --tag restaurant
```

### **Custom File Paths**

```bash
# Use custom database file
expense-tracker -p ~/my-expenses.psv list

# Works with all commands
expense-tracker -p ./data/expenses.psv add -a 25.00 -c transport -d "Taxi"
```

### **Get Help**

```bash
# General help
expense-tracker --help

# Command-specific help
expense-tracker add --help
expense-tracker filter --help
```

---

## **Project Structure**

```
src/
├── main.rs           # Entry point with Result<> error handling
├── lib.rs            # Library exports
├── cli.rs            # Clap CLI definitions and argument validation
├── config.rs         # Configuration constants (file format, etc.)
├── expense.rs        # Expense struct, business logic, and operations
├── file_parser.rs    # File I/O operations with error handling
└── path.rs           # Path utilities (~ expansion, validation)

tests/
├── integration_test.rs       # Integration tests with assert_cmd
└── resources/
    └── mock_expenses.psv     # Test fixtures

.github/workflows/
├── test.yml                  # Comprehensive test suite
├── release.yml               # Automated releases
├── pr-title-check.yml        # PR title validation
├── command-dispatch.yml      # /bump, /test, /docs commands
├── docs.yml                  # Documentation generation
└── pre-release.yml           # Pre-release versions
```

---

## **Technical Details**

**Version:** 0.5.3-alpha.1

**Implementation Highlights:**
- ✅ **Clap derive macros** - Type-safe CLI with automatic help generation
- ✅ **Result<> error handling** - Idiomatic Rust error propagation
- ✅ **Custom validators** - Amount validation, file path construction
- ✅ **Integration tests** - Using `assert_cmd` and `rstest`
- ✅ **Path expansion** - Supports `~/path` notation
- ✅ **Subcommands** - `add`, `list`, `total`, `filter`
- ✅ **Optional arguments** - Description and tags are optional
- ✅ **Multi-tag support** - Add multiple tags per expense

---

## **Dependencies**

### **Runtime Dependencies**
- `chrono = "0.4.42"` - DateTime handling and formatting
- `clap = { version = "4.5.51", features = ["derive"] }` - CLI parsing with derive macros

### **Development Dependencies**
- `assert_cmd = "2.1.1"` - Integration testing for CLI applications
- `rstest = "0.26.1"` - Fixture-based testing framework

---

## **Contributing**

### **Quick Start**

**PR Commands** (comment on your PR):
- `/bump` - Auto-detect version bump from PR title (feat: → minor, fix: → patch, etc.)
- `/pre-release alpha` - Create pre-release (alpha/beta/rc)
- `/test` - Run full test suite
- `/docs` - Generate and commit documentation

**Note:** Make sure your PR title follows conventional commit format (e.g., `feat:`, `fix:`, `chore:`). The `/bump` command reads your PR title to determine the version bump type.

See [VCS-README.md](VCS-README.md) for:
- Complete workflow documentation
- Testing guidelines
- Pull request requirements
- Version management system
- Documentation generation

See [HOOKS-README.md](HOOKS-README.md) for:
- Git hooks setup
- Commit message standards
- Bash automation scripts

---

## **Future Enhancements**

See [ROADMAP.md](ROADMAP.md) for planned features including:
- Export to multiple formats (JSON, CSV, HTML)
- Database encryption
- Enhanced UI with tables and coloring
- Multi-user support
- Error handling improvements

---

## **Development**

### **Running Tests**

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_list

# Run integration tests only
cargo test --test integration_test
```

### **Linting and Formatting**

```bash
# Check formatting
cargo fmt --check

# Format code
cargo fmt

# Run clippy
cargo clippy --all-targets --all-features

# Fix clippy warnings
cargo clippy --fix
```

### **Building for Release**

```bash
# Build optimized binary
cargo build --release

# Run benchmarks (if available)
cargo bench
```

---

## **License**

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## **Why This Project?**

This project demonstrates:
- ✅ **Practical CLI application** - Real-world use case
- ✅ **Modern Rust practices** - Idiomatic error handling, derive macros
- ✅ **Comprehensive testing** - Unit, integration, and doc tests
- ✅ **CI/CD pipeline** - Automated testing, releases, and publishing
- ✅ **Clean architecture** - Separation of concerns, modular design
- ✅ **Production-ready** - Published on crates.io with semantic versioning
