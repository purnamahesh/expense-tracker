#!/bin/bash
# Local test script - runs unit and integration tests
# Usage: ./test.sh [OPTIONS]
#
# Options:
#   --unit         Run only unit tests
#   --integration  Run only integration tests
#   --doc          Run only doc tests
#   --coverage     Generate code coverage report
#   --watch        Run tests in watch mode
#   --verbose      Show verbose output
#   --release      Run tests in release mode
#   --help         Show this help message

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Default options
RUN_UNIT=false
RUN_INTEGRATION=false
RUN_DOC=false
RUN_COVERAGE=false
WATCH_MODE=false
VERBOSE=""
RELEASE=""
RUN_ALL=true

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --unit)
      RUN_UNIT=true
      RUN_ALL=false
      shift
      ;;
    --integration)
      RUN_INTEGRATION=true
      RUN_ALL=false
      shift
      ;;
    --doc)
      RUN_DOC=true
      RUN_ALL=false
      shift
      ;;
    --coverage)
      RUN_COVERAGE=true
      RUN_ALL=false
      shift
      ;;
    --watch)
      WATCH_MODE=true
      shift
      ;;
    --verbose)
      VERBOSE="--verbose"
      shift
      ;;
    --release)
      RELEASE="--release"
      shift
      ;;
    --help)
      echo "Usage: $0 [OPTIONS]"
      echo ""
      echo "Options:"
      echo "  --unit         Run only unit tests"
      echo "  --integration  Run only integration tests"
      echo "  --doc          Run only doc tests"
      echo "  --coverage     Generate code coverage report"
      echo "  --watch        Run tests in watch mode (requires cargo-watch)"
      echo "  --verbose      Show verbose output"
      echo "  --release      Run tests in release mode"
      echo "  --help         Show this help message"
      echo ""
      echo "Examples:"
      echo "  $0                    # Run all tests"
      echo "  $0 --unit             # Run only unit tests"
      echo "  $0 --coverage         # Generate coverage report"
      echo "  $0 --watch --unit     # Watch and run unit tests on change"
      exit 0
      ;;
    *)
      echo -e "${RED}Unknown option: $1${NC}"
      echo "Use --help for usage information"
      exit 1
      ;;
  esac
