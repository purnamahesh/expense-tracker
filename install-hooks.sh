#!/bin/bash

# Install git hooks from the hooks/ directory to .git/hooks/
# This script should be run by each developer after cloning the repository

set -e

HOOKS_DIR="hooks"
GIT_HOOKS_DIR=".git/hooks"

# Color codes
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "Installing git hooks..."

# Check if hooks directory exists
if [ ! -d "$HOOKS_DIR" ]; then
    echo "Error: hooks/ directory not found"
    exit 1
fi

# Check if .git directory exists
if [ ! -d ".git" ]; then
    echo "Error: Not a git repository"
    exit 1
fi

# Install each hook from hooks/ directory
for hook_file in "$HOOKS_DIR"/*; do
    if [ -f "$hook_file" ]; then
        hook_name=$(basename "$hook_file")

        # Copy hook to .git/hooks/
        cp "$hook_file" "$GIT_HOOKS_DIR/$hook_name"
        chmod +x "$GIT_HOOKS_DIR/$hook_name"

        echo -e "${GREEN}✓${NC} Installed $hook_name hook"
    fi
done

echo ""
echo -e "${GREEN}Git hooks installed successfully!${NC}"
echo ""
echo "Installed hooks:"
echo "  - commit-msg: Validates commit message format and suggests [skip tests] suffix"
echo "  - pre-merge-commit: Automatically bumps version when merging to main/master"
echo ""
echo -e "${YELLOW}Note:${NC} These hooks will run automatically on git commit and merge"
