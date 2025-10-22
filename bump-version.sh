#!/bin/bash
# Manual version bumping script for Cargo.toml
# Usage: ./bump-version.sh [major|minor|patch]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Parse argument (major, minor, patch)
BUMP_TYPE=${1:-patch}

# Validate bump type
if [[ ! "$BUMP_TYPE" =~ ^(major|minor|patch)$ ]]; then
  echo -e "${RED}Error: Invalid bump type '$BUMP_TYPE'${NC}"
  echo ""
  echo "Usage: $0 [major|minor|patch]"
  echo ""
  echo "Examples:"
  echo "  $0 major   # 0.1.0 → 1.0.0 (breaking changes)"
  echo "  $0 minor   # 0.1.0 → 0.2.0 (new features)"
  echo "  $0 patch   # 0.1.0 → 0.1.1 (bug fixes)"
  echo ""
  exit 1
fi

# Check if we're in a Rust project
if [ ! -f "Cargo.toml" ]; then
  echo -e "${RED}Error: Cargo.toml not found in current directory${NC}"
  echo "Please run this script from the root of your Rust project."
  exit 1
fi

# Install cargo-bump if needed
if ! command -v cargo-bump &> /dev/null; then
  echo -e "${YELLOW}cargo-bump not found. Installing...${NC}"
  cargo install cargo-bump
  echo ""
fi

# Get current version
CURRENT_VERSION=$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)
echo -e "${YELLOW}Current version: ${NC}$CURRENT_VERSION"

# Bump version
echo -e "${YELLOW}Bumping $BUMP_TYPE version...${NC}"
cargo bump $BUMP_TYPE

# Get new version
NEW_VERSION=$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)

# Update Cargo.lock with new version
echo -e "${YELLOW}Updating Cargo.lock...${NC}"
PACKAGE_NAME=$(cargo pkgid | cut -d# -f1 | rev | cut -d/ -f1 | rev)
cargo update -p "$PACKAGE_NAME" --quiet 2>/dev/null || true

echo ""
echo -e "${GREEN}✓ Version bumped successfully!${NC}"
echo -e "  $CURRENT_VERSION → $NEW_VERSION"
echo ""

# Ask if user wants to commit and tag
read -p "Do you want to commit and tag this version? (y/N): " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
  # Check if git repo
  if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo -e "${RED}Error: Not a git repository${NC}"
    exit 1
  fi

  # Check for uncommitted changes (excluding Cargo files)
  if ! git diff --quiet --exit-code -- ':!Cargo.toml' ':!Cargo.lock'; then
    echo -e "${YELLOW}Warning: You have uncommitted changes (excluding Cargo files)${NC}"
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
      echo "Aborted."
      exit 1
    fi
  fi

  # Stage Cargo files
  git add Cargo.toml Cargo.lock 2>/dev/null || git add Cargo.toml

  # Commit
  COMMIT_MSG="chore: bump version to $NEW_VERSION"
  git commit -m "$COMMIT_MSG"

  # Create tag
  TAG_NAME="v$NEW_VERSION"
  git tag -a "$TAG_NAME" -m "Release version $NEW_VERSION"

  echo ""
  echo -e "${GREEN}✓ Committed and tagged as $TAG_NAME${NC}"
  echo ""
  echo "Next steps:"
  echo "  git push origin $(git rev-parse --abbrev-ref HEAD)"
  echo "  git push origin --tags"
  echo ""
  echo "To publish to crates.io:"
  echo "  cargo publish"
else
  echo ""
  echo "Version bumped but not committed."
  echo "To commit manually:"
  echo "  git add Cargo.toml Cargo.lock"
  echo "  git commit -m 'chore: bump version to $NEW_VERSION'"
  echo "  git tag v$NEW_VERSION"
  echo ""
  echo "To publish to crates.io:"
  echo "  cargo publish"
fi

echo ""
echo -e "${GREEN}Done!${NC}"
