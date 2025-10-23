#!/bin/bash
# Local pre-release script - replicates GitHub Actions pre-release workflow
# Usage: ./pre-release.sh [alpha|beta]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Pre-Release Script - Local Workflow${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Parse argument
PRERELEASE_TYPE=${1:-}

if [[ ! "$PRERELEASE_TYPE" =~ ^(alpha|beta)$ ]]; then
  echo -e "${RED}Error: Invalid or missing pre-release type${NC}"
  echo ""
  echo "Usage: $0 [alpha|beta]"
  echo ""
  echo "Examples:"
  echo "  $0 alpha   # Create alpha pre-release (e.g., 1.0.1a0)"
  echo "  $0 beta    # Create beta pre-release (e.g., 1.0.1b0)"
  echo ""
  exit 1
fi

# ========================================
# Check Prerequisites
# ========================================
echo -e "${YELLOW}Checking prerequisites...${NC}"
echo ""

MISSING_PREREQS=0

# Check if we're in a Rust project
if [ ! -f "Cargo.toml" ]; then
  echo -e "${RED}✗ Cargo.toml not found${NC}"
  echo "  Please run this script from the root of your Rust project."
  exit 1
fi
echo -e "${GREEN}✓ Cargo.toml found${NC}"

# Check if git repo
if ! git rev-parse --git-dir > /dev/null 2>&1; then
  echo -e "${RED}✗ Not a git repository${NC}"
  MISSING_PREREQS=1
else
  echo -e "${GREEN}✓ Git repository${NC}"
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
  echo -e "${YELLOW}⚠ You have uncommitted changes${NC}"
  read -p "Continue anyway? (y/N): " -n 1 -r
  echo ""
  if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 1
  fi
else
  echo -e "${GREEN}✓ No uncommitted changes${NC}"
fi

# Check if cargo-bump is installed
if ! command -v cargo-bump &> /dev/null; then
  echo -e "${RED}✗ cargo-bump not installed${NC}"
  MISSING_PREREQS=1
else
  echo -e "${GREEN}✓ cargo-bump installed${NC}"
fi

# Check if CARGO_REGISTRY_TOKEN is set (for publish)
if [ -z "$CARGO_REGISTRY_TOKEN" ]; then
  echo -e "${YELLOW}⚠ CARGO_REGISTRY_TOKEN not set${NC}"
  echo "  Publishing to crates.io will require manual token input or will be skipped"
else
  echo -e "${GREEN}✓ CARGO_REGISTRY_TOKEN is set${NC}"
fi

echo ""

# Install missing prerequisites
if [ $MISSING_PREREQS -eq 1 ]; then
  echo -e "${YELLOW}Some prerequisites are missing.${NC}"
  read -p "Do you want to install cargo-bump? (y/N): " -n 1 -r
  echo ""
  if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}Installing cargo-bump...${NC}"
    cargo install cargo-bump
    echo -e "${GREEN}✓ cargo-bump installed${NC}"
  else
    echo -e "${RED}Cannot proceed without cargo-bump${NC}"
    exit 1
  fi
fi

# ========================================
# Check for Rust file changes
# ========================================
echo -e "${YELLOW}Checking for Rust file changes...${NC}"

# Get the base branch
BASE_BRANCH=$(git remote show origin 2>/dev/null | grep "HEAD branch" | cut -d ":" -f 2 | xargs || echo "main")
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

if [[ "$CURRENT_BRANCH" == "$BASE_BRANCH" ]]; then
  echo -e "${YELLOW}⚠ You are on the base branch ($BASE_BRANCH)${NC}"
  echo "  Pre-releases are typically created from feature branches"
  read -p "Continue anyway? (y/N): " -n 1 -r
  echo ""
  if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 1
  fi
fi

# Check for .rs changes compared to base branch
git fetch origin $BASE_BRANCH --quiet 2>/dev/null || true
CHANGED_RS_FILES=$(git diff --name-only origin/$BASE_BRANCH...HEAD 2>/dev/null | grep '\.rs$' || true)

if [ -n "$CHANGED_RS_FILES" ]; then
  echo -e "${GREEN}✓ Rust files changed:${NC}"
  echo "$CHANGED_RS_FILES" | sed 's/^/  /'
else
  echo -e "${YELLOW}⏭️  No Rust files changed compared to $BASE_BRANCH${NC}"
  read -p "Continue with pre-release anyway? (y/N): " -n 1 -r
  echo ""
  if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 0
  fi
fi

echo ""

# ========================================
# Bump to pre-release version
# ========================================
echo -e "${YELLOW}Current version:${NC}"
CURRENT_VERSION=$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)
echo "  $CURRENT_VERSION"
echo ""

# Set suffix based on type (a for alpha, b for beta)
if [ "$PRERELEASE_TYPE" = "alpha" ]; then
  SUFFIX="a"
else
  SUFFIX="b"
