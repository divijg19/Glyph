#!/usr/bin/env bash
set -euo pipefail

# Build the glyph binary and run the example
cargo build --manifest-path "$(dirname "$0")/../Cargo.toml" --bin glyph
REPO_ROOT="$(dirname "$0")/../.."
GLYPH_MANIFEST="$(dirname "$0")/../Cargo.toml"
EXAMPLE="$REPO_ROOT/examples/basic/add.gly"

cargo run --manifest-path "$GLYPH_MANIFEST" --bin glyph -- build "$EXAMPLE"
cargo run --manifest-path "$GLYPH_MANIFEST" --bin glyph -- run "$EXAMPLE"
