//! Display render ops — clear / geometry branches (P · `*_stp` levers).

use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::record_line_layers_rt::RECORD_LINE_LAYERS_EMPTY;
use crate::gpu::MODUL0_VK_DISPLAY::proc::display::record_frame::record_display_frame;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::{FrameRenderDefaultRtPkg, FrameSlotDefaultRtPkg};
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::RendererDefaultRtCrg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::{DeviceDefaultRtPkg, PresentationDefaultRtCrg};
use crate::ModulResult;

/// Clear-only record convenience (`bind_geometry_stp = false`).
pub fn record_clear(
    device: &DeviceDefaultRtPkg,
    presentation: &PresentationDefaultRtCrg,
    renderer: &RendererDefaultRtCrg,
    slot: &FrameSlotDefaultRtPkg,
    render_policy: &FrameRenderDefaultRtPkg,
    image_index: u32,
) -> ModulResult<()> {
    record_display_frame(
        device,
        presentation,
        renderer,
        slot,
        render_policy,
        false,
        None,
        None,
        RECORD_LINE_LAYERS_EMPTY,
        image_index,
    )
}

/// Triangle geometry record convenience (`bind_geometry_stp = true`).
pub fn record_triangle(
    device: &DeviceDefaultRtPkg,
    presentation: &PresentationDefaultRtCrg,
    renderer: &RendererDefaultRtCrg,
    slot: &FrameSlotDefaultRtPkg,
    render_policy: &FrameRenderDefaultRtPkg,
    image_index: u32,
) -> ModulResult<()> {
    record_display_frame(
        device,
        presentation,
        renderer,
        slot,
        render_policy,
        true,
        None,
        None,
        RECORD_LINE_LAYERS_EMPTY,
        image_index,
    )
}
