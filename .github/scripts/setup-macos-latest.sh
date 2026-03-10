#!/usr/bin/env bash
set -euo pipefail

brew install sqlite

# Homebrew doesn't link sqlite into default paths on macOS (conflicts with system SQLite).
# Export paths so libsqlite3-sys (used by sqlx sqlite-unbundled) finds the brew version.
SQLITE_PREFIX="$(brew --prefix sqlite)"
echo "PKG_CONFIG_PATH=${SQLITE_PREFIX}/lib/pkgconfig" >> "$GITHUB_ENV"
echo "SQLITE3_LIB_DIR=${SQLITE_PREFIX}/lib" >> "$GITHUB_ENV"
echo "SQLITE3_INCLUDE_DIR=${SQLITE_PREFIX}/include" >> "$GITHUB_ENV"
