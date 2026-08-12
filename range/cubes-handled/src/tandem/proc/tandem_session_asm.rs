//! Handled-only tandem boot · **one** `assemble_tandem_session`.
//!
//! Every modul `*Stp` / `*StpPkg` used on this path is built here field-by-field.
//! Every `vk::*` lever on those bags is written as an explicit enum/flags value.
//! No Auto catalogs. No helper split. No silent assembler knobs for fields that live on *Stp.

use std::ffi::CStr;

use ash::vk;
use modul::gpu::MODUL0_VK_DISPLAY::conv::port::DisplayTransportable;
use modul::gpu::MODUL0_VK_DISPLAY::mem::asm_disasm::vk_bfr::handled::display_bfr_hld_asm::DisplayBfrHandled;
use modul::gpu::MODUL0_VK_DISPLAY::mem::base::embedded::buffer::DisplayBfr;
use modul::gpu::MODUL0_VK_FRAME::conv::port::FrameTransportable;
use modul::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_bfr::handled::frame_bfr_hld_asm::FrameBfrHandled;
use modul::gpu::MODUL0_VK_FRAME::mem::base::embedded::buffer::FrameBfr;
use modul::gpu::MODUL0_VK_MESH::mem::asm_disasm::vk_pkg::handled::mesh_gpu_hld_asm::MeshGpuDefaultHandled;
use modul::gpu::MODUL0_VK_MESH::mem::base::transport::prt::MeshDrawPrt;
use modul::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::{
    MeshGpuDefaultRtPkg, MeshPushRt,
};
use modul::cpu::MODUL0_MESH::mem::base::transport::runtime::mesh_soa_rt_bfr::MeshSoaRtBfr;
use modul::gpu::MODUL0_VK_MESH::mem::base::transport::setup::mesh_draw_default_stp_pkg::MeshDrawDefaultStpPkg;
use modul::gpu::MODUL0_VK_PIPELINE::conv::port::RendererTransportable;
use modul::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_bfr::handled::renderer_bfr_hld_asm::RendererBfrHandled;
use modul::gpu::MODUL0_VK_PIPELINE::mem::base::embedded::buffer::RendererBfr;
use modul::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::op::RenderPassAttachmentLayoutStpPkgOp;
use modul::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::{
    DescriptorSetLayoutDefaultStpPkg, PipelineTriangleStpPkg, RenderPassTriangleStpPkg,
    SamplerDefaultStpPkg,
};
use modul::gpu::MODUL0_VK_SWAPCHAIN::conv::port::{
    PresentationTransportable, SwapchainTransportable,
};
use modul::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_bfr::handled::presentation_bfr_hld_asm::PresentationBfrHandled;
use modul::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_bfr::handled::swapchain_bfr_hld_asm::SwapchainBfrHandled;
use modul::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::buffer::{PresentationBfr, SwapchainBfr};
use modul::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::prt::SwapchainAssemblyPrt;
use modul::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::setup::boot_res_intsct_stp_pkgs::{
    SurfaceWindowStpPkg, SwapchainAssemblyDefaultStpPkg,
};
use modul::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::setup::present_res_intsct_stp_pkgs::{
    PresentationDefaultStpPkg, SwapchainDefaultStpPkg,
};
use modul::gpu::MODUL0_VK_SWAPCHAIN::proc::processor::device_caps::{
    pick_depth_format, pick_sample_count_prefer,
};
use modul::tandem::MODUL0_TANDEM::mem::base::transport::setup::TandemSessionStpPkg;
use modul::tandem::MODUL0_TANDEM::TandemBfr;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::window::Window;

use modul::gpu::MODUL0_VK_DISPLAY::mem::asm_disasm::vk_pkg::auto::display_present_prt_at_asm::DisplayPresentDefaultStpAuto;
use modul::gpu::MODUL0_VK_DISPLAY::mem::base::transport::setup::display_present_default_stp_pkg::DisplayPresentDefaultStpPkg;
use modul::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_pkg::auto::frame_fif_prt_at_asm::FrameFifDefaultStpAuto;
use modul::gpu::MODUL0_VK_FRAME::mem::base::transport::setup::frame_fif_default_stp_pkg::FrameFifDefaultStpPkg;
use modul::gpu::MODUL0_VK_MESH::mem::asm_disasm::vk_pkg::auto::mesh_draw_prt_at_asm::{
    mesh_draw_mode_stp, mesh_draw_prt_from_mode_stp,
};
use modul::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_pkg::auto::swapchain_prt_at_asm::swapchain_prt_format_present;
use modul::tandem::MODUL0_TANDEM::mem::asm_disasm::vk_pkg::auto::validation_sample_prefer_at_asm::{
    sample_count_prefer_from_prt, swapchain_assembly_from_validation_prt,
    validation_layers_stp_from_prt,
};

