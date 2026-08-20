//! Auto tandem boot · one method · mem/asm_disasm/**/auto · FIX-129/131.
//!
//! Aggregate Prt → auto_assemble *Bfr → import_for_asm* PortMatch.
//! No helper split.

use std::ffi::CStr;

use ash::vk;
use modul::gpu::MODUL0_VK_DISPLAY::conv::port::{DisplayBfr, DisplayBfrAuto, DisplayTransportable};
use modul::gpu::MODUL0_VK_FRAME::conv::port::{FrameBfr, FrameBfrAuto, FrameTransportable};
use modul::gpu::MODUL0_VK_MESH::conv::port::{MeshGpuBfr, MeshGpuBfrAuto, MeshGpuTransportable};
use modul::cpu::MODUL0_MESH::proc::processor::unit_cuboid_lattice_meta;
use modul::gpu::MODUL0_VK_MESH::proc::processor::{
    mesh_gpu_center_rt, mesh_gpu_radius_rt, mesh_push_from_orbit,
};
use modul::gpu::MODUL0_VK_PIPELINE::conv::port::{
    RendererBfr, RendererBfrAuto, RendererTransportable,
};
use modul::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_pkg::auto::render_lane_stp_at_asm::render_lane_prt_for_sample_count;
use modul::gpu::MODUL0_VK_SWAPCHAIN::conv::port::{
    PresentationBfr, PresentationBfrAuto, PresentationTransportable, SwapchainBfr,
    SwapchainBfrAuto, SwapchainTransportable,
};
use modul::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::prt::SwapchainAssemblyPrt;
use modul::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::setup::present_res_intsct_stp_pkgs::SwapchainDefaultStpPkg;
use modul::gpu::MODUL0_VK_SWAPCHAIN::proc::processor::device_caps::{
    pick_depth_format, pick_sample_count_prefer,
};
use modul::tandem::MODUL0_TANDEM::mem::base::transport::prt::TandemSessionPrt;
use modul::tandem::MODUL0_TANDEM::TandemBfr;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::window::Window;

use modul::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_pkg::auto::frame_fif_prt_at_asm::frame_fif_frames_in_flight;
use modul::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_pkg::auto::swapchain_prt_at_asm::{
    surface_window_stp_from_raw, swapchain_prt_format_present,
};
use modul::tandem::MODUL0_TANDEM::mem::asm_disasm::vk_pkg::auto::validation_sample_prefer_at_asm::{
    sample_count_prefer_from_prt, swapchain_assembly_from_validation_prt,
    validation_layers_stp_from_prt,
};
use modul::tandem::MODUL0_TANDEM::mem::asm_disasm::vk_pkg::auto::tandem_session_prt_at_asm::{
    tandem_session_prt_to_stp, tandem_session_stp_ship_env,
};

use crate::tandem::proc::session_log;

