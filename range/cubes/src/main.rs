//! **modul/range/cubes** — etalon Vulkan app on **modul** (TANDEM · direct only).
//!
//! ```bash
//! cargo run -p cubes --release
//! ```

#![allow(
    non_snake_case,
    reason = "MODUL0_* CAPS segments per Factory Mind"
)]

mod shell;
mod tandem;

fn main() {
    eprintln!("modul/range/cubes · direct · T.Hub MODUL0_TANDEM");
    shell::run();
}