done

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Test Suite - Local Execution${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# ========================================
# Check Prerequisites
# ========================================
echo -e "${YELLOW}Checking prerequisites...${NC}"

# Check if we're in a Rust project
if [ ! -f "Cargo.toml" ]; then
  echo -e "${RED}✗ Cargo.toml not found${NC}"
  echo "  Please run this script from the root of your Rust project."
  exit 1
fi
echo -e "${GREEN}✓ Cargo.toml found${NC}"

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
  echo -e "${RED}✗ cargo not installed${NC}"
  exit 1
fi
echo -e "${GREEN}✓ cargo installed${NC}"

# Check for cargo-watch if watch mode requested
if [ "$WATCH_MODE" = true ]; then
  if ! command -v cargo-watch &> /dev/null; then
    echo -e "${YELLOW}⚠ cargo-watch not installed${NC}"
    read -p "Install cargo-watch? (y/N): " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
      echo -e "${YELLOW}Installing cargo-watch...${NC}"
      cargo install cargo-watch
      echo -e "${GREEN}✓ cargo-watch installed${NC}"
    else
      echo -e "${RED}Cannot use watch mode without cargo-watch${NC}"
      exit 1
    fi
  else
    echo -e "${GREEN}✓ cargo-watch installed${NC}"
  fi
fi

# Check for cargo-llvm-cov if coverage requested
if [ "$RUN_COVERAGE" = true ]; then
  if ! command -v cargo-llvm-cov &> /dev/null; then
    echo -e "${YELLOW}⚠ cargo-llvm-cov not installed${NC}"
    read -p "Install cargo-llvm-cov? (y/N): " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
      echo -e "${YELLOW}Installing cargo-llvm-cov...${NC}"
      cargo install cargo-llvm-cov
      echo -e "${GREEN}✓ cargo-llvm-cov installed${NC}"
    else
      echo -e "${RED}Cannot generate coverage without cargo-llvm-cov${NC}"
      exit 1
    fi
  else
    echo -e "${GREEN}✓ cargo-llvm-cov installed${NC}"
  fi
fi

echo ""

# ========================================
# Check for Rust file changes (optional info)
# ========================================
if git rev-parse --git-dir > /dev/null 2>&1; then
  LAST_COMMIT=$(git rev-parse HEAD 2>/dev/null || echo "")
  if [ -n "$LAST_COMMIT" ]; then
    CHANGED_RS_FILES=$(git diff --name-only HEAD | grep '\.rs$' || true)
    if [ -n "$CHANGED_RS_FILES" ]; then
      echo -e "${CYAN}ℹ Uncommitted Rust file changes detected:${NC}"
      echo "$CHANGED_RS_FILES" | sed 's/^/  /'
      echo ""
    fi
  fi
fi

# ========================================
# Run Tests
# ========================================

FAILED=false

# Function to run tests with error handling
run_test() {
  local test_name=$1
  local test_command=$2

  echo -e "${YELLOW}Running $test_name...${NC}"
  echo -e "${CYAN}Command: $test_command${NC}"
  echo ""

  if eval $test_command; then
    echo ""
    echo -e "${GREEN}✓ $test_name passed${NC}"
    echo ""
  else
    echo ""
    echo -e "${RED}✗ $test_name failed${NC}"
    echo ""
    FAILED=true
  fi
}

# Watch mode
if [ "$WATCH_MODE" = true ]; then
  echo -e "${CYAN}Starting watch mode...${NC}"
  echo -e "${YELLOW}Tests will run automatically when files change${NC}"
  echo -e "${YELLOW}Press Ctrl+C to stop${NC}"
  echo ""

  WATCH_CMD="cargo watch -x 'test $VERBOSE $RELEASE'"

  if [ "$RUN_UNIT" = true ]; then
    WATCH_CMD="cargo watch -x 'test --lib $VERBOSE $RELEASE'"
  elif [ "$RUN_INTEGRATION" = true ]; then
    WATCH_CMD="cargo watch -x 'test --test \"*\" $VERBOSE $RELEASE'"
  elif [ "$RUN_DOC" = true ]; then
    WATCH_CMD="cargo watch -x 'test --doc $VERBOSE $RELEASE'"
  fi

  eval $WATCH_CMD
  exit 0
fi

# Coverage mode
if [ "$RUN_COVERAGE" = true ]; then
  echo -e "${YELLOW}Generating code coverage report...${NC}"
  echo ""

  if cargo llvm-cov --all-features --workspace $VERBOSE; then
    echo ""
    echo -e "${GREEN}✓ Coverage report generated${NC}"
    echo ""
    echo -e "${CYAN}To view detailed HTML report:${NC}"
    echo "  cargo llvm-cov --all-features --workspace --html"
    echo "  open target/llvm-cov/html/index.html"
    echo ""
    echo -e "${CYAN}To generate lcov.info for external tools:${NC}"
    echo "  cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info"
  else
    echo ""
    echo -e "${RED}✗ Coverage generation failed${NC}"
    exit 1
  fi
  exit 0
fi

# Code formatting check
echo -e "${YELLOW}Checking code formatting...${NC}"
if cargo fmt -- --check 2>/dev/null; then
  echo -e "${GREEN}✓ Code formatting is correct${NC}"
else
  echo -e "${YELLOW}⚠ Code formatting issues found${NC}"
  echo "  Run 'cargo fmt' to fix formatting"
fi
echo ""

# Clippy check
echo -e "${YELLOW}Running clippy (linter)...${NC}"
if cargo clippy --all-targets --all-features -- -D warnings 2>&1; then
  echo -e "${GREEN}✓ Clippy checks passed${NC}"
else
  echo -e "${YELLOW}⚠ Clippy found issues${NC}"
  FAILED=true
fi
echo ""

# Build check
echo -e "${YELLOW}Building project...${NC}"
if cargo build $VERBOSE $RELEASE; then
  echo -e "${GREEN}✓ Build successful${NC}"
else
  echo -e "${RED}✗ Build failed${NC}"
  exit 1
fi
echo ""

# Run specified tests or all tests
if [ "$RUN_ALL" = true ]; then
  run_test "Unit Tests" "cargo test --lib $VERBOSE $RELEASE"
  run_test "Integration Tests" "cargo test --test '*' $VERBOSE $RELEASE"
  run_test "Doc Tests" "cargo test --doc $VERBOSE $RELEASE"
  run_test "All Tests (with all features)" "cargo test --all-features $VERBOSE $RELEASE"
else
  if [ "$RUN_UNIT" = true ]; then
    run_test "Unit Tests" "cargo test --lib $VERBOSE $RELEASE"
  fi

  if [ "$RUN_INTEGRATION" = true ]; then
    run_test "Integration Tests" "cargo test --test '*' $VERBOSE $RELEASE"
  fi

  if [ "$RUN_DOC" = true ]; then
    run_test "Doc Tests" "cargo test --doc $VERBOSE $RELEASE"
  fi
fi

# ========================================
# Summary
# ========================================
echo -e "${BLUE}========================================${NC}"
if [ "$FAILED" = true ]; then
  echo -e "${RED}  ✗ Some tests failed${NC}"
  echo -e "${BLUE}========================================${NC}"
  exit 1
else
  echo -e "${GREEN}  ✓ All tests passed!${NC}"
  echo -e "${BLUE}========================================${NC}"
  exit 0
fi
