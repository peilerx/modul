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

mod tandem;

fn main() {
    use tandem::proc::session_log;

    let dir = session_log::init();
    session_log::log("modul/range/cubes · ship · 1_000_000 cubes default (CUBES_COUNT overrides)");
    session_log::log("Controls: LMB orbit · wheel zoom · Esc quit · FPS in window title");
    session_log::log(&format!(
        "session log: {}",
        session_log::session_path().display()
    ));
    session_log::log(&format!(
        "vk validation log: {}",
        session_log::vk_validation_path().display()
    ));
    session_log::log(&format!("working folder for logs: {}", dir.display()));

    modul::gpu::MODUL0_VK_SWAPCHAIN::proc::processor::debug_messenger::set_vk_validation_log_path(
        &session_log::vk_validation_path(),
    );

    tandem::run_shell();
    session_log::log("cubes: exit");
}
