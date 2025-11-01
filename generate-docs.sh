#!/usr/bin/env bash

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo "========================================"
echo "  Documentation Generation Script"
echo "========================================"
echo ""

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}✗ cargo is not installed${NC}"
    exit 1
fi

echo -e "${GREEN}✓ cargo is installed${NC}"
echo ""

# Function to display menu
show_menu() {
    echo -e "${BLUE}Choose an option:${NC}"
    echo "1) Generate documentation (HTML)"
    echo "2) Generate and open documentation in browser"
    echo "3) Generate documentation with private items"
    echo "4) Check for broken doc links"
    echo "5) Run doc tests"
    echo "6) Generate JSON documentation"
    echo "7) Clean documentation"
    echo "8) Exit"
    echo ""
}

# Function to generate basic docs
generate_docs() {
    echo -e "${BLUE}Generating documentation...${NC}"
    cargo doc --no-deps --all-features
    echo -e "${GREEN}✓ Documentation generated at: target/doc${NC}"
}

# Function to generate and open docs
generate_and_open() {
    echo -e "${BLUE}Generating and opening documentation...${NC}"
    cargo doc --no-deps --all-features --open
}

# Function to generate docs with private items
generate_private() {
    echo -e "${BLUE}Generating documentation with private items...${NC}"
    cargo doc --no-deps --all-features --document-private-items
    echo -e "${GREEN}✓ Documentation with private items generated${NC}"
}

# Function to check for broken links
check_links() {
    echo -e "${BLUE}Checking for broken intra-doc links...${NC}"
    if cargo rustdoc --all-features -- -D warnings; then
        echo -e "${GREEN}✓ No broken links found${NC}"
    else
        echo -e "${RED}✗ Found broken documentation links${NC}"
        exit 1
    fi
}

# Function to run doc tests
run_doc_tests() {
    echo -e "${BLUE}Running documentation tests...${NC}"
    if cargo test --doc; then
        echo -e "${GREEN}✓ All doc tests passed${NC}"
    else
        echo -e "${RED}✗ Some doc tests failed${NC}"
        exit 1
    fi
}

# Function to generate JSON docs
generate_json() {
    echo -e "${BLUE}Generating JSON documentation...${NC}"
    cargo +nightly doc --no-deps --all-features -Z unstable-options --output-format json
    echo -e "${GREEN}✓ JSON documentation generated${NC}"
}

# Function to clean docs
clean_docs() {
    echo -e "${BLUE}Cleaning documentation...${NC}"
    cargo clean --doc
    echo -e "${GREEN}✓ Documentation cleaned${NC}"
}

# Main loop
while true; do
    show_menu
    read -r -p "Enter choice [1-8]: " choice
    echo ""

    case $choice in
        1)
            generate_docs
            ;;
        2)
            generate_and_open
            ;;
        3)
            generate_private
            ;;
        4)
            check_links
            ;;
        5)
            run_doc_tests
            ;;
        6)
            generate_json
            ;;
        7)
            clean_docs
            ;;
        8)
            echo -e "${GREEN}Goodbye!${NC}"
            exit 0
            ;;
        *)
            echo -e "${RED}Invalid option${NC}"
            ;;
    esac

    echo ""
    read -r -p "Press Enter to continue..."
    echo ""
done
