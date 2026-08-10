//! `DisplayBfr` — atom slots + cargo (type only).

use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayCommandDefaultRt;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayDefaultRtCrg;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayInputDefaultRtPkg;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayRenderDefaultRt;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::VulkanDisplayDefaultRt;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::setup::display_present_default_stp_pkg::DisplayPresentDefaultStpPkg;

/// `DisplayBfr` — buffer / warehouse bag (display bfr).
/// Memory-layer bag: owned fields, no product control flow.
/// Belongs to: command-buffer record / display MCG.
/// Module path context: `gpu/MODUL0_VK_DISPLAY/mem/base/embedded/buffer`.
pub struct DisplayBfr {
    /// Nested package bag field `display_present_default_stp_pkg`.
    pub display_present_default_stp_pkg: Option<DisplayPresentDefaultStpPkg>,
    /// Nested package bag field `display_input_default_rt_pkg`.
    pub display_input_default_rt_pkg: Option<DisplayInputDefaultRtPkg>,
    /// Runtime phase field `command_rt`.
    pub command_rt: Option<DisplayCommandDefaultRt>,
    /// Runtime phase field `display_render_default_rt`.
    pub display_render_default_rt: Option<DisplayRenderDefaultRt>,
    /// Runtime phase field `vulkan_display_default_rt`.
    pub vulkan_display_default_rt: Option<VulkanDisplayDefaultRt>,
    /// Runtime phase field `cargo_rt`.
    pub cargo_rt: Option<DisplayDefaultRtCrg>,
}
