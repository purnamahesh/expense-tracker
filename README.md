## **Personal Expense Tracker (CLI)**

> **Current Status:** Active development on `feature/clap` branch
> **Version:** 0.5.2
> **Latest Update:** Migrating to `clap` for CLI argument parsing

A command-line application for tracking personal expenses with categories, tags, and filtering capabilities.

---

## **Documentation Index**

- **[VCS-README.md](VCS-README.md)** - GitHub workflows, CI/CD, version management, and testing
- **[HOOKS-README.md](HOOKS-README.md)** - Git hooks and bash automation scripts
- **[ROADMAP.md](ROADMAP.md)** - Future features and enhancement plans

---

## **Core Features**

- ✅ Add expenses with description, amount, category, and tags
- ✅ List all expenses with formatted output
- ✅ Calculate total spending
- ✅ Filter expenses by category, amount, or tags
- ✅ DateTime tracking for each expense
- ✅ PSV (Pipe-Separated Values) storage format
- 🚧 SQLite migration (planned)

**What You'll Learn:**
- Basic Rust syntax and ownership concepts
- Working with `struct` and `enum` types
- Using `Vec` for storing data
- Pattern matching with `match`
- File I/O operations
- Error handling with `Result`
- ✅ Parsing command-line arguments using `clap` crate with derive macros

---

## **Installation**

### **1. Clone the Repository**
```bash
git clone <repository-url>
cd expense-tracker
```

### **2. Build the Project**
```bash
cargo build --release
```

### **3. Install Git Hooks** (Optional but Recommended)
```bash
./install-hooks.sh
```
See [HOOKS-README.md](HOOKS-README.md) for details on commit message validation and automatic version bumping.

---

## **Usage**

### **Add an Expense**
```bash
# Basic expense
$ expense-tracker add --amount 12.50 --category food --description "Lunch"

# With tags
$ expense-tracker add --amount 12.50 --category food --description "Lunch" --tag meal --tag restaurant
```

### **List Expenses**
```bash
$ expense-tracker list
```

### **Calculate Total**
```bash
$ expense-tracker total
```

### **Filter Expenses**
```bash
# By category
$ expense-tracker filter --category food

# By amount
$ expense-tracker filter --amount 50.0

# By tags
$ expense-tracker filter --tag food --tag coffee
```

---

## **Project Structure**

```
src/
├── main.rs           # Entry point
├── cli.rs            # Clap CLI definitions
├── config.rs         # Configuration constants
├── utils.rs          # Module declarations
└── utils/
    ├── expense.rs    # Expense struct and logic
    └── file_parser.rs # File I/O operations
```

---

## **Current Implementation (v0.5.2)**

- ✅ **Clap-based CLI** - Modern argument parsing with derive macros (`src/cli.rs:1`)
- ✅ **Subcommands** - `add`, `list`, `total`, `filter`
- ✅ **Type-safe arguments** - Validated by clap at compile time
- ✅ **DateTime tracking** - Automatic timestamp for each expense
- ✅ **Multi-tag support** - Multiple tags per expense
- ✅ **PSV storage** - Pipe-separated values format

---

## **Dependencies**

- `chrono = "0.4.42"` - DateTime handling
- `clap = { version = "4.5.51", features = ["derive"] }` - CLI parsing

---

## **Contributing**

See [VCS-README.md](VCS-README.md) for:
- Testing guidelines
- Pull request requirements
- Version management workflows
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

## **Why This Project?**

This project is perfect because it's:
- **Practical and useful** - Real-world CLI application
- **Small enough** - Can be completed in a few days
- **Covers fundamentals** - Core Rust concepts
- **Extensible** - Easy to add more features as you learn
