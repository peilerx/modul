//! `run_tandem_pulse` — one takt · **direct** only (camera → push → record → present).

use std::time::Instant;

use modul::gpu::MODUL0_VK_DISPLAY::proc::display::display_frame::record_frame_with_serial;
use modul::gpu::MODUL0_VK_FRAME::conv::port::export::frame::export_asmed_frame_render;
use modul::gpu::MODUL0_VK_FRAME::proc::processor::frame_tick::{begin_frame, end_frame};
use modul::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::CadSteelPushRt;
use modul::gpu::MODUL0_VK_SWAPCHAIN::conv::port::{SwapchainBfr, SwapchainTransportable};

use crate::tandem::MODUL0_TANDEM::mem::tandem_bfr::TandemBfr;

/// Max radial separation when fully “open” (shader `look3.y`).
const SEP_MAX: f32 = 1.6;
/// Full open/close cycle length in seconds (shader `look3.w`).
/// Was accidentally 0 → shader clamped to 0.5s (too fast). Original etalon used ~7s.
const PULSE_PERIOD_SECS: f32 = 12.0;

/// One discrete unit of product pulse (direct cubes).
pub fn run_tandem_pulse(hub: &mut TandemBfr) -> Result<(), String> {
    let extent = hub.presentation_rt.swapchain_default_rt_pkg.extent_rt;
    let aspect = if extent.height > 0 {
        extent.width as f32 / extent.height as f32
    } else {
        1.333
    };
    let t = hub.pulse_t0.elapsed().as_secs_f32();

    let radius = hub.mesh_gpu_rt.radius_rt() * 2.8 / hub.zoom.max(0.2);
    hub.steel_push_rt = CadSteelPushRt::from_orbit(
        hub.mesh_gpu_rt.center_rt(),
        radius,
        hub.orbit_yaw,
        hub.orbit_pitch,
        aspect,
        [
            hub.mesh_gpu_rt.steel_r_rt,
            hub.mesh_gpu_rt.steel_g_rt,
            hub.mesh_gpu_rt.steel_b_rt,
        ],
    );
    let y_half = (0.5
        * (hub.mesh_gpu_rt.bounds_max_rt[1] - hub.mesh_gpu_rt.bounds_min_rt[1]))
    .max(0.5);
    // look3: x=time, y=sep_max, z=y_half, w=period (shader cubes.vert)
    hub.steel_push_rt.look3[0] = t;
    hub.steel_push_rt.look3[1] = SEP_MAX;
    hub.steel_push_rt.look3[2] = y_half;
    hub.steel_push_rt.look3[3] = PULSE_PERIOD_SECS;
    hub.frame_rt.frame_render_default_rt_pkg.clear_color_rt = [0.05, 0.05, 0.08, 1.0];

    let boot = SwapchainBfr::export_asmed1(&hub.swapchain_bfr)
        .ok_or_else(|| "run_tandem_pulse: boot cargo missing".to_string())?;
    let device = &boot.device_default_rt_pkg;
    let loader = &boot.swapchain_loader_default_rt_pkg;

    let (slot, image_index) =
        begin_frame(device, &hub.presentation_rt, loader, &hub.frame_rt)
            .map_err(|e| e.to_string())?;

    let render_policy = export_asmed_frame_render(&hub.frame_rt);
    record_frame_with_serial(
        device,
        &hub.presentation_rt,
        &hub.renderer_rt,
        &slot,
        &render_policy,
        true,
        Some(&hub.mesh_gpu_rt),
        Some(&hub.steel_push_rt),
        Some(&hub.grid_line_rt),
        None,
        None,
        image_index,
        &mut hub.display_rt,
    )
    .map_err(|e| e.to_string())?;

    end_frame(
        device,
        &hub.presentation_rt,
        loader,
        &mut hub.frame_rt,
        &slot,
        image_index,
    )
    .map_err(|e| e.to_string())?;

    let now = Instant::now();
    let dt = now
        .duration_since(hub.last_frame_end)
        .as_secs_f32()
        .max(1e-6);
    hub.last_frame_end = now;
    hub.fps_instant = 1.0 / dt;
    hub.fps_frames += 1;
    let win = now
        .duration_since(hub.fps_window_start)
        .as_secs_f32();
    if win >= 0.5 {
        hub.fps = hub.fps_frames as f32 / win;
        hub.fps_frames = 0;
        hub.fps_window_start = now;
        hub.fps_sample_ready = true;
    }
    Ok(())
}
