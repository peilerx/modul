//! Display render ops — soa-vulkan: only `record_display_frame` (compute).

use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayDefaultRtCrg;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::record_line_layers_rt::RECORD_LINE_LAYERS_EMPTY;
use crate::gpu::MODUL0_VK_DISPLAY::proc::display::record_frame::record_display_frame;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::{FrameRenderDefaultRtPkg, FrameSlotDefaultRtPkg};
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::RendererDefaultRtCrg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::{DeviceDefaultRtPkg, PresentationDefaultRtCrg};
use crate::ModulResult;

/// Compute record convenience.
pub fn record_clear(
    device: &DeviceDefaultRtPkg,
    presentation: &PresentationDefaultRtCrg,
    renderer: &RendererDefaultRtCrg,
    slot: &FrameSlotDefaultRtPkg,
    render_policy: &FrameRenderDefaultRtPkg,
    image_index: u32,
    display: &mut DisplayDefaultRtCrg,
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
        display,
    )
}
