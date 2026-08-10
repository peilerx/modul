#!/usr/bin/env bash
# Build modul rustdoc HTML with IBM Plex Sans header.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WS="$(cd "$ROOT/.." && pwd)"
HEADER="$ROOT/docs/rustdoc-header.html"

export RUSTDOCFLAGS="--html-in-header ${HEADER}"

cd "$WS"
cargo doc -p modul --no-deps --document-private-items

echo "Open: $WS/target/doc/modul/index.html"
echo "Or:   http://127.0.0.1:8765/modul/index.html (if http.server is running)"
