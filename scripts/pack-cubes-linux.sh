#!/usr/bin/env bash
# Build ship-ready cubes binary for Telegram / multi-GPU testing (Linux x86_64).
# Digital hygiene: no host home / workspace paths in the binary or zip text files.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"   # modul-project/
OUT_DIR="${ROOT}/dist/cubes-linux-x64"
STAMP="$(date -u +%Y%m%d)"
ZIP_NAME="cubes-linux-x64-${STAMP}.zip"

cd "$ROOT"

# Remap source paths so panic/location strings never embed /home/<user>/...
# Longer prefixes first. Generic anchors only — nothing identifying the builder.
CARGO_HOME_R="${CARGO_HOME:-${HOME}/.cargo}"
RUSTUP_HOME_R="${RUSTUP_HOME:-${HOME}/.rustup}"
# Resolve to real paths so remaps match rustc's absolute file! strings.
CARGO_HOME_R="$(cd "$CARGO_HOME_R" 2>/dev/null && pwd || echo "$CARGO_HOME_R")"
RUSTUP_HOME_R="$(cd "$RUSTUP_HOME_R" 2>/dev/null && pwd || echo "$RUSTUP_HOME_R")"
HOME_R="$(cd "$HOME" 2>/dev/null && pwd || echo "$HOME")"
ROOT_R="$(pwd)"

# rustc: later remap flags win when several prefixes match → put longest last.
REMAP_FLAGS=(
  "--remap-path-prefix=${HOME_R}/=/user/"
  "--remap-path-prefix=${ROOT_R}/=/src/"
  "--remap-path-prefix=${RUSTUP_HOME_R}/=/rustup/"
  "--remap-path-prefix=${CARGO_HOME_R}/=/cargo/"
)
export RUSTFLAGS="${REMAP_FLAGS[*]}${RUSTFLAGS:+ ${RUSTFLAGS}}"

echo "==> cargo build -p cubes-auto --release (path-remapped)"
cargo build -p cubes-auto --release

BIN="$ROOT/target/release/cubes-auto"
test -x "$BIN"

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"
cp -a "$BIN" "$OUT_DIR/cubes"
# Drop symbols / leftover debug sections (best-effort).
strip --strip-all "$OUT_DIR/cubes" 2>/dev/null || strip "$OUT_DIR/cubes" 2>/dev/null || true
chmod +x "$OUT_DIR/cubes"

# Fail pack if personal host paths still appear in the binary.
if strings "$OUT_DIR/cubes" | grep -E -q "${HOME_R}|${ROOT_R}|/home/[^/]+/"; then
  echo "ERROR: host path still present in cubes binary (remap failed)" >&2
  strings "$OUT_DIR/cubes" | grep -E "${HOME_R}|${ROOT_R}|/home/[^/]+/" | head -20 >&2
  exit 1
fi

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
    # optional (writes VK spec messages into cubes_vk_validation.txt):
    #   sudo apt install vulkan-validationlayers
    # NVIDIA proprietary (if you use the blob driver):
    #   sudo apt install nvidia-driver-XXX
  Fedora:
    sudo dnf install mesa-vulkan-drivers vulkan-tools
    # optional: vulkan-validation-layers
  Check:
    vulkaninfo --summary

RUN
  ./cubes
  # fewer cubes if the machine is weak:
  CUBES_COUNT=10000 ./cubes

  # Wayland crashes / "wp_linux_drm_syncobj" / one frame then exit:
  WAYLAND_DISPLAY= ./cubes          # force X11 via XWayland
  # or try (if your stack honours them):
  DISABLE_EXPLICIT_SYNC=1 ./cubes
  NW_EXPLICIT_SYNC=0 ./cubes

CONTROLS
  LMB drag  — orbit
  Wheel     — zoom
  Esc       — quit
  Window title shows FPS + process CPU% (MAILBOX present when GPU allows it, else FIFO)

LOG FILES (created next to ./cubes on every run)
  cubes_session_log.txt   — timeline: GPU, path, FPS (~2/s), CPU process/system %, errors
  cubes_vk_validation.txt — Khronos validation ERROR/WARNING/INFO (when layers install)

  Please send BOTH txt files + GPU model + driver if something fails.

SHIP NOTES
  • Binary loads libvulkan at runtime (starts even if missing → clear error).
  • No validation by default (FPS). Opt-in: CUBES_VALIDATION=1 (needs vulkan-validationlayers).
  • Resize / OUT_OF_DATE recreates swapchain+FB only (mesh/device kept — no OOM thrash).
  • MSAA 4× when supported, else 1×; depth D32→D24→D16.
  • Please report: GPU name, FPS + CPU% from title/session log, or the two txt files.

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

# Host-agnostic build notes only (no absolute builder paths).
(
  cd "$OUT_DIR"
  {
    echo "# cubes ship build info (no host paths)"
    echo "target=x86_64-unknown-linux-gnu"
    echo "stamp=${STAMP}"
    echo
    echo "# file (basename only)"
    file -b ./cubes || true
    echo
    echo "# linked libraries (sonames)"
    ldd ./cubes 2>/dev/null | awk '{print $1}' | sort -u || true
  } > BUILD_INFO.txt
)

mkdir -p "$ROOT/dist"
( cd "$ROOT/dist" && rm -f "$ZIP_NAME" && zip -r "$ZIP_NAME" "cubes-linux-x64" )
echo "==> packed: dist/${ZIP_NAME}"
echo "    folder: dist/cubes-linux-x64"
ls -la "$OUT_DIR" "$ROOT/dist/$ZIP_NAME"