use crate::tandem::proc::session_log;

/// Handled session · single linear method · full *Stp + full vk levers on every bag.
pub fn assemble_tandem_session(
    window: &Window,
    session_stp: TandemSessionStpPkg,
) -> Result<TandemBfr, String> {
    let size = window.inner_size();
    let w = size.width.max(1);
    let h = size.height.max(1);
    session_log::log(&format!("viewport · {w}x{h} · {}", session_stp.desc));

    // ══════════════════════════════════════════════════════════════════════════
    // 1 · SWAPCHAIN BOOT *Stp (modul API · every field)
    // ══════════════════════════════════════════════════════════════════════════
    let display = window
        .display_handle()
        .map_err(|e| format!("cubes: display handle: {e}"))?
        .as_raw();
    let window_h = window
        .window_handle()
        .map_err(|e| format!("cubes: window handle: {e}"))?
        .as_raw();

    // SurfaceWindowStpPkg — window peels (extrl handles)
    let surface_window_stp_pkg = SurfaceWindowStpPkg {
        display_handle_extrl: display,
        window_handle_extrl: window_h,
        desc: "cubes_vk_surface",
    };

    // SwapchainAssemblyDefaultStpPkg — validation knob (bool setup, not Prt)
    let swapchain_assembly_default_stp_pkg = SwapchainAssemblyDefaultStpPkg {
        validation_layers_stp: validation_layers_stp_from_prt(session_stp.validation_prefer_op),
        desc: "cubes_hld_assembly",
    };
    let assembly_intent = swapchain_assembly_from_validation_prt(session_stp.validation_prefer_op);

    session_log::log(&format!(
        "Stp · SurfaceWindowStpPkg {{ desc={} }}",
        surface_window_stp_pkg.desc
    ));
    session_log::log(&format!(
        "Stp · SwapchainAssemblyDefaultStpPkg {{ validation_layers_stp={} · desc={} }}",
        swapchain_assembly_default_stp_pkg.validation_layers_stp,
        swapchain_assembly_default_stp_pkg.desc
    ));

    let mut swapchain_bfr = SwapchainBfr::handled_assemble(surface_window_stp_pkg);
    let validation_on = match SwapchainBfr::import_for_asm7_from_stp(
        &mut swapchain_bfr,
        swapchain_assembly_default_stp_pkg.validation_layers_stp,
        assembly_intent,
    ) {
        Ok(()) => {
            session_log::log(&format!(
                "swapchain boot · HANDLED · validation_layers_stp={}",
                swapchain_assembly_default_stp_pkg.validation_layers_stp
            ));
            swapchain_assembly_default_stp_pkg.validation_layers_stp
        }
        Err(e) if swapchain_assembly_default_stp_pkg.validation_layers_stp => {
            session_log::log(&format!(
                "swapchain validation fail · {e} · validation_layers_stp=false"
            ));
            let display = window
                .display_handle()
                .map_err(|e| format!("cubes: display handle: {e}"))?
                .as_raw();
            let window_h = window
                .window_handle()
                .map_err(|e| format!("cubes: window handle: {e}"))?
                .as_raw();
            let surface_window_stp_pkg = SurfaceWindowStpPkg {
                display_handle_extrl: display,
                window_handle_extrl: window_h,
                desc: "cubes_vk_surface",
            };
            let swapchain_assembly_default_stp_pkg = SwapchainAssemblyDefaultStpPkg {
                validation_layers_stp: false,
                desc: "cubes_hld_assembly_noval",
            };
            swapchain_bfr = SwapchainBfr::handled_assemble(surface_window_stp_pkg);
            SwapchainBfr::import_for_asm7_from_stp(
                &mut swapchain_bfr,
                swapchain_assembly_default_stp_pkg.validation_layers_stp,
                SwapchainAssemblyPrt::GRAPHICS_PRESENT_NO_VALIDATION,
            )?;
            false
        }
        Err(e) => return Err(e),
    };

    let swapchain_rt_crg = SwapchainBfr::export_asmed1(&swapchain_bfr)
        .ok_or_else(|| "cubes: swapchain_rt_crg missing".to_string())?;
    let inst = &swapchain_rt_crg.instance_default_rt.instance_extrl;
    let phys = swapchain_rt_crg
        .physical_device_default_rt_pkg
        .physical_device_extrl;
    let props = unsafe { inst.get_physical_device_properties(phys) };
    let gpu_name = unsafe { CStr::from_ptr(props.device_name.as_ptr()) }.to_string_lossy();
    session_log::log(&format!(
        "GPU · {gpu_name} · type=0x{:x} · vendor=0x{:x} · device=0x{:x} · api={}.{}.{}",
        props.device_type.as_raw(),
        props.vendor_id,
        props.device_id,
        vk::api_version_major(props.api_version),
        vk::api_version_minor(props.api_version),
        vk::api_version_patch(props.api_version),
    ));

    // Device picks feed *Stp (still written as explicit vk:: values below)
    let sample_count_op: vk::SampleCountFlags =
        pick_sample_count_prefer(inst, phys, sample_count_prefer_from_prt(session_stp.sample_prefer_op));
    let depth_format_op: vk::Format = pick_depth_format(inst, phys)?;

    // ══════════════════════════════════════════════════════════════════════════
    // 2 · SWAPCHAIN KHR *Stp — every vk lever explicit (Format · PresentModeKHR)
    // ══════════════════════════════════════════════════════════════════════════
    // Expand session present picture → DirectVk (W.DirectVk · closed on *Stp)
    let (surface_format_op, present_mode_op, khr_desc) =
        swapchain_prt_format_present(session_stp.present_prt_op);
    let swapchain_default_stp_pkg = SwapchainDefaultStpPkg {
        extent_width_stp: w,
        extent_height_stp: h,
        surface_format_op, // vk::Format
        present_mode_op,   // vk::PresentModeKHR
        image_usage_op: vk::ImageUsageFlags::COLOR_ATTACHMENT,
        composite_alpha_op: vk::CompositeAlphaFlagsKHR::OPAQUE,
        desc: khr_desc,
    };
    session_log::log(&format!(
        "Stp · SwapchainDefaultStpPkg {{ extent={}x{} · format=0x{:x} · present=0x{:x} · usage=0x{:x} · composite_alpha=0x{:x} }}",
        swapchain_default_stp_pkg.extent_width_stp,
        swapchain_default_stp_pkg.extent_height_stp,
        swapchain_default_stp_pkg.surface_format_op.as_raw(),
        swapchain_default_stp_pkg.present_mode_op.as_raw(),
        swapchain_default_stp_pkg.image_usage_op.as_raw(),
        swapchain_default_stp_pkg.composite_alpha_op.as_raw(),
    ));
    SwapchainBfr::import_present_for_asm1_from_stp(
        &mut swapchain_bfr,
        swapchain_default_stp_pkg,
    )?;
    let swapchain_default_rt_pkg = swapchain_bfr
        .swapchain_default_rt_pkg
        .take()
        .ok_or_else(|| "cubes: KHR missing".to_string())?;
    // Re-export crg after present (same boot cargo)
    let swapchain_rt_crg = SwapchainBfr::export_asmed1(&swapchain_bfr)
        .ok_or_else(|| "cubes: swapchain_rt_crg missing".to_string())?;
    let inst = &swapchain_rt_crg.instance_default_rt.instance_extrl;
    let phys = swapchain_rt_crg
        .physical_device_default_rt_pkg
        .physical_device_extrl;

    let surface_format_op: vk::Format = swapchain_default_rt_pkg.surface_format_op.format;
    let wire = matches!(session_stp.mesh_draw_prt_op, MeshDrawPrt::WIREFRAME);
    session_log::log(&format!(
        "path · HANDLED · validation={validation_on} · samples=0x{:x} · depth=0x{:x} · surface=0x{:x} · wire={wire}",
        sample_count_op.as_raw(),
        depth_format_op.as_raw(),
        surface_format_op.as_raw(),
    ));

    // ══════════════════════════════════════════════════════════════════════════
    // 3 · RENDER PASS *Stp — every field · every vk enum
    // ══════════════════════════════════════════════════════════════════════════
    let attachment_layout_op = if sample_count_op == vk::SampleCountFlags::TYPE_1 {
        RenderPassAttachmentLayoutStpPkgOp::SIMPLE
    } else {
        RenderPassAttachmentLayoutStpPkgOp::MSAA
    };
    let render_pass_triangle_stp_pkg = RenderPassTriangleStpPkg {
        surface_format_op,
        sample_count_op,
        attachment_layout_op,
        depth_format_op,
        color_layout_op: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        depth_layout_op: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        present_layout_op: vk::ImageLayout::PRESENT_SRC_KHR,
        initial_layout_op: vk::ImageLayout::UNDEFINED,
        color_load_op: vk::AttachmentLoadOp::CLEAR,
        color_store_op: vk::AttachmentStoreOp::STORE,
        depth_load_op: vk::AttachmentLoadOp::CLEAR,
        depth_store_op: vk::AttachmentStoreOp::DONT_CARE,
        desc: "cubes_hld_rp",
    };
    session_log::log(&format!(
        "Stp · RenderPassTriangleStpPkg {{ surface=0x{:x} · samples=0x{:x} · layout={:?} · depth=0x{:x} · color_load=0x{:x} · color_store=0x{:x} · depth_load=0x{:x} · depth_store=0x{:x} }}",
        render_pass_triangle_stp_pkg.surface_format_op.as_raw(),
        render_pass_triangle_stp_pkg.sample_count_op.as_raw(),
        render_pass_triangle_stp_pkg.attachment_layout_op,
        render_pass_triangle_stp_pkg.depth_format_op.as_raw(),
        render_pass_triangle_stp_pkg.color_load_op.as_raw(),
        render_pass_triangle_stp_pkg.color_store_op.as_raw(),
        render_pass_triangle_stp_pkg.depth_load_op.as_raw(),
        render_pass_triangle_stp_pkg.depth_store_op.as_raw(),
    ));

    // ══════════════════════════════════════════════════════════════════════════
    // 4 · PIPELINE *Stp — every field · P0 depth/blend/primitive
    // ══════════════════════════════════════════════════════════════════════════
    let pipeline_triangle_stp_pkg = PipelineTriangleStpPkg {
        sample_count_op,
        topology_op: vk::PrimitiveTopology::TRIANGLE_LIST,
        polygon_mode_op: if wire {
            vk::PolygonMode::LINE
        } else {
            vk::PolygonMode::FILL
        },
        cull_mode_op: vk::CullModeFlags::NONE,
        front_face_op: vk::FrontFace::COUNTER_CLOCKWISE,
        depth_test_enable_stp: true,
        depth_write_enable_stp: true,
        depth_compare_op: vk::CompareOp::LESS,
        color_write_mask_op: vk::ColorComponentFlags::RGBA,
        blend_enable_stp: false,
        src_color_blend_factor_op: vk::BlendFactor::ONE,
        dst_color_blend_factor_op: vk::BlendFactor::ZERO,
        color_blend_op: vk::BlendOp::ADD,
        primitive_restart_enable_stp: false,
        line_width_stp: 1.0,
        extent_width_stp: w,
        extent_height_stp: h,
        desc: "cubes_hld_pl",
    };
    session_log::log(&format!(
        "Stp · PipelineTriangleStpPkg {{ samples=0x{:x} · topology=0x{:x} · polygon=0x{:x} · cull=0x{:x} · depth_test={} · depth_write={} · depth_cmp=0x{:x} · blend={} · src=0x{:x} · dst=0x{:x} · line_w={} · extent={}x{} }}",
        pipeline_triangle_stp_pkg.sample_count_op.as_raw(),
        pipeline_triangle_stp_pkg.topology_op.as_raw(),
        pipeline_triangle_stp_pkg.polygon_mode_op.as_raw(),
        pipeline_triangle_stp_pkg.cull_mode_op.as_raw(),
        pipeline_triangle_stp_pkg.depth_test_enable_stp,
        pipeline_triangle_stp_pkg.depth_write_enable_stp,
        pipeline_triangle_stp_pkg.depth_compare_op.as_raw(),
        pipeline_triangle_stp_pkg.blend_enable_stp,
        pipeline_triangle_stp_pkg.src_color_blend_factor_op.as_raw(),
        pipeline_triangle_stp_pkg.dst_color_blend_factor_op.as_raw(),
        pipeline_triangle_stp_pkg.line_width_stp,
        pipeline_triangle_stp_pkg.extent_width_stp,
        pipeline_triangle_stp_pkg.extent_height_stp,
    ));

    // Idle bags (modul API) — product path does not bind descriptors yet; knobs still explicit.
    let sampler_default_stp_pkg = SamplerDefaultStpPkg {
        mag_filter_op: vk::Filter::LINEAR,
        min_filter_op: vk::Filter::LINEAR,
        address_mode_u_op: vk::SamplerAddressMode::REPEAT,
        address_mode_v_op: vk::SamplerAddressMode::REPEAT,
        address_mode_w_op: vk::SamplerAddressMode::REPEAT,
        anisotropy_enable_stp: false,
        max_anisotropy_stp: 1.0,
        desc: "cubes_hld_sampler",
    };
    let descriptor_set_layout_default_stp_pkg = DescriptorSetLayoutDefaultStpPkg {
        bindings_op: Vec::new(),
        desc: "cubes_hld_dsl_empty",
    };
    session_log::log(&format!(
        "Stp · SamplerDefaultStpPkg {{ mag=0x{:x} · min=0x{:x} · address_u=0x{:x} · aniso={} }}",
        sampler_default_stp_pkg.mag_filter_op.as_raw(),
        sampler_default_stp_pkg.min_filter_op.as_raw(),
        sampler_default_stp_pkg.address_mode_u_op.as_raw(),
        sampler_default_stp_pkg.anisotropy_enable_stp,
    ));
    session_log::log(&format!(
        "Stp · DescriptorSetLayoutDefaultStpPkg {{ bindings={} · desc={} }}",
        descriptor_set_layout_default_stp_pkg.bindings_op.len(),
        descriptor_set_layout_default_stp_pkg.desc,
    ));
    let _ = (&sampler_default_stp_pkg, &descriptor_set_layout_default_stp_pkg);

    let mut renderer_bfr = RendererBfr::handled_assemble(
        render_pass_triangle_stp_pkg,
        pipeline_triangle_stp_pkg,
    );
    RendererBfr::import_for_asm8_from_stp(
        &mut renderer_bfr,
        &swapchain_rt_crg.device_default_rt_pkg,
    )?;
    let renderer_rt = renderer_bfr
        .cargo_rt
        .take()
        .ok_or_else(|| "cubes: renderer cargo missing".to_string())?;

    // ══════════════════════════════════════════════════════════════════════════
    // 5 · PRESENTATION *Stp — sample_count · depth_format
    // ══════════════════════════════════════════════════════════════════════════
    let presentation_default_stp_pkg = PresentationDefaultStpPkg {
        sample_count_op, // vk::SampleCountFlags
        depth_format_op, // vk::Format
        desc: "cubes_hld_present",
    };
    session_log::log(&format!(
        "Stp · PresentationDefaultStpPkg {{ samples=0x{:x} · depth=0x{:x} }}",
        presentation_default_stp_pkg.sample_count_op.as_raw(),
        presentation_default_stp_pkg.depth_format_op.as_raw(),
    ));
    let mut presentation_bfr =
        PresentationBfr::handled_assemble(presentation_default_stp_pkg);
    PresentationBfr::import_for_asm5_from_stp(
        &mut presentation_bfr,
        swapchain_rt_crg,
        &renderer_rt.render_pass_triangle_rt_pkg,
        swapchain_default_rt_pkg,
    )?;
    let presentation_rt = presentation_bfr
        .cargo_rt
        .take()
        .ok_or_else(|| "cubes: presentation cargo missing".to_string())?;

    // ══════════════════════════════════════════════════════════════════════════
    // 6 · FRAME FIF *Stp — every field
    // ══════════════════════════════════════════════════════════════════════════
    let frame_fif_default_stp_pkg = FrameFifDefaultStpPkg::auto_assemble(session_stp.frame_fif_prt_op);
    let fif = frame_fif_default_stp_pkg.frames_in_flight_stp;
    session_log::log(&format!(
        "Stp · FrameFifDefaultStpPkg {{ frames_in_flight_stp={} · fences_signaled_stp={} · primary_command_buffers_stp={} }}",
        frame_fif_default_stp_pkg.frames_in_flight_stp,
        frame_fif_default_stp_pkg.fences_signaled_stp,
        frame_fif_default_stp_pkg.primary_command_buffers_stp,
    ));
    let mut frame_bfr = FrameBfr::handled_assemble(frame_fif_default_stp_pkg);
    FrameBfr::import_for_asm2_from_stp(
        &mut frame_bfr,
        &swapchain_rt_crg.device_default_rt_pkg,
        &swapchain_rt_crg.swapchain_command_pool_default_rt_pkg,
    )?;
    let frame_rt = frame_bfr
        .cargo_rt
        .take()
        .ok_or_else(|| "cubes: frame cargo missing".to_string())?;

    // ══════════════════════════════════════════════════════════════════════════
    // 7 · DISPLAY *Stp — every field
    // ══════════════════════════════════════════════════════════════════════════
    let display_present_default_stp_pkg =
        DisplayPresentDefaultStpPkg::auto_assemble(session_stp.display_present_prt_op, fif);
    session_log::log(&format!(
        "Stp · DisplayPresentDefaultStpPkg {{ frames_in_flight_stp={} · clear_only_stp={} · bind_geometry_stp={} }}",
        display_present_default_stp_pkg.frames_in_flight_stp,
        display_present_default_stp_pkg.clear_only_stp,
        display_present_default_stp_pkg.bind_geometry_stp,
    ));
    let mut display_bfr = DisplayBfr::handled_assemble(display_present_default_stp_pkg);
    DisplayBfr::import_for_asm4_from_stp(
        &mut display_bfr,
        &swapchain_rt_crg.device_default_rt_pkg,
        &swapchain_rt_crg.swapchain_command_pool_default_rt_pkg,
    )?;
    let display_rt = display_bfr
        .cargo_rt
        .take()
        .ok_or_else(|| "cubes: display cargo missing".to_string())?;

    // ══════════════════════════════════════════════════════════════════════════
    // 8 · MESH *Stp — every field (mode + steel base RGB + counts)
    // ══════════════════════════════════════════════════════════════════════════
    let cube_count = session_stp.cube_count_stp.max(1);
    let mesh_soa =
        MeshSoaRtBfr::unit_cuboid_instanced_lattice(cube_count, session_stp.lattice_spacing_stp);
    // MeshDrawDefaultStpPkg mode_stp from MeshDrawPrt peel
    let mesh_draw_default_stp_pkg = MeshDrawDefaultStpPkg {
        mode_stp: mesh_draw_mode_stp(session_stp.mesh_draw_prt_op),
        vertex_count_stp: mesh_soa.pos_xs.len() as u32,
        index_count_stp: mesh_soa.indices.len() as u32,
        base_r_stp: 0.55,
        base_g_stp: 0.62,
        base_b_stp: 0.78,
        desc: "cubes_hld_mesh",
    };
    session_log::log(&format!(
        "Stp · MeshDrawDefaultStpPkg {{ mode_stp={} · vertex_count_stp={} · index_count_stp={} · base_rgb=[{},{},{}] }}",
        mesh_draw_default_stp_pkg.mode_stp,
        mesh_draw_default_stp_pkg.vertex_count_stp,
        mesh_draw_default_stp_pkg.index_count_stp,
        mesh_draw_default_stp_pkg.base_r_stp,
        mesh_draw_default_stp_pkg.base_g_stp,
        mesh_draw_default_stp_pkg.base_b_stp,
    ));
    // Drive product from MeshDrawDefaultStpPkg.mode_stp (not session Prt mask).
    let mesh_draw_prt = mesh_draw_prt_from_mode_stp(mesh_draw_default_stp_pkg.mode_stp);
    let mesh_gpu_rt = MeshGpuDefaultRtPkg::handled_assemble(
        &swapchain_rt_crg.device_default_rt_pkg.device_extrl,
        inst,
        phys,
        &mesh_soa,
        mesh_draw_prt,
    )?;

    let aspect = w as f32 / h as f32;
    let mesh_push_rt = MeshPushRt::from_orbit(
        mesh_gpu_rt.center_rt(),
        mesh_gpu_rt.radius_rt() * session_stp.camera_radius_scale_stp,
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
        "TANDEM · HANDLED · n={cube_count} · tris={} · {w}x{h} · {}",
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
