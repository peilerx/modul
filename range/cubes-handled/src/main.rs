//! **modul/range/cubes-handled** — Handled mem path only.
//!
//! `main` owns raw *Prt/*Stp knobs. Boot: `assemble_tandem_session`
//! (`mem/asm_disasm/**/handled` · `handled_assemble`). No Auto catalogs.
//!
//! ```bash
//! cargo run -p cubes-handled --release
//! ```

#![allow(
    non_snake_case,
    reason = "MODUL0_* CAPS segments per Factory Mind"
)]

mod tandem;

use modul::gpu::MODUL0_VK_DISPLAY::mem::base::transport::prt::DisplayPresentPrt;
use modul::gpu::MODUL0_VK_FRAME::mem::base::transport::prt::FrameFifPrt;
use modul::gpu::MODUL0_VK_MESH::mem::base::transport::prt::MeshDrawPrt;
use modul::gpu::MODUL0_VK_PIPELINE::mem::base::transport::prt::RenderLanePrt;
use modul::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::prt::SwapchainPrt;
use modul::tandem::MODUL0_TANDEM::{
    SampleCountPreferPrt, TandemSessionStpPkg, ValidationPreferPrt,
};

fn main() {
    use tandem::proc::session_log;

    let _dir = session_log::init();
    session_log::log("modul/range/cubes-handled · HANDLED mem (TandemSessionStpPkg)");
    session_log::log("Controls: LMB orbit · wheel zoom · Esc quit · FPS in window title");
    session_log::log(&format!("session log: {}", session_log::session_path().display()));
    session_log::log(&format!(
        "vk validation log: {}",
        session_log::vk_validation_path().display()
    ));
    session_log::log("--- edit handled_session_stp() · Prt/Stp only · assemble_tandem_session = Handled ---");

    modul::gpu::MODUL0_VK_SWAPCHAIN::proc::processor::debug_messenger::set_vk_validation_log_path(
        &session_log::vk_validation_path(),
    );

    // Raw knobs only; shell moves them into assemble_tandem_session (no clone).
    tandem::run_shell_handled(handled_session_stp());
    session_log::log("cubes-handled: exit");
}

/// Explicit product levers · every field intentional (Handled, not Auto Prt table).
const fn handled_session_stp() -> TandemSessionStpPkg {
    TandemSessionStpPkg {
        validation_prefer_op: ValidationPreferPrt::NO_VALIDATION,
        present_prt_op: SwapchainPrt::SRGB_MAILBOX,
        sample_prefer_op: SampleCountPreferPrt::PREFER_4_ELSE_1,
        frame_fif_prt_op: FrameFifPrt::DOUBLE_BUFFERED,
        mesh_draw_prt_op: MeshDrawPrt::SOLID,
        display_present_prt_op: DisplayPresentPrt::DEFAULT_PRESENT,
        render_lane_prt_op: Some(RenderLanePrt::TRIANGLE_SOLID_DEPTH_AA4),
        cube_count_stp: 1_000_000,
        lattice_spacing_stp: 1.25,
        clear_color_rt: [0.04, 0.05, 0.09, 1.0],
        pulse_period_secs_stp: 12.0,
        sep_max_stp: 1.6,
        orbit_yaw_stp: 0.6,
        orbit_pitch_stp: 0.4,
        zoom_stp: 1.0,
        camera_radius_scale_stp: 2.8,
        desc: "cubes_handled_full_knobs",
    }
}
