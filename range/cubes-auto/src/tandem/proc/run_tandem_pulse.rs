//! `run_tandem_pulse` — one takt · camera → push → record → present.

use std::time::Instant;

use modul::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_crg::auto::frame_default_rt_crg_export::frame_export_asmed_render1;

use modul::gpu::MODUL0_VK_DISPLAY::proc::display::display_frame::record_frame_with_serial;
use modul::gpu::MODUL0_VK_FRAME::proc::processor::frame_tick::{begin_frame, end_frame};
use modul::gpu::MODUL0_VK_MESH::proc::processor::{
    mesh_gpu_center_rt, mesh_gpu_radius_rt, mesh_push_from_orbit,
};
use modul::gpu::MODUL0_VK_SWAPCHAIN::conv::port::{SwapchainBfr, SwapchainTransportable};
use modul::tandem::MODUL0_TANDEM::TandemBfr;

/// One discrete unit of product pulse (direct cubes).
pub fn run_tandem_pulse(bfr: &mut TandemBfr) -> Result<(), String> {
    let extent = bfr.presentation_rt.swapchain_default_rt_pkg.extent_rt;
    let aspect = if extent.height > 0 {
        extent.width as f32 / extent.height as f32
    } else {
        1.333
    };
    let t = bfr.pulse_t0.elapsed().as_secs_f32();
    let sep_max = bfr.session_stp.sep_max_stp;
    let pulse_period = bfr.session_stp.pulse_period_secs_stp;
    let painting = bfr.heat_painting;
    let ew = extent.width.max(1) as f32;
    let eh = extent.height.max(1) as f32;
    bfr.display_rt.heat_mouse_x_rt = (bfr.cursor_px.0 / ew) * 2.0 - 1.0;
    bfr.display_rt.heat_mouse_y_rt = (bfr.cursor_px.1 / eh) * 2.0 - 1.0;
    bfr.display_rt.heat_paint_rt = u32::from(painting);
    bfr.display_rt.heat_run_rt = u32::from(painting);

    let radius = mesh_gpu_radius_rt(&bfr.mesh_gpu_rt) * 2.8 / bfr.zoom.max(0.08);
    bfr.mesh_push_rt = mesh_push_from_orbit(
        mesh_gpu_center_rt(&bfr.mesh_gpu_rt),
        radius,
        bfr.orbit_yaw,
        bfr.orbit_pitch,
        aspect,
        [
            bfr.mesh_gpu_rt.base_r_rt,
            bfr.mesh_gpu_rt.base_g_rt,
            bfr.mesh_gpu_rt.base_b_rt,
        ],
    );
    let y_half = (0.5
        * (bfr.mesh_gpu_rt.bounds_max_rt[1] - bfr.mesh_gpu_rt.bounds_min_rt[1]))
    .max(0.5);
    bfr.mesh_push_rt.look3[0] = t;
    bfr.mesh_push_rt.look3[1] = sep_max;
    bfr.mesh_push_rt.look3[2] = y_half;
    bfr.mesh_push_rt.look3[3] = pulse_period;
    bfr.frame_rt.frame_render_default_rt_pkg.clear_color_rt = bfr.session_stp.clear_color_rt;

    let swapchain_rt_crg = SwapchainBfr::export_asmed1(&bfr.swapchain_bfr)
        .ok_or_else(|| "run_tandem_pulse: swapchain_rt_crg missing".to_string())?;
    let device = &swapchain_rt_crg.device_default_rt_pkg;
    let loader = &swapchain_rt_crg.swapchain_loader_default_rt_pkg;

    let (slot, image_index) =
        begin_frame(device, &bfr.presentation_rt, loader, &bfr.frame_rt)?;

    // dt AFTER the FIF fence: before begin_frame this is only the winit turn (~1e-4 s).
    let dt = bfr
        .last_frame_end
        .elapsed()
        .as_secs_f32()
        .clamp(1e-4, 0.05);
    if painting {
        bfr.heat_hold_rt += dt;
    } else {
        bfr.heat_hold_rt = 0.0;
    }
    bfr.display_rt.heat_dt_rt = dt;
    bfr.display_rt.heat_hold_rt = bfr.heat_hold_rt;

    let render_policy = frame_export_asmed_render1(&bfr.frame_rt);
    record_frame_with_serial(
        device,
        &bfr.presentation_rt,
        &bfr.renderer_rt,
        &slot,
        render_policy,
        true,
        Some(&bfr.mesh_gpu_rt),
        Some(&bfr.mesh_push_rt),
        None,
        None,
        None,
        image_index,
        &mut bfr.display_rt,
    )?;

    end_frame(
        device,
        &bfr.presentation_rt,
        loader,
        &mut bfr.frame_rt,
        &slot,
        image_index,
    )?;

    let now = Instant::now();
    let dt = now
        .duration_since(bfr.last_frame_end)
        .as_secs_f32()
        .max(1e-6);
    bfr.last_frame_end = now;
    bfr.fps_instant = 1.0 / dt;
    bfr.fps_frames = bfr.fps_frames.saturating_add(1);
    let win = now
        .duration_since(bfr.fps_window_start)
        .as_secs_f32();
    if win >= 0.5 {
        bfr.fps = bfr.fps_frames as f32 / win;
        bfr.fps_frames = 0;
        bfr.fps_window_start = now;
        bfr.fps_sample_ready = true;
        crate::tandem::proc::session_log::log(&format!(
            "HEAT io · paint={} hold={:.2} mouse=({:.3},{:.3}) dt={:.4}",
            bfr.display_rt.heat_paint_rt,
            bfr.display_rt.heat_hold_rt,
            bfr.display_rt.heat_mouse_x_rt,
            bfr.display_rt.heat_mouse_y_rt,
            bfr.display_rt.heat_dt_rt
        ));
    }
    Ok(())
}
