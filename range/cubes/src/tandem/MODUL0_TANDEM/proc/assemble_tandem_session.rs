//! `assemble_tandem_session` — boot once · order PTP Slot-Factory-Line ports · direct cubes.
//!
//! Session assembly is **explicit** Assembly-Buffer protocol (not a hidden high-level wrapper):
//! each MCG (`SwapchainBfr`, `RendererBfr`, …) is imported with a named intent (`*Prt`) so
//! present mode, MSAA, FIF, and render lane stay visible. Custom etalons copy this function and
//! change only the ports they need; optional thin presets can call this function, not replace it.

use ash::vk;
use modul::gpu::MODUL0_VK_DISPLAY::conv::port::{DisplayBfr, DisplayBfrAuto, DisplayTransportable};
use modul::gpu::MODUL0_VK_DISPLAY::mem::base::transport::prt::display_present_prt::DisplayPresentPrt;
use modul::gpu::MODUL0_VK_FRAME::conv::port::{FrameBfr, FrameBfrAuto, FrameTransportable};
use modul::gpu::MODUL0_VK_FRAME::mem::base::transport::prt::frame_fif_prt::FrameFifPrt;
use modul::gpu::MODUL0_VK_MESH::conv::port::{MeshGpuBfr, MeshGpuBfrAuto, MeshGpuTransportable};
use modul::gpu::MODUL0_VK_MESH::mem::asm_disasm::vk_pkg::auto::line_gpu_res_intsct_at_asm::LineGpuDefaultAuto;
use modul::gpu::MODUL0_VK_MESH::mem::base::transport::prt::mesh_draw_prt::MeshDrawPrt;
use modul::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::line_gpu_default_rt_pkg::LineGpuDefaultRtPkg;
use modul::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::CadSteelPushRt;
use modul::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_soa_rt_bfr::MeshSoaRtBfr;
use modul::gpu::MODUL0_VK_PIPELINE::conv::port::{
    RendererBfr, RendererBfrAuto, RendererTransportable,
};
use modul::gpu::MODUL0_VK_PIPELINE::mem::base::transport::prt::render_lane_prt::RenderLanePrt;
use modul::gpu::MODUL0_VK_SWAPCHAIN::conv::port::{
    PresentationBfr, PresentationBfrAuto, PresentationTransportable, SwapchainBfr,
    SwapchainBfrAuto, SwapchainTransportable,
};
use modul::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::prt::{
    SwapchainAssemblyPrt, SwapchainPrt,
};
use modul::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::setup::boot_res_intsct_stp_pkgs::SurfaceWindowStpPkg;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::window::Window;

use crate::tandem::MODUL0_TANDEM::mem::tandem_bfr::TandemBfr;

/// Frames-in-flight — must match [`FrameFifPrt::DoubleBuffered`].
const FIF: u32 = 2;
/// Default lattice size (override with `CUBES_COUNT`).
const DEFAULT_CUBE_COUNT: usize = 1_000_000;

fn surface_stp(window: &Window) -> Result<SurfaceWindowStpPkg, String> {
    let display_handle_extrl = window
        .display_handle()
        .map_err(|e| format!("cubes: display handle: {e}"))?
        .as_raw();
    let window_handle_extrl = window
        .window_handle()
        .map_err(|e| format!("cubes: window handle: {e}"))?
        .as_raw();
    Ok(SurfaceWindowStpPkg {
        display_handle_extrl,
        window_handle_extrl,
        desc: "cubes_vk_surface",
    })
}

