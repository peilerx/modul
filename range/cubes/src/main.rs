//! **modul/range/cubes** — ship-friendly Vulkan etalon (1M instanced cubes).
//!
//! ```bash
//! cargo run -p cubes --release
//! # pack for Telegram:
//! bash modul/scripts/pack-cubes-linux.sh
//! ```

#![allow(
    non_snake_case,
    reason = "MODUL0_* CAPS segments per Factory Mind"
)]

mod shell;
mod tandem;

fn main() {
    // Always print something so a double-clicked terminal / Telegram tester sees life.
    eprintln!("modul/range/cubes · ship · 1_000_000 cubes default (CUBES_COUNT overrides)");
    eprintln!("Controls: LMB orbit · wheel zoom · Esc quit · FPS in window title");
    shell::run();
    // If run returned without a window path, leave a moment for logs on Windows/Telegram.
    eprintln!("cubes: exit");
}
