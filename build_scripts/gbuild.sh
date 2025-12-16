#!/bin/sh
set -e

# Version flag
if [ "$1" = "-v" ] || [ "$1" = "--version" ]; then
  gleam --version
  exit 0
fi

FILE="${1:-main.gleam}"

if [ ! -f "$FILE" ]; then
  echo "Error: $FILE not found"
  exit 1
fi

# Create project structure
mkdir -p src

# Write gleam.toml (hardcoded template)
cat > gleam.toml <<'EOF'
name = "cp_gleam"
version = "0.1.0"

[dependencies]
bidict = "1.0.0"
bigdecimal = "1.1.0"
bitsandbobs = "1.1.0"
dijkstra = "1.0.0"
gleam_community_maths = "2.0.0"
gleam_deque = "1.0.0"
gleam_regexp = "1.1.1"
gleam_stdlib = "0.63.0"
gleam_yielder = "1.1.0"
gleamy_structures = "1.2.0"
glearray = "2.1.0"
ieee_float = "1.5.0"
iv = "1.3.2"
stdin = "2.0.2"
trie_again = "1.1.3"
EOF

# Move user code
mv "$FILE" src/main.gleam

# Build
gleam build --no-print-progress
