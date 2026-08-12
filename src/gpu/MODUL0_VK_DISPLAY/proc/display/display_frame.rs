//! Frame pass — proc owns branching (A2-PROC-COMPUTE).

use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayDefaultRtCrg;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::record_line_layers_rt::{RecordLineLayersRt, RECORD_LINE_LAYERS_EMPTY};
use crate::gpu::MODUL0_VK_DISPLAY::proc::display::record_frame::record_display_frame;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::{FrameRenderDefaultRtPkg, FrameSlotDefaultRtPkg};
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::line_gpu_default_rt_pkg::LineGpuDefaultRtPkg;
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::{
    MeshPushRt, MeshGpuDefaultRtPkg,
};
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::RendererDefaultRtCrg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::{DeviceDefaultRtPkg, PresentationDefaultRtCrg};
use crate::ModulResult;

/// Marker trait for Vulkan display targets.
pub trait VulkanDisplayble {}

/// Record display frame from peer W peels (`bind_geometry_stp` lever).
pub fn record_display_frame_from_peels(
    device: &DeviceDefaultRtPkg,
    presentation: &PresentationDefaultRtCrg,
    renderer: &RendererDefaultRtCrg,
    slot: &FrameSlotDefaultRtPkg,
    render_policy: &FrameRenderDefaultRtPkg,
    bind_geometry_stp: bool,
    image_index: u32,
) -> ModulResult<()> {
    record_display_frame(
        device,
        presentation,
        renderer,
        slot,
        render_policy,
        bind_geometry_stp,
        None,
        None,
        RECORD_LINE_LAYERS_EMPTY,
        image_index,
    )
}

/// Record one display frame; bumps Internal frame serial on success.
pub fn record_frame_with_serial(
    device: &DeviceDefaultRtPkg,
    presentation: &PresentationDefaultRtCrg,
    renderer: &RendererDefaultRtCrg,
    slot: &FrameSlotDefaultRtPkg,
    render_policy: &FrameRenderDefaultRtPkg,
    bind_geometry_stp: bool,
    mesh_gpu: Option<&MeshGpuDefaultRtPkg>,
    mesh_push: Option<&MeshPushRt>,
    grid: Option<&LineGpuDefaultRtPkg>,
    sketch: Option<&LineGpuDefaultRtPkg>,
    outline: Option<&LineGpuDefaultRtPkg>,
    image_index: u32,
    display_rt: &mut DisplayDefaultRtCrg,
) -> ModulResult<()> {
    display_rt.command_rt.recording_rt = true;
    let result = record_display_frame(
        device,
        presentation,
        renderer,
        slot,
        render_policy,
        bind_geometry_stp,
        mesh_gpu,
        mesh_push,
        RecordLineLayersRt {
            grid_line_gpu_default_rt_pkg: grid,
            sketch_line_gpu_default_rt_pkg: sketch,
            outline_line_gpu_default_rt_pkg: outline,
        },
        image_index,
    );
    display_rt.command_rt.recording_rt = false;
    result?;
    display_rt.display_render_default_rt.frame_serial_rt = display_rt
        .display_render_default_rt
        .frame_serial_rt
        .saturating_add(1);
    Ok(())
}