fi

echo -e "${YELLOW}Calculating new pre-release version...${NC}"

# Check if already a pre-release version with same type
if echo "$CURRENT_VERSION" | grep -qE "${SUFFIX}[0-9]+$"; then
  # Extract base version and current pre-release number
  BASE_VERSION=$(echo "$CURRENT_VERSION" | sed -E "s/${SUFFIX}[0-9]+$//")
  CURRENT_NUM=$(echo "$CURRENT_VERSION" | grep -oE "[0-9]+$")
  NEW_NUM=$((CURRENT_NUM + 1))
  NEW_VERSION="${BASE_VERSION}${SUFFIX}${NEW_NUM}"
  echo -e "${BLUE}Incrementing $PRERELEASE_TYPE version${NC}"
else
  # Check if it's a different pre-release type (switch from a to b or vice versa)
  if echo "$CURRENT_VERSION" | grep -qE "[ab][0-9]+$"; then
    # Remove old pre-release suffix
    BASE_VERSION=$(echo "$CURRENT_VERSION" | sed -E "s/[ab][0-9]+$//")
    NEW_VERSION="${BASE_VERSION}${SUFFIX}0"
    echo -e "${BLUE}Switching to $PRERELEASE_TYPE (from different type)${NC}"
  else
    # Not a pre-release, bump patch and add suffix
    echo -e "${BLUE}Creating first $PRERELEASE_TYPE pre-release${NC}"
    cargo bump patch > /dev/null
    BASE_VERSION=$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)
    NEW_VERSION="${BASE_VERSION}${SUFFIX}0"
  fi
fi

echo -e "${GREEN}New version will be: $NEW_VERSION${NC}"
echo ""

read -p "Proceed with version bump to $NEW_VERSION? (y/N): " -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
  echo "Aborted."
  exit 0
fi

# Update Cargo.toml
echo -e "${YELLOW}Updating Cargo.toml...${NC}"
if [ "$(uname)" = "Darwin" ]; then
  # macOS
  sed -i '' "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
else
  # Linux
  sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
fi

# Update Cargo.lock
echo -e "${YELLOW}Updating Cargo.lock...${NC}"
PACKAGE_NAME=$(cargo pkgid | cut -d# -f1 | rev | cut -d/ -f1 | rev)
cargo update -p "$PACKAGE_NAME" --quiet

echo -e "${GREEN}✓ Version bumped: $CURRENT_VERSION → $NEW_VERSION${NC}"
echo ""

# ========================================
# Commit and tag
# ========================================
read -p "Commit and tag this pre-release? (y/N): " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
  git config user.name "$(git config user.name)"
  git config user.email "$(git config user.email)"
  git add Cargo.toml Cargo.lock
  git commit -m "chore: bump version to $NEW_VERSION [skip ci]"
  git tag -a "v$NEW_VERSION" -m "Pre-release version $NEW_VERSION ($PRERELEASE_TYPE)"

  echo -e "${GREEN}✓ Committed and tagged as v$NEW_VERSION${NC}"
  echo ""

  # Push
  read -p "Push to remote? (y/N): " -n 1 -r
  echo ""
  if [[ $REPLY =~ ^[Yy]$ ]]; then
    git push origin $CURRENT_BRANCH
    git push origin --tags
    echo -e "${GREEN}✓ Pushed to remote${NC}"
  else
    echo -e "${YELLOW}⚠ Remember to push manually:${NC}"
    echo "  git push origin $CURRENT_BRANCH"
    echo "  git push origin --tags"
  fi
else
  echo -e "${YELLOW}Skipped commit and tag${NC}"
fi

echo ""

# ========================================
# Publish to crates.io
# ========================================
read -p "Publish pre-release to crates.io? (y/N): " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
  echo -e "${YELLOW}Running dry-run...${NC}"
  if cargo publish --dry-run; then
    echo -e "${GREEN}✓ Dry-run successful${NC}"
    echo ""
    read -p "Proceed with actual publish? (y/N): " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
      echo -e "${YELLOW}Publishing pre-release to crates.io...${NC}"
      cargo publish
      echo -e "${GREEN}✓ Published to crates.io${NC}"
      echo ""
      echo "Users can install with:"
      echo "  cargo install $PACKAGE_NAME --version $NEW_VERSION"
    else
      echo -e "${YELLOW}Publish cancelled${NC}"
    fi
  else
    echo -e "${RED}✗ Dry-run failed. Fix errors before publishing.${NC}"
  fi
else
  echo -e "${YELLOW}Skipped publishing${NC}"
  echo ""
  echo "To publish manually:"
  echo "  cargo publish"
fi

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  Pre-Release Complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "${BLUE}Summary:${NC}"
echo "  Type: $PRERELEASE_TYPE"
echo "  Version: $NEW_VERSION"
echo "  Branch: $CURRENT_BRANCH"