/// Boot product once · PTP order · fill `TandemBfr` (direct only).
pub fn assemble_tandem_session(window: &Window) -> Result<TandemBfr, String> {
    let size = window.inner_size();
    let w = size.width.max(1);
    let h = size.height.max(1);
    eprintln!("cubes viewport/swapchain · {w}x{h}");
    let surface = surface_stp(window)?;

    // --- MCG: swapchain (instance · device · surface · KHR present) ---
    let mut swapchain_bfr = SwapchainBfr::auto_assemble();
    SwapchainBfr::import_for_asm8(
        &mut swapchain_bfr,
        SwapchainAssemblyPrt::GraphicsPresentNoValidation,
        surface,
    )?;
    SwapchainBfr::import_present_for_asm1(
        &mut swapchain_bfr,
        SwapchainPrt::SrgbMailbox,
        w,
        h,
    )?;
    let swapchain_default_rt_pkg = swapchain_bfr
        .swapchain_default_rt_pkg
        .take()
        .ok_or_else(|| "cubes: KHR product missing".to_string())?;
    let boot = SwapchainBfr::export_asmed1(&swapchain_bfr)
        .ok_or_else(|| "cubes: boot cargo missing".to_string())?;

    // --- MCG: renderer (render pass + steel/line pipelines) ---
    let mut renderer_bfr = RendererBfr::auto_assemble();
    RendererBfr::import_for_asm9(
        &mut renderer_bfr,
        RenderLanePrt::TriangleSolidDepthAa4,
        w,
        h,
        vk::Format::B8G8R8A8_SRGB,
        &boot.device_default_rt_pkg,
    )?;
    let renderer_rt = renderer_bfr
        .cargo_rt
        .take()
        .ok_or_else(|| "cubes: renderer cargo missing".to_string())?;

    // --- MCG: presentation (framebuffers · depth · MSAA resolve) ---
    let mut presentation_bfr = PresentationBfr::auto_assemble();
    PresentationBfr::import_for_asm6(
        &mut presentation_bfr,
        boot,
        &renderer_rt.render_pass_triangle_rt_pkg,
        swapchain_default_rt_pkg,
        vk::SampleCountFlags::TYPE_4,
        vk::Format::D32_SFLOAT,
    )?;
    let presentation_rt = presentation_bfr
        .cargo_rt
        .take()
        .ok_or_else(|| "cubes: presentation cargo missing".to_string())?;

    // --- MCG: frame (FIF · semaphores · fences · command buffers) ---
    let mut frame_bfr = FrameBfr::auto_assemble();
    FrameBfr::import_for_asm3(
        &mut frame_bfr,
        FrameFifPrt::DoubleBuffered,
        &boot.device_default_rt_pkg,
        &boot.swapchain_command_pool_default_rt_pkg,
    )?;
    let frame_rt = frame_bfr
        .cargo_rt
        .take()
        .ok_or_else(|| "cubes: frame cargo missing".to_string())?;

    // --- MCG: display (record-side serial / command bookkeeping) ---
    let mut display_bfr = DisplayBfr::auto_assemble();
    DisplayBfr::import_for_asm5(
        &mut display_bfr,
        DisplayPresentPrt::DefaultPresent,
        FIF,
        &boot.device_default_rt_pkg,
        &boot.swapchain_command_pool_default_rt_pkg,
    )?;
    let display_rt = display_bfr
        .cargo_rt
        .take()
        .ok_or_else(|| "cubes: display cargo missing".to_string())?;

    let dev = &boot.device_default_rt_pkg.device_extrl;
    let inst = &boot.instance_default_rt.instance_extrl;
    let phys = boot.physical_device_default_rt_pkg.physical_device_extrl;

    let cube_count: usize = std::env::var("CUBES_COUNT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_CUBE_COUNT)
        .max(1);
    let mesh = MeshSoaRtBfr::unit_cuboid_instanced_lattice(cube_count, 1.25);

    // --- MCG: mesh (instance lattice · steel solid) ---
    let mut mesh_gpu_bfr = MeshGpuBfr::auto_assemble();
    MeshGpuBfr::import_for_asm1(
        &mut mesh_gpu_bfr,
        MeshDrawPrt::SteelSolid,
        dev,
        inst,
        phys,
        &mesh,
    )?;
    let mesh_gpu_rt = mesh_gpu_bfr
        .mesh_gpu_default_rt_pkg
        .take()
        .ok_or_else(|| "cubes: mesh_gpu missing".to_string())?;

    // Empty line bag — grid peel reserved for custom etalons (LOD overlays, etc.)
    let grid_line_rt =
        LineGpuDefaultRtPkg::auto_assemble(dev, inst, phys, &[], [0.35, 0.37, 0.40, 1.0])?;

    let aspect = w as f32 / h as f32;
    let steel_push_rt = CadSteelPushRt::from_orbit(
        mesh_gpu_rt.center_rt(),
        mesh_gpu_rt.radius_rt() * 2.8,
        0.6,
        0.4,
        aspect,
        [
            mesh_gpu_rt.steel_r_rt,
            mesh_gpu_rt.steel_g_rt,
            mesh_gpu_rt.steel_b_rt,
        ],
    );

    eprintln!(
        "cubes TANDEM ok · direct · n={cube_count} · tris={} · {w}x{h}",
        mesh_gpu_rt.triangle_count_rt
    );

    Ok(TandemBfr {
        swapchain_bfr,
        renderer_rt,
        presentation_rt,
        frame_rt,
        display_rt,
        mesh_gpu_rt,
        grid_line_rt,
        steel_push_rt,
        orbit_yaw: 0.6,
        orbit_pitch: 0.4,
        zoom: 1.0,
        dragging: false,
        last_cursor: None,
        fps: 0.0,
        fps_instant: 0.0,
        fps_sample_ready: false,
        fps_frames: 0,
        fps_window_start: std::time::Instant::now(),
        last_frame_end: std::time::Instant::now(),
        pulse_t0: std::time::Instant::now(),
    })
}
