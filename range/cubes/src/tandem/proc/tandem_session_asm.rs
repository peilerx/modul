//! `tandem_session_asm` — compose GPU ports from session Prt + Stp knobs.
//!
//! Dual API:
//! - high-level: [`TandemSessionPrt`] aggregate arms  
//! - full knobs: [`TandemSessionStpPkg`] (after Prt expand + env / overrides)

use std::ffi::CStr;

use ash::vk;
use modul::gpu::MODUL0_VK_DISPLAY::conv::port::{DisplayBfr, DisplayBfrAuto, DisplayTransportable};
use modul::gpu::MODUL0_VK_DISPLAY::mem::base::transport::prt::display_present_prt::DisplayPresentPrt;
use modul::gpu::MODUL0_VK_FRAME::conv::port::{FrameBfr, FrameBfrAuto, FrameTransportable};
use modul::gpu::MODUL0_VK_MESH::conv::port::{MeshGpuBfr, MeshGpuBfrAuto, MeshGpuTransportable};
use modul::gpu::MODUL0_VK_MESH::mem::base::transport::prt::mesh_draw_prt::MeshDrawPrt;
use modul::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::MeshPushRt;
use modul::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_soa_rt_bfr::MeshSoaRtBfr;
use modul::gpu::MODUL0_VK_PIPELINE::conv::port::{
    RendererBfr, RendererBfrAuto, RendererTransportable,
};
use modul::gpu::MODUL0_VK_PIPELINE::mem::base::transport::prt::render_lane_prt::RenderLanePrt;
use modul::gpu::MODUL0_VK_SWAPCHAIN::conv::port::{
    PresentationBfr, PresentationBfrAuto, PresentationTransportable, SwapchainBfr,
    SwapchainBfrAuto, SwapchainTransportable,
};
use modul::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::prt::SwapchainAssemblyPrt;
use modul::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::setup::boot_res_intsct_stp_pkgs::SurfaceWindowStpPkg;
use modul::gpu::MODUL0_VK_SWAPCHAIN::proc::processor::device_caps::{
    pick_depth_format, pick_sample_count_prefer, SampleCountPrefer,
};
use modul::tandem::MODUL0_TANDEM::mem::base::transport::prt::{
    SampleCountPreferPrt, TandemSessionPrt, ValidationPreferPrt,
};
use modul::tandem::MODUL0_TANDEM::mem::base::transport::setup::TandemSessionStpPkg;
use modul::tandem::MODUL0_TANDEM::TandemBfr;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::window::Window;

use crate::tandem::proc::session_log;

fn surface_from_window(window: &Window) -> Result<SurfaceWindowStpPkg, String> {
    let display = window
        .display_handle()
        .map_err(|e| format!("cubes: display handle: {e}"))?
        .as_raw();
    let window_h = window
        .window_handle()
        .map_err(|e| format!("cubes: window handle: {e}"))?
        .as_raw();
    Ok(SurfaceWindowStpPkg::from_raw(
        display,
        window_h,
        "cubes_vk_surface",
    ))
}

const fn map_sample_prefer(p: SampleCountPreferPrt) -> SampleCountPrefer {
    match p {
        SampleCountPreferPrt::Prefer4Else1 => SampleCountPrefer::Prefer4Else1,
        SampleCountPreferPrt::Force1 => SampleCountPrefer::Force1,
        SampleCountPreferPrt::Prefer8Else4Else1 => SampleCountPrefer::Prefer8Else4Else1,
    }
}

const fn frames_in_flight_u32(
    fif: modul::gpu::MODUL0_VK_FRAME::mem::base::transport::prt::frame_fif_prt::FrameFifPrt,
) -> u32 {
    use modul::gpu::MODUL0_VK_FRAME::mem::base::transport::prt::frame_fif_prt::FrameFifPrt;
    match fif {
        FrameFifPrt::TripleBuffered => 3,
        FrameFifPrt::DoubleBuffered => 2,
        FrameFifPrt::SingleBuffered => 1,
    }
}

