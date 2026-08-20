//! `vk_crg` — pack `DisplayDefaultRtCrg` (FIX-120).

use ash::vk;

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
            soa_color_image_extrl: vk::Image::null(),
            soa_color_memory_extrl: vk::DeviceMemory::null(),
            soa_color_view_extrl: vk::ImageView::null(),
            soa_color_extent_rt: vk::Extent2D::default(),
            soa_heat_buffer_extrl: vk::Buffer::null(),
            soa_heat_memory_extrl: vk::DeviceMemory::null(),
            soa_heat_bytes_rt: 0,
            soa_heat_cleared_rt: false,
            heat_mouse_x_rt: 0.0,
            heat_mouse_y_rt: 0.0,
            heat_dt_rt: 1.0 / 60.0,
            heat_hold_rt: 0.0,
            heat_paint_rt: 0,
            heat_run_rt: 0,
            desc: "display_rt",
        }
    }
}
