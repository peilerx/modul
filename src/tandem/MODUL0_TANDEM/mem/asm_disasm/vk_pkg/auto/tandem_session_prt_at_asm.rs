//! PortMatch Auto · `TandemSessionPrt` → `TandemSessionStpPkg` (base ¬ impl).

use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::prt::display_present_prt::DisplayPresentPrt;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::prt::frame_fif_prt::FrameFifPrt;
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::prt::mesh_draw_prt::MeshDrawPrt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::prt::{
    SwapchainAssemblyPrt, SwapchainPrt,
};
use crate::tandem::MODUL0_TANDEM::mem::base::transport::prt::sample_count_prefer_prt::SampleCountPreferPrt;
use crate::tandem::MODUL0_TANDEM::mem::base::transport::prt::tandem_session_prt::TandemSessionPrt;
use crate::tandem::MODUL0_TANDEM::mem::base::transport::prt::validation_prefer_prt::ValidationPreferPrt;
use crate::tandem::MODUL0_TANDEM::mem::base::transport::setup::tandem_session_stp_pkg::TandemSessionStpPkg;

/// Expand aggregate arm into full session setup knobs.
#[must_use]
pub const fn tandem_session_prt_to_stp(prt: TandemSessionPrt) -> TandemSessionStpPkg {
    match prt {
        TandemSessionPrt::SHIP_MAILBOX_AA4_NO_VALIDATION => TandemSessionStpPkg {
            validation_prefer_op: ValidationPreferPrt::NO_VALIDATION,
            present_prt_op: SwapchainPrt::SRGB_MAILBOX,
            sample_prefer_op: SampleCountPreferPrt::PREFER_4_ELSE_1,
            frame_fif_prt_op: FrameFifPrt::DOUBLE_BUFFERED,
            mesh_draw_prt_op: MeshDrawPrt::SOLID,
            display_present_prt_op: DisplayPresentPrt::DEFAULT_PRESENT,
            render_lane_prt_op: None,
            cube_count_stp: 800_000_000,
            lattice_spacing_stp: 1.0,
            clear_color_rt: [0.78, 0.78, 0.80, 1.0],
            pulse_period_secs_stp: 12.0,
            sep_max_stp: 0.0,
            orbit_yaw_stp: 0.6,
            orbit_pitch_stp: 0.4,
            zoom_stp: 1.0,
            camera_radius_scale_stp: 2.8,
            desc: "tandem_session_ship_mailbox_aa4_no_validation",
        },
        TandemSessionPrt::SHIP_MAILBOX_AA4_PREFER_VALIDATION => TandemSessionStpPkg {
            validation_prefer_op: ValidationPreferPrt::PREFER_VALIDATION,
            present_prt_op: SwapchainPrt::SRGB_MAILBOX,
            sample_prefer_op: SampleCountPreferPrt::PREFER_4_ELSE_1,
            frame_fif_prt_op: FrameFifPrt::DOUBLE_BUFFERED,
            mesh_draw_prt_op: MeshDrawPrt::SOLID,
            display_present_prt_op: DisplayPresentPrt::DEFAULT_PRESENT,
            render_lane_prt_op: None,
            cube_count_stp: 800_000_000,
            lattice_spacing_stp: 1.0,
            clear_color_rt: [0.78, 0.78, 0.80, 1.0],
            pulse_period_secs_stp: 12.0,
            sep_max_stp: 0.0,
            orbit_yaw_stp: 0.6,
            orbit_pitch_stp: 0.4,
            zoom_stp: 1.0,
            camera_radius_scale_stp: 2.8,
            desc: "tandem_session_ship_mailbox_aa4_prefer_validation",
        },
        TandemSessionPrt::DEV_FIFO_AA1_PREFER_VALIDATION => TandemSessionStpPkg {
            validation_prefer_op: ValidationPreferPrt::PREFER_VALIDATION,
            present_prt_op: SwapchainPrt::SRGB_MAILBOX,
            sample_prefer_op: SampleCountPreferPrt::FORCE_1,
            frame_fif_prt_op: FrameFifPrt::DOUBLE_BUFFERED,
            mesh_draw_prt_op: MeshDrawPrt::SOLID,
            display_present_prt_op: DisplayPresentPrt::DEFAULT_PRESENT,
            render_lane_prt_op: None,
            cube_count_stp: 100_000,
            lattice_spacing_stp: 1.25,
            clear_color_rt: [0.05, 0.05, 0.08, 1.0],
            pulse_period_secs_stp: 12.0,
            sep_max_stp: 1.6,
            orbit_yaw_stp: 0.6,
            orbit_pitch_stp: 0.4,
            zoom_stp: 1.0,
            camera_radius_scale_stp: 2.8,
            desc: "tandem_session_dev_fifo_aa1_prefer_validation",
        },
        TandemSessionPrt::BENCHMARK_MAILBOX_AA1_NO_VALIDATION => TandemSessionStpPkg {
            validation_prefer_op: ValidationPreferPrt::NO_VALIDATION,
            present_prt_op: SwapchainPrt::SRGB_MAILBOX,
            sample_prefer_op: SampleCountPreferPrt::FORCE_1,
            frame_fif_prt_op: FrameFifPrt::DOUBLE_BUFFERED,
            mesh_draw_prt_op: MeshDrawPrt::SOLID,
            display_present_prt_op: DisplayPresentPrt::DEFAULT_PRESENT,
            render_lane_prt_op: None,
            cube_count_stp: 200_000_000,
            lattice_spacing_stp: 1.25,
            clear_color_rt: [0.02, 0.02, 0.03, 1.0],
            pulse_period_secs_stp: 12.0,
            sep_max_stp: 1.6,
            orbit_yaw_stp: 0.6,
            orbit_pitch_stp: 0.4,
            zoom_stp: 1.0,
            camera_radius_scale_stp: 2.8,
            desc: "tandem_session_benchmark_mailbox_aa1_no_validation",
        },
        TandemSessionPrt::LOW_END_FIFO_AA1_NO_VALIDATION => TandemSessionStpPkg {
            validation_prefer_op: ValidationPreferPrt::NO_VALIDATION,
            present_prt_op: SwapchainPrt::SRGB_MAILBOX,
            sample_prefer_op: SampleCountPreferPrt::FORCE_1,
            frame_fif_prt_op: FrameFifPrt::DOUBLE_BUFFERED,
            mesh_draw_prt_op: MeshDrawPrt::SOLID,
            display_present_prt_op: DisplayPresentPrt::DEFAULT_PRESENT,
            render_lane_prt_op: None,
            cube_count_stp: 50_000,
            lattice_spacing_stp: 1.25,
            clear_color_rt: [0.05, 0.05, 0.08, 1.0],
            pulse_period_secs_stp: 12.0,
            sep_max_stp: 1.6,
            orbit_yaw_stp: 0.6,
            orbit_pitch_stp: 0.4,
            zoom_stp: 1.0,
            camera_radius_scale_stp: 2.8,
            desc: "tandem_session_low_end_fifo_aa1_no_validation",
        },
    }
}