/// Ship default: aggregate Prt + env overrides on Stp.
pub fn assemble_tandem_session(window: &Window) -> Result<TandemBfr, String> {
    assemble_tandem_session_with(
        window,
        TandemSessionPrt::ShipMailboxAa4NoValidation,
        None,
    )
}

/// Dual path: `prt` expands to knobs; optional `stp_override` replaces full bag; then ship env.
pub fn assemble_tandem_session_with(
    window: &Window,
    prt: TandemSessionPrt,
    stp_override: Option<TandemSessionStpPkg>,
) -> Result<TandemBfr, String> {
    let mut session_stp = prt.to_session_stp();
    if let Some(o) = stp_override {
        session_stp = session_stp.merge_override(&o);
    }
    session_stp = session_stp.with_ship_env();
    assemble_tandem_session_knobs(window, session_stp)
}

/// Full knobs path (no aggregate Prt).
pub fn assemble_tandem_session_knobs(
    window: &Window,
    session_stp: TandemSessionStpPkg,
) -> Result<TandemBfr, String> {
    let size = window.inner_size();
    let w = size.width.max(1);
    let h = size.height.max(1);
    session_log::log(&format!(
        "cubes viewport/swapchain · {w}x{h} · session={}",
        session_stp.desc
    ));

    let assembly = match session_stp.validation_prefer_op {
        ValidationPreferPrt::PreferValidation => SwapchainAssemblyPrt::GraphicsPresentValidation,
        ValidationPreferPrt::NoValidation => SwapchainAssemblyPrt::GraphicsPresentNoValidation,
    };

    let mut swapchain_bfr = SwapchainBfr::auto_assemble();
    let validation_on = match SwapchainBfr::import_for_asm8(
        &mut swapchain_bfr,
        assembly,
        surface_from_window(window)?,
    ) {
        Ok(()) => {
            session_log::log(&format!("swapchain boot · {assembly:?}"));
            matches!(
                session_stp.validation_prefer_op,
                ValidationPreferPrt::PreferValidation
            )
        }
        Err(e) if matches!(assembly, SwapchainAssemblyPrt::GraphicsPresentValidation) => {
            session_log::log(&format!(
                "swapchain validation path failed · {e} · falling back to NoValidation"
            ));
            swapchain_bfr = SwapchainBfr::auto_assemble();
            SwapchainBfr::import_for_asm8(
                &mut swapchain_bfr,
                SwapchainAssemblyPrt::GraphicsPresentNoValidation,
                surface_from_window(window)?,
            )?;
            session_log::log("swapchain boot · GraphicsPresentNoValidation");
            false
        }
        Err(e) => return Err(e),
    };

    SwapchainBfr::import_present_for_asm1(
        &mut swapchain_bfr,
        session_stp.present_prt_op,
        w,
        h,
    )?;
    let swapchain_default_rt_pkg = swapchain_bfr
        .swapchain_default_rt_pkg
        .take()
        .ok_or_else(|| "cubes: KHR product missing".to_string())?;
    let swapchain_rt_crg = SwapchainBfr::export_asmed1(&swapchain_bfr)
        .ok_or_else(|| "cubes: swapchain_rt_crg missing".to_string())?;

    let inst = &swapchain_rt_crg.instance_default_rt.instance_extrl;
    let phys = swapchain_rt_crg
        .physical_device_default_rt_pkg
        .physical_device_extrl;
    let props = unsafe { inst.get_physical_device_properties(phys) };
    let gpu_name = unsafe { CStr::from_ptr(props.device_name.as_ptr()) }.to_string_lossy();
    session_log::log(&format!(
        "GPU · {gpu_name} · type=0x{:x} · vendor=0x{:x} · device=0x{:x} · driver=0x{:x} · api={}.{}.{}",
        props.device_type.as_raw(),
        props.vendor_id,
        props.device_id,
        props.driver_version,
        vk::api_version_major(props.api_version),
        vk::api_version_minor(props.api_version),
        vk::api_version_patch(props.api_version),
    ));

    let sample_count = pick_sample_count_prefer(
        inst,
        phys,
        map_sample_prefer(session_stp.sample_prefer_op),
    );
    let render_lane = RenderLanePrt::for_sample_count(sample_count);
    let depth_format = pick_depth_format(inst, phys)?;
    let surface_format = swapchain_default_rt_pkg.surface_format_op.format;
    session_log::log(&format!(
        "cubes ship GPU path · validation={validation_on} · samples=0x{:x} · depth=0x{:x} · lane={render_lane:?} · present={:?} · format=0x{:x}",
        sample_count.as_raw(),
        depth_format.as_raw(),
        session_stp.present_prt_op,
        surface_format.as_raw(),
    ));

    let mut renderer_bfr = RendererBfr::auto_assemble();
    RendererBfr::import_for_asm9(
        &mut renderer_bfr,
        render_lane,
        w,
        h,
        surface_format,
        &swapchain_rt_crg.device_default_rt_pkg,
    )?;
    let renderer_rt = renderer_bfr
        .cargo_rt
        .take()
        .ok_or_else(|| "cubes: renderer cargo missing".to_string())?;

    let mut presentation_bfr = PresentationBfr::auto_assemble();
    PresentationBfr::import_for_asm6(
        &mut presentation_bfr,
        swapchain_rt_crg,
        &renderer_rt.render_pass_triangle_rt_pkg,
        swapchain_default_rt_pkg,
        sample_count,
        depth_format,
    )?;
    let presentation_rt = presentation_bfr
        .cargo_rt
        .take()
        .ok_or_else(|| "cubes: presentation cargo missing".to_string())?;

    let mut frame_bfr = FrameBfr::auto_assemble();
    FrameBfr::import_for_asm3(
        &mut frame_bfr,
        session_stp.frame_fif_prt_op,
        &swapchain_rt_crg.device_default_rt_pkg,
        &swapchain_rt_crg.swapchain_command_pool_default_rt_pkg,
    )?;
    let frame_rt = frame_bfr
        .cargo_rt
        .take()
        .ok_or_else(|| "cubes: frame cargo missing".to_string())?;

    let fif = frames_in_flight_u32(session_stp.frame_fif_prt_op);
    let mut display_bfr = DisplayBfr::auto_assemble();
    DisplayBfr::import_for_asm5(
        &mut display_bfr,
        DisplayPresentPrt::DefaultPresent,
        fif,
        &swapchain_rt_crg.device_default_rt_pkg,
        &swapchain_rt_crg.swapchain_command_pool_default_rt_pkg,
    )?;
    let display_rt = display_bfr
        .cargo_rt
        .take()
        .ok_or_else(|| "cubes: display cargo missing".to_string())?;

    let cube_count = session_stp.cube_count_stp.max(1);
    let mesh = MeshSoaRtBfr::unit_cuboid_instanced_lattice(cube_count, 1.25);

    let mut mesh_gpu_bfr = MeshGpuBfr::auto_assemble();
    MeshGpuBfr::import_for_asm1(
        &mut mesh_gpu_bfr,
        MeshDrawPrt::Solid,
        &swapchain_rt_crg.device_default_rt_pkg.device_extrl,
        inst,
        phys,
        &mesh,
    )?;
    let mesh_gpu_rt = mesh_gpu_bfr
        .mesh_gpu_default_rt_pkg
        .take()
        .ok_or_else(|| "cubes: mesh_gpu missing".to_string())?;

    let aspect = w as f32 / h as f32;
    let mesh_push_rt = MeshPushRt::from_orbit(
        mesh_gpu_rt.center_rt(),
        mesh_gpu_rt.radius_rt() * 2.8,
        0.6,
        0.4,
        aspect,
        [
            mesh_gpu_rt.base_r_rt,
            mesh_gpu_rt.base_g_rt,
            mesh_gpu_rt.base_b_rt,
        ],
    );

    session_log::log(&format!(
        "cubes TANDEM ok · n={cube_count} · tris={} · {w}x{h} · {}",
        mesh_gpu_rt.triangle_count_rt, session_stp.desc
    ));

    Ok(TandemBfr {
        swapchain_bfr,
        renderer_rt,
        presentation_rt,
        frame_rt,
        display_rt,
        mesh_gpu_rt,
        mesh_push_rt,
        session_stp,
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
