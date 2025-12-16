#!/bin/sh
set -e

# Version flag
if [ "$1" = "-v" ] || [ "$1" = "--version" ]; then
  lean --version
  exit 0
fi

FILE="${1:-Main.lean}"

if [ ! -f "$FILE" ]; then
  echo "Error: $FILE not found"
  exit 1
fi

# Project structure
mkdir -p .lake/packages

# Hardcoded lakefile.toml
cat > lakefile.toml <<'EOF'
name = "cp_lean"
version = "0.1.0"
default_targets = ["main"]

[[require]]
name = "Regex"
git = "https://github.com/pandaman64/lean-regex"
rev = "v4.22.0"
subDir = "regex"

[[lean_exe]]
name = "main"
root = "Main"
EOF

# Lean toolchain (CRITICAL)
cat > lean-toolchain <<'EOF'
leanprover/lean4:v4.26.0
EOF

# Move user code
mv "$FILE" Main.lean

# Build
lake -q build main

# copy exe
cp .lake/build/bin/main main