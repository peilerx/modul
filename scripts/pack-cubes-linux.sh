#!/usr/bin/env bash
# Build ship-ready cubes binary for Telegram / multi-GPU testing (Linux x86_64).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"   # modul-project/
MODUL_CRATE="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="${ROOT}/dist/cubes-linux-x64"
STAMP="$(date -u +%Y%m%d)"
ZIP_NAME="cubes-linux-x64-${STAMP}.zip"

echo "==> workspace: $ROOT"
cd "$ROOT"

echo "==> cargo build -p cubes --release"
cargo build -p cubes --release

BIN="$ROOT/target/release/cubes"
test -x "$BIN"

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"
cp -a "$BIN" "$OUT_DIR/cubes"
strip "$OUT_DIR/cubes" 2>/dev/null || true
chmod +x "$OUT_DIR/cubes"

cat > "$OUT_DIR/RUN.txt" <<'EOF'
modul / cubes — ship demo (1 000 000 instanced cubes)
======================================================

WHAT YOU NEED
  • Linux x86_64
  • GPU with a working Vulkan driver (Mesa / NVIDIA / AMD)
  • Display (X11 or Wayland)

INSTALL VULKAN (if the app says loader not found)
  Debian/Ubuntu:
    sudo apt install mesa-vulkan-drivers vulkan-tools
    # NVIDIA proprietary (if you use the blob driver):
    #   sudo apt install nvidia-driver-XXX
  Fedora:
    sudo dnf install mesa-vulkan-drivers vulkan-tools
  Check:
    vulkaninfo --summary

RUN
  ./cubes
  CUBES_COUNT=100000 cargo run   # not needed in this zip — use env:
  CUBES_COUNT=10000 ./cubes

CONTROLS
  LMB drag  — orbit
  Wheel     — zoom
  Esc       — quit
  Window title shows FPS (MAILBOX present when the GPU allows it, else FIFO)

SHIP NOTES
  • Binary loads libvulkan at runtime (starts even if missing → clear error).
  • No validation layers required.
  • MSAA 4× when supported, else 2×/1×; depth D32→D24→D16.
  • Please report: GPU name, driver, FPS from title, or the error text.

Project: https://github.com/peilerx/modul
EOF

cat > "$OUT_DIR/run.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
export CUBES_COUNT="${CUBES_COUNT:-1000000}"
exec ./cubes "$@"
EOF
chmod +x "$OUT_DIR/run.sh"

# Dependency summary for packagers
{
  echo "# ldd (host build machine)"
  ldd "$OUT_DIR/cubes" || true
  echo
  echo "# file"
  file "$OUT_DIR/cubes" || true
} > "$OUT_DIR/BUILD_INFO.txt"

mkdir -p "$ROOT/dist"
( cd "$ROOT/dist" && rm -f "$ZIP_NAME" && zip -r "$ZIP_NAME" "cubes-linux-x64" )
echo "==> packed: $ROOT/dist/$ZIP_NAME"
echo "    folder: $OUT_DIR"
ls -la "$OUT_DIR" "$ROOT/dist/$ZIP_NAME"