/// Auto session boot · single linear method · Prt + auto_assemble + import_for_asm*.
pub fn assemble_tandem_session(window: &Window) -> Result<TandemBfr, String> {
    let session_stp =
        tandem_session_stp_ship_env(tandem_session_prt_to_stp(TandemSessionPrt::SHIP_MAILBOX_AA4_NO_VALIDATION));

    let size = window.inner_size();
    let w = size.width.max(1);
    let h = size.height.max(1);
    session_log::log(&format!(
        "cubes viewport/swapchain · {w}x{h} · session={}",
        session_stp.desc
    ));

    // ── SWAPCHAIN ────────────────────────────────────────────────────────────
    let display = window
        .display_handle()
        .map_err(|e| format!("cubes: display handle: {e}"))?
        .as_raw();
    let window_h = window
        .window_handle()
        .map_err(|e| format!("cubes: window handle: {e}"))?
        .as_raw();
    let surface_window_stp = surface_window_stp_from_raw(display, window_h, "cubes_vk_surface");

    let assembly = swapchain_assembly_from_validation_prt(session_stp.validation_prefer_op);

    let mut swapchain_bfr = SwapchainBfr::auto_assemble();
    let validation_on = match SwapchainBfr::import_for_asm8(
        &mut swapchain_bfr,
        assembly,
        surface_window_stp,
    ) {
        Ok(()) => {
            session_log::log(&format!("swapchain boot · {assembly:?}"));
            validation_layers_stp_from_prt(session_stp.validation_prefer_op)
        }
        Err(e) if matches!(assembly, SwapchainAssemblyPrt::GRAPHICS_PRESENT_VALIDATION) => {
            session_log::log(&format!(
                "swapchain validation path failed · {e} · falling back to NO_VALIDATION"
            ));
            let display = window
                .display_handle()
                .map_err(|e| format!("cubes: display handle: {e}"))?
                .as_raw();
            let window_h = window
                .window_handle()
                .map_err(|e| format!("cubes: window handle: {e}"))?
                .as_raw();
            swapchain_bfr = SwapchainBfr::auto_assemble();
            SwapchainBfr::import_for_asm8(
                &mut swapchain_bfr,
                SwapchainAssemblyPrt::GRAPHICS_PRESENT_NO_VALIDATION,
                surface_window_stp_from_raw(display, window_h, "cubes_vk_surface"),
            )?;
            session_log::log("swapchain boot · GRAPHICS_PRESENT_NO_VALIDATION");
            false
        }
        Err(e) => return Err(e),
    };

    // PortMatch SwapchainPrt → full SwapchainDefaultStpPkg (vk levers via Prt peel)
    let (surface_format_op, present_mode_op, khr_desc) =
        swapchain_prt_format_present(session_stp.present_prt_op);
    let swapchain_default_stp_pkg = SwapchainDefaultStpPkg {
        extent_width_stp: w,
        extent_height_stp: h,
        surface_format_op,
        present_mode_op,
        image_usage_op: vk::ImageUsageFlags::COLOR_ATTACHMENT
            | vk::ImageUsageFlags::TRANSFER_DST,
        composite_alpha_op: vk::CompositeAlphaFlagsKHR::OPAQUE,
        desc: khr_desc,
    };
    session_log::log(&format!(
        "Stp · SwapchainDefaultStpPkg {{ extent={}x{} · format=0x{:x} · present=0x{:x} }}",
        swapchain_default_stp_pkg.extent_width_stp,
        swapchain_default_stp_pkg.extent_height_stp,
        swapchain_default_stp_pkg.surface_format_op.as_raw(),
        swapchain_default_stp_pkg.present_mode_op.as_raw(),
    ));
    SwapchainBfr::import_present_for_asm1_from_stp(
        &mut swapchain_bfr,
        swapchain_default_stp_pkg,
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
    if let Ok(listed) = unsafe { inst.enumerate_physical_devices() } {
        for (i, dev) in listed.iter().enumerate() {
            let p = unsafe { inst.get_physical_device_properties(*dev) };
            let name = unsafe { CStr::from_ptr(p.device_name.as_ptr()) }.to_string_lossy();
            let mark = if *dev == phys { "PICK" } else { "    " };
            let kind = match p.device_type {
                vk::PhysicalDeviceType::DISCRETE_GPU => "DISCRETE",
                vk::PhysicalDeviceType::INTEGRATED_GPU => "INTEGRATED",
                vk::PhysicalDeviceType::VIRTUAL_GPU => "VIRTUAL",
                vk::PhysicalDeviceType::CPU => "CPU",
                _ => "OTHER",
            };
            session_log::log(&format!(
                "GPU[{i}] {mark} {kind} {name} type=0x{:x} vendor=0x{:x} device=0x{:x}",
                p.device_type.as_raw(),
                p.vendor_id,
                p.device_id
            ));
        }
    }
    session_log::log(&format!(
        "CUBES_COUNT · requested={} · override=CUBES_COUNT env or argv --count/-n/integer",
        session_stp.cube_count_stp
    ));

    let sample_count = pick_sample_count_prefer(
        inst,
        phys,
        sample_count_prefer_from_prt(session_stp.sample_prefer_op),
    );
    let render_lane = session_stp
        .render_lane_prt_op
        .unwrap_or_else(|| render_lane_prt_for_sample_count(sample_count));
    let depth_format = pick_depth_format(inst, phys)?;
    let surface_format = swapchain_default_rt_pkg.surface_format_op.format;
    session_log::log(&format!(
        "cubes ship GPU path · validation={validation_on} · samples=0x{:x} · depth=0x{:x} · lane={render_lane:?} · present={:?} · format=0x{:x}",
        sample_count.as_raw(),
        depth_format.as_raw(),
        session_stp.present_prt_op,
        surface_format.as_raw(),
    ));

    // ── PIPELINE (Auto Prt → *Stp table inside import_for_asm9 · peel Stp after) ─
    let mut renderer_bfr = RendererBfr::auto_assemble();
    RendererBfr::import_for_asm9(
        &mut renderer_bfr,
        render_lane,
        w,
        h,
        surface_format,
        &swapchain_rt_crg.device_default_rt_pkg,
    )?;
    if let Some(rp) = renderer_bfr.render_pass_triangle_stp_pkg.as_ref() {
        session_log::log(&format!(
            "Stp · RenderPassTriangleStpPkg {{ surface=0x{:x} · samples=0x{:x} · depth=0x{:x} · color=0x{:x} · depth_layout=0x{:x} · present=0x{:x} · initial=0x{:x} }}",
            rp.surface_format_op.as_raw(),
            rp.sample_count_op.as_raw(),
            rp.depth_format_op.as_raw(),
            rp.color_layout_op.as_raw(),
            rp.depth_layout_op.as_raw(),
            rp.present_layout_op.as_raw(),
            rp.initial_layout_op.as_raw(),
        ));
    }
    if let Some(pl) = renderer_bfr.pipeline_triangle_stp_pkg.as_ref() {
        session_log::log(&format!(
            "Stp · PipelineTriangleStpPkg {{ samples=0x{:x} · topology=0x{:x} · polygon=0x{:x} · cull=0x{:x} · front=0x{:x} · depth_cmp=0x{:x} · color_mask=0x{:x} · extent={}x{} }}",
            pl.sample_count_op.as_raw(),
            pl.topology_op.as_raw(),
            pl.polygon_mode_op.as_raw(),
            pl.cull_mode_op.as_raw(),
            pl.front_face_op.as_raw(),
            pl.depth_compare_op.as_raw(),
            pl.color_write_mask_op.as_raw(),
            pl.extent_width_stp,
            pl.extent_height_stp,
        ));
    }
    let renderer_rt = renderer_bfr
        .cargo_rt
        .take()
        .ok_or_else(|| "cubes: renderer cargo missing".to_string())?;

    // ── PRESENTATION ─────────────────────────────────────────────────────────
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

    // ── FRAME ────────────────────────────────────────────────────────────────
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

    let fif = frame_fif_frames_in_flight(session_stp.frame_fif_prt_op);

    // ── DISPLAY ──────────────────────────────────────────────────────────────
    let mut display_bfr = DisplayBfr::auto_assemble();
    DisplayBfr::import_for_asm5(
        &mut display_bfr,
        session_stp.display_present_prt_op,
        fif,
        &swapchain_rt_crg.device_default_rt_pkg,
        &swapchain_rt_crg.swapchain_command_pool_default_rt_pkg,
    )?;
    let mut display_rt = display_bfr
        .cargo_rt
        .take()
        .ok_or_else(|| "cubes: display cargo missing".to_string())?;
    modul::gpu::MODUL0_VK_DISPLAY::proc::display::soa_color_target::update_soa_color_target(
        &swapchain_rt_crg.device_default_rt_pkg.device_extrl,
        inst,
        phys,
        presentation_rt.swapchain_default_rt_pkg.extent_rt,
        &mut display_rt,
    )?;
    modul::gpu::MODUL0_VK_MESH::proc::processor::mesh_soa_bind::bind_soa_color_image(
        &swapchain_rt_crg.device_default_rt_pkg.device_extrl,
        &renderer_rt,
        &display_rt,
    )?;
    session_log::log("soa-vulkan · heat-destroy cube · vkCmdDispatch · no cmdDraw · MAILBOX");

    // ── MESH ─────────────────────────────────────────────────────────────────
    let cube_count = session_stp.cube_count_stp.max(1);
    let mesh = unit_cuboid_lattice_meta(
        cube_count,
        session_stp.lattice_spacing_stp,
    );
    let mut mesh_gpu_bfr = MeshGpuBfr::auto_assemble();
    MeshGpuBfr::import_for_asm1(
        &mut mesh_gpu_bfr,
        session_stp.mesh_draw_prt_op,
        &swapchain_rt_crg.device_default_rt_pkg.device_extrl,
        inst,
        phys,
        &mesh,
    )?;
    let mut mesh_gpu_rt = mesh_gpu_bfr
        .mesh_gpu_default_rt_pkg
        .take()
        .ok_or_else(|| "cubes: mesh_gpu missing".to_string())?;
    mesh_gpu_rt.base_r_rt = 0.11;
    mesh_gpu_rt.base_g_rt = 0.11;
    mesh_gpu_rt.base_b_rt = 0.14;
    let heat_heap =
        modul::gpu::MODUL0_VK_DISPLAY::proc::display::soa_color_target::update_soa_heat_image(
            &swapchain_rt_crg.device_default_rt_pkg.device_extrl,
            inst,
            phys,
            mesh_gpu_rt.instance_count_rt,
            &mut display_rt,
        )?;
    modul::gpu::MODUL0_VK_DISPLAY::proc::display::soa_color_target::clear_soa_heat_image(
        &swapchain_rt_crg.device_default_rt_pkg.device_extrl,
        swapchain_rt_crg.device_default_rt_pkg.graphics_queue_extrl,
        swapchain_rt_crg
            .swapchain_command_pool_default_rt_pkg
            .command_pool_extrl,
        &mut display_rt,
    )?;
    modul::gpu::MODUL0_VK_MESH::proc::processor::mesh_soa_bind::bind_soa_color_image(
        &swapchain_rt_crg.device_default_rt_pkg.device_extrl,
        &renderer_rt,
        &display_rt,
    )?;
    session_log::log(&format!(
        "lattice n={} · heat image3D {}x{}x{} · {:.2} GiB · {} · vkCmdDispatch",
        mesh_gpu_rt.instance_count_rt,
        display_rt.soa_heat_extent_rt.width,
        display_rt.soa_heat_extent_rt.height,
        display_rt.soa_heat_extent_rt.depth,
        display_rt.soa_heat_bytes_rt as f64 / (1024.0 * 1024.0 * 1024.0),
        heat_heap
    ));

    let aspect = w as f32 / h as f32;
    let mesh_push_rt = mesh_push_from_orbit(
        mesh_gpu_center_rt(&mesh_gpu_rt),
        mesh_gpu_radius_rt(&mesh_gpu_rt) * session_stp.camera_radius_scale_stp,
        session_stp.orbit_yaw_stp,
        session_stp.orbit_pitch_stp,
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

    let orbit_yaw = session_stp.orbit_yaw_stp;
    let orbit_pitch = session_stp.orbit_pitch_stp;
    let zoom = session_stp.zoom_stp;

    Ok(TandemBfr {
        swapchain_bfr,
        renderer_rt,
        presentation_rt,
        frame_rt,
        display_rt,
        mesh_gpu_rt,
        mesh_push_rt,
        session_stp,
        orbit_yaw,
        orbit_pitch,
        zoom,
        dragging: false,
        heat_painting: false,
        last_cursor: None,
        cursor_px: (w as f32 * 0.5, h as f32 * 0.5),
        heat_hold_rt: 0.0,
        heat_decay_tail_rt: 0.0,
        fps: 0.0,
        fps_instant: 0.0,
        fps_sample_ready: false,
        fps_frames: 0,
        fps_window_start: std::time::Instant::now(),
        last_frame_end: std::time::Instant::now(),
        pulse_t0: std::time::Instant::now(),
        heat_diag_dumped: false,
    })
}