/// Map arm to swapchain assembly intent.
#[must_use]
pub const fn tandem_session_prt_assembly(prt: TandemSessionPrt) -> SwapchainAssemblyPrt {
    match prt {
        TandemSessionPrt::SHIP_MAILBOX_AA4_PREFER_VALIDATION
        | TandemSessionPrt::DEV_FIFO_AA1_PREFER_VALIDATION => {
            SwapchainAssemblyPrt::GRAPHICS_PRESENT_VALIDATION
        }
        TandemSessionPrt::SHIP_MAILBOX_AA4_NO_VALIDATION
        | TandemSessionPrt::BENCHMARK_MAILBOX_AA1_NO_VALIDATION
        | TandemSessionPrt::LOW_END_FIFO_AA1_NO_VALIDATION => {
            SwapchainAssemblyPrt::GRAPHICS_PRESENT_NO_VALIDATION
        }
    }
}

fn parse_cube_count(raw: &str) -> Option<usize> {
    let cleaned: String = raw.chars().filter(char::is_ascii_digit).collect();
    if cleaned.is_empty() {
        return None;
    }
    cleaned.parse::<usize>().ok().filter(|&n| n > 0)
}

/// Apply ship overrides (`CUBES_COUNT`, argv `--count`/`-n`/bare integer, `CUBES_VALIDATION`).
///
/// Last source wins: portrait default → `CUBES_COUNT` → argv.
#[must_use]
pub fn tandem_session_stp_ship_env(mut stp: TandemSessionStpPkg) -> TandemSessionStpPkg {
    if let Ok(s) = std::env::var("CUBES_COUNT") {
        if let Some(n) = parse_cube_count(&s) {
            stp.cube_count_stp = n;
        }
    }
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        if a == "--count" || a == "-n" {
            if let Some(v) = args.get(i + 1) {
                if let Some(n) = parse_cube_count(v) {
                    stp.cube_count_stp = n;
                }
            }
            i += 2;
            continue;
        }
        if let Some(n) = parse_cube_count(a) {
            stp.cube_count_stp = n;
        }
        i += 1;
    }
    if std::env::var("CUBES_VALIDATION")
        .is_ok_and(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
    {
        stp.validation_prefer_op = ValidationPreferPrt::PREFER_VALIDATION;
    }
    stp
}

/// Full replace merge from override bag.
#[must_use]
pub fn tandem_session_stp_merge_override(
    mut stp: TandemSessionStpPkg,
    o: &TandemSessionStpPkg,
) -> TandemSessionStpPkg {
    stp.validation_prefer_op = o.validation_prefer_op;
    stp.present_prt_op = o.present_prt_op;
    stp.sample_prefer_op = o.sample_prefer_op;
    stp.frame_fif_prt_op = o.frame_fif_prt_op;
    stp.mesh_draw_prt_op = o.mesh_draw_prt_op;
    stp.display_present_prt_op = o.display_present_prt_op;
    stp.render_lane_prt_op = o.render_lane_prt_op;
    stp.cube_count_stp = o.cube_count_stp.max(1);
    stp.lattice_spacing_stp = o.lattice_spacing_stp;
    stp.clear_color_rt = o.clear_color_rt;
    stp.pulse_period_secs_stp = o.pulse_period_secs_stp;
    stp.sep_max_stp = o.sep_max_stp;
    stp.orbit_yaw_stp = o.orbit_yaw_stp;
    stp.orbit_pitch_stp = o.orbit_pitch_stp;
    stp.zoom_stp = o.zoom_stp;
    stp.camera_radius_scale_stp = o.camera_radius_scale_stp;
    stp.desc = o.desc;
    stp
}

/// Default session knobs = ship arm expand.
#[must_use]
pub const fn tandem_session_stp_default() -> TandemSessionStpPkg {
    tandem_session_prt_to_stp(TandemSessionPrt::SHIP_MAILBOX_AA4_NO_VALIDATION)
}
