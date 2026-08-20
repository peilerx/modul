//! **modul/range/cubes-auto** — AUTO session etalon (`TandemSessionPrt` presets).
//!
//! ```bash
//! cargo run -p cubes-auto --release
//! ```

#![allow(
    non_snake_case,
    reason = "MODUL0_* CAPS segments per Factory Mind"
)]

mod tandem;

fn main() {
    use tandem::proc::session_log;

    let dir = session_log::init();
    session_log::log("modul/range/cubes-auto · AUTO · TandemSessionPrt::SHIP_MAILBOX_AA4_NO_VALIDATION");
    session_log::log("Controls: hold LMB/RMB on metal to melt · wheel zoom · Esc quit");
    session_log::log("Env: CUBES_COUNT · CUBES_GPU=discrete|0|nvidia · CUBES_VALIDATION=1 · argv --count N");
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
    session_log::log("cubes-auto: exit");
}
