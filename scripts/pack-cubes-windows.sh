#!/usr/bin/env bash
# Build ship-ready cubes-auto.exe for Telegram / multi-GPU testing (Windows x86_64 MSVC).
# Cross-compile from Linux via cargo-xwin. Same logging/recreate as Linux ship.
# Digital hygiene: no host home / workspace paths in the binary or zip text files.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"   # modul-project/
OUT_DIR="${ROOT}/dist/cubes-windows-x64"
STAMP="$(date -u +%Y%m%d)"
ZIP_NAME="cubes-windows-x64-${STAMP}.zip"
TARGET="x86_64-pc-windows-msvc"

cd "$ROOT"

if ! command -v cargo-xwin >/dev/null 2>&1 && ! cargo xwin --version >/dev/null 2>&1; then
  echo "ERROR: cargo-xwin required (cargo install cargo-xwin)" >&2
  exit 1
fi

# Remap source paths (later flags win when prefixes overlap).
CARGO_HOME_R="${CARGO_HOME:-${HOME}/.cargo}"
RUSTUP_HOME_R="${RUSTUP_HOME:-${HOME}/.rustup}"
CARGO_HOME_R="$(cd "$CARGO_HOME_R" 2>/dev/null && pwd || echo "$CARGO_HOME_R")"
RUSTUP_HOME_R="$(cd "$RUSTUP_HOME_R" 2>/dev/null && pwd || echo "$RUSTUP_HOME_R")"
HOME_R="$(cd "$HOME" 2>/dev/null && pwd || echo "$HOME")"
ROOT_R="$(pwd)"

REMAP_FLAGS=(
  "--remap-path-prefix=${HOME_R}/=/user/"
  "--remap-path-prefix=${ROOT_R}/=/src/"
  "--remap-path-prefix=${RUSTUP_HOME_R}/=/rustup/"
  "--remap-path-prefix=${CARGO_HOME_R}/=/cargo/"
)
export RUSTFLAGS="${REMAP_FLAGS[*]}${RUSTFLAGS:+ ${RUSTFLAGS}}"

# clang-cl + lld-link (user-local extract; no system install required)
LLVM_BIN="${LLVM_BIN:-$HOME/.local/opt/llvm-19/usr/lib/llvm-19/bin}"
if [[ -x "$LLVM_BIN/clang" && -x "$LLVM_BIN/lld" ]]; then
  export PATH="$LLVM_BIN:$PATH"
  # cargo-xwin expects clang-cl / lld-link names
  [[ -e "$LLVM_BIN/clang-cl" ]] || ln -sfn clang "$LLVM_BIN/clang-cl"
  [[ -e "$LLVM_BIN/lld-link" ]] || ln -sfn lld "$LLVM_BIN/lld-link"
  echo "==> using LLVM from $LLVM_BIN"
else
  echo "WARN: no clang in $LLVM_BIN — cargo-xwin may fail to link" >&2
fi

# Only x86_64 CRT/SDK (default also pulls aarch64 = 2× download).
# Skip re-download when DONE marker exists (written by cargo-xwin after first success,
# or pre-seeded if CRT was splat manually).
XWIN_CACHE="${XWIN_CACHE_DIR:-$HOME/.cache/cargo-xwin}/xwin"
if [[ -f "$XWIN_CACHE/crt/lib/x86_64/libcmt.lib" && ! -f "$XWIN_CACHE/DONE" ]]; then
  echo "x86_64" > "$XWIN_CACHE/DONE"
  echo "==> seeded xwin DONE marker (x86_64 CRT already present)"
fi

echo "==> cargo xwin build -p cubes-auto --release --target ${TARGET} --xwin-arch x86_64"
cargo xwin build -p cubes-auto --release --target "${TARGET}" --xwin-arch x86_64

BIN="$ROOT/target/${TARGET}/release/cubes-auto-auto.exe"
test -f "$BIN"

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"
cp -a "$BIN" "$OUT_DIR/cubes-auto.exe"

# Fail pack if personal host paths still appear.
if command -v strings >/dev/null 2>&1; then
  if strings "$OUT_DIR/cubes-auto.exe" | grep -E -q "${HOME_R}|${ROOT_R}|/home/[^/]+/"; then
    echo "ERROR: host path still present in cubes-auto.exe (remap failed)" >&2
    strings "$OUT_DIR/cubes-auto.exe" | grep -E "${HOME_R}|${ROOT_R}|/home/[^/]+/" | head -20 >&2
    exit 1
  fi
fi

cat > "$OUT_DIR/RUN.txt" <<'EOF'
modul / cubes — ship demo (1 000 000 instanced cubes)
======================================================

WHAT YOU NEED
  • Windows 10/11 x64
  • GPU with a working Vulkan driver (AMD / NVIDIA / Intel)
  • Desktop session (not pure RDP without GPU in some setups)

INSTALL VULKAN
  • Install GPU vendor driver (GeForce / Adrenalin / Intel Arc+).
  • Optional (VK validation → cubes_vk_validation.txt):
      Vulkan SDK from https://vulkan.lunarg.com/
  • Check:  vulkaninfo (if SDK/tools installed)

RUN
  Double-click cubes-auto.exe
  or in PowerShell / cmd:
    .\cubes-auto.exe
    set CUBES_COUNT=10000
    .\cubes-auto.exe

CONTROLS
  LMB drag  — orbit
  Wheel     — zoom
  Esc       — quit
  Title bar shows FPS + process CPU%

LOG FILES (created next to cubes-auto.exe on every run)
  cubes_session_log.txt   — GPU, path, FPS (~2/s), CPU process/system %, errors
  cubes_vk_validation.txt — Khronos validation (when Vulkan SDK layers installed)

  Please send BOTH txt files + GPU model + driver if something fails.

SHIP NOTES
  • Binary loads vulkan-1.dll at runtime (clear error if missing).
  • No validation by default (FPS). Opt-in: set CUBES_VALIDATION=1 (Vulkan SDK layers).
  • Resize / OUT_OF_DATE recreates swapchain+FB only (mesh/device kept).
  • MSAA 4× when supported, else 1×; depth D32→D24→D16.
  • Please report: GPU name, FPS + CPU% from title/session log, or the two txt files.

Project: https://github.com/peilerx/modul
EOF

(
  cd "$OUT_DIR"
  {
    echo "# cubes ship build info (no host paths)"
    echo "target=${TARGET}"
    echo "stamp=${STAMP}"
    echo "host=cross-linux-xwin"
    echo
    echo "# file"
    file -b ./cubes-auto.exe 2>/dev/null || echo "PE32+ executable (Windows x86-64)"
  } > BUILD_INFO.txt
)

mkdir -p "$ROOT/dist"
( cd "$ROOT/dist" && rm -f "$ZIP_NAME" && zip -r "$ZIP_NAME" "cubes-windows-x64" )
echo "==> packed: dist/${ZIP_NAME}"
echo "    folder: dist/cubes-windows-x64"
ls -la "$OUT_DIR" "$ROOT/dist/$ZIP_NAME"
