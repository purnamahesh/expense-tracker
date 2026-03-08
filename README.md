# pense

A minimal CLI expense tracker backed by SQLite.

## Installation

From crates.io:

```bash
cargo install pense
```

Or build from source:

```bash
cargo install --path .
```

## Usage

```bash
# Add an expense
pense add -a 12.50 -c food -d "Lunch" -t "meal,restaurant"

# List all expenses
pense list

# Calculate total spending
pense total

# Filter expenses
pense filter -c food
pense filter -a 50.0
pense filter -t restaurant
pense filter --ge 10 --le 50
pense filter -c food -l 5

# Use a custom database path
pense -p ./my-expenses.db list

# Help
pense --help
```

## License

MIT
