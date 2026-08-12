//! `vk_crg` — pack `DisplayDefaultRtCrg` (FIX-120).

use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::{
    DisplayCommandDefaultRt, DisplayInputDefaultRtPkg, DisplayRenderDefaultRt,
    DisplayDefaultRtCrg, VulkanDisplayDefaultRt,
};

/// Catalog — pack already-built display session members.
pub trait DisplayDefaultRtCrgAuto {
    fn auto_assemble(
        display_input_default_rt_pkg: DisplayInputDefaultRtPkg,
        command_rt: DisplayCommandDefaultRt,
        display_render_default_rt: DisplayRenderDefaultRt,
        vulkan_display_default_rt: VulkanDisplayDefaultRt,
    ) -> DisplayDefaultRtCrg;
}

impl DisplayDefaultRtCrgAuto for DisplayDefaultRtCrg {
    fn auto_assemble(
        display_input_default_rt_pkg: DisplayInputDefaultRtPkg,
        command_rt: DisplayCommandDefaultRt,
        display_render_default_rt: DisplayRenderDefaultRt,
        vulkan_display_default_rt: VulkanDisplayDefaultRt,
    ) -> DisplayDefaultRtCrg {
        Self {
            display_input_default_rt_pkg,
            command_rt,
            display_render_default_rt,
            vulkan_display_default_rt,
            desc: "display_rt",
        }
    }
}
