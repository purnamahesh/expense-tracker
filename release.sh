#!/bin/bash
# Local release script - replicates GitHub Actions release workflow
# Usage: ./release.sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Release Script - Local Workflow${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

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

# Check if on main/master branch
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "")
if [[ "$CURRENT_BRANCH" != "main" && "$CURRENT_BRANCH" != "master" ]]; then
  echo -e "${YELLOW}⚠ Not on main/master branch (current: $CURRENT_BRANCH)${NC}"
  read -p "Continue anyway? (y/N): " -n 1 -r
  echo ""
  if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 1
  fi
else
  echo -e "${GREEN}✓ On $CURRENT_BRANCH branch${NC}"
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
echo -e "${YELLOW}Checking for Rust file changes since last tag...${NC}"
LAST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "")

if [ -z "$LAST_TAG" ]; then
  # No tags exist
  RS_FILES=$(find . -name "*.rs" -type f | wc -l)
  if [ "$RS_FILES" -gt 0 ]; then
    echo -e "${GREEN}✓ Found Rust files in repository${NC}"
    HAS_CHANGES=true
  else
    echo -e "${RED}✗ No Rust files found${NC}"
    exit 1
  fi
else
  # Check for .rs changes since last tag
  CHANGED_RS_FILES=$(git diff --name-only ${LAST_TAG}..HEAD | grep '\.rs$' || true)

  if [ -n "$CHANGED_RS_FILES" ]; then
    echo -e "${GREEN}✓ Rust files changed since ${LAST_TAG}:${NC}"
    echo "$CHANGED_RS_FILES" | sed 's/^/  /'
    HAS_CHANGES=true
  else
    echo -e "${YELLOW}⏭️  No Rust files changed since ${LAST_TAG}${NC}"
    read -p "Continue with release anyway? (y/N): " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
      echo "Aborted."
      exit 0
    fi
    HAS_CHANGES=true
  fi
fi

echo ""

# ========================================
# Determine version bump type
# ========================================
echo -e "${YELLOW}Analyzing commits for version bump type...${NC}"

if [ -z "$LAST_TAG" ]; then
  COMMITS=$(git log --pretty=format:"%s")
else
  COMMITS=$(git log ${LAST_TAG}..HEAD --pretty=format:"%s")
fi

echo "Recent commits:"
echo "$COMMITS" | head -5 | sed 's/^/  /'
if [ $(echo "$COMMITS" | wc -l) -gt 5 ]; then
  echo "  ... and $(($(echo "$COMMITS" | wc -l) - 5)) more"
fi
echo ""

# Determine bump type based on conventional commits
if echo "$COMMITS" | grep -qE "^(BREAKING CHANGE:|.*!:)"; then
  BUMP_TYPE="major"
  echo -e "${BLUE}Detected: MAJOR bump (breaking change)${NC}"
elif echo "$COMMITS" | grep -qE "^feat(\(.+\))?:"; then
  BUMP_TYPE="minor"
  echo -e "${BLUE}Detected: MINOR bump (new feature)${NC}"
elif echo "$COMMITS" | grep -qE "^fix(\(.+\))?:"; then
  BUMP_TYPE="patch"
  echo -e "${BLUE}Detected: PATCH bump (bug fix)${NC}"
else
  BUMP_TYPE="patch"
  echo -e "${BLUE}Defaulting to: PATCH bump${NC}"
fi

echo ""
read -p "Proceed with $BUMP_TYPE version bump? (y/N): " -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
  echo "Aborted."
  exit 0
fi

# ========================================
# Bump version
# ========================================
echo ""
echo -e "${YELLOW}Current version:${NC}"
CURRENT_VERSION=$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)
echo "  $CURRENT_VERSION"

echo ""
echo -e "${YELLOW}Bumping version...${NC}"
cargo bump $BUMP_TYPE

NEW_VERSION=$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)
echo -e "${GREEN}New version: $NEW_VERSION${NC}"

# Update Cargo.lock
echo -e "${YELLOW}Updating Cargo.lock...${NC}"
PACKAGE_NAME=$(cargo pkgid | cut -d# -f1 | rev | cut -d/ -f1 | rev)
cargo update -p "$PACKAGE_NAME" --quiet

echo -e "${GREEN}✓ Version bumped: $CURRENT_VERSION → $NEW_VERSION${NC}"
echo ""

# ========================================
# Commit and tag
# ========================================
read -p "Commit and tag this version? (y/N): " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
  git config user.name "$(git config user.name)"
  git config user.email "$(git config user.email)"
  git add Cargo.toml Cargo.lock
  git commit -m "chore: bump version to $NEW_VERSION [skip ci]"
  git tag -a "v$NEW_VERSION" -m "Release version $NEW_VERSION"

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
read -p "Publish to crates.io? (y/N): " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
  echo -e "${YELLOW}Running dry-run...${NC}"
  if cargo publish --dry-run; then
    echo -e "${GREEN}✓ Dry-run successful${NC}"
    echo ""
    read -p "Proceed with actual publish? (y/N): " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
      echo -e "${YELLOW}Publishing to crates.io...${NC}"
      cargo publish
      echo -e "${GREEN}✓ Published to crates.io${NC}"
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
echo -e "${GREEN}  Release Complete!${NC}"
echo -e "${GREEN}========================================${NC}"
