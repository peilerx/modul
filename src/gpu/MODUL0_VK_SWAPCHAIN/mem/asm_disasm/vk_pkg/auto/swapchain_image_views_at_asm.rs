//! vk_pkg swapchain image views — one import: **vk::image** MCU (`ImageViewHandled`).

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::DeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SwapchainDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SwapchainImageViewsDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk::handled::image_res_intsct_hld_asm::ImageViewHandled;
use crate::ModulResult;
use ash::vk;

/// Catalog — pack image views for swapchain images.
pub trait SwapchainImageViewsDefaultAuto {
    fn auto_assemble(
        device_default_rt_pkg: &DeviceDefaultRtPkg,
        swapchain_default_rt_pkg: &SwapchainDefaultRtPkg,
    ) -> ModulResult<SwapchainImageViewsDefaultRtPkg>;
}

impl SwapchainImageViewsDefaultAuto for SwapchainImageViewsDefaultRtPkg {
    fn auto_assemble(
        device_default_rt_pkg: &DeviceDefaultRtPkg,
        swapchain_default_rt_pkg: &SwapchainDefaultRtPkg,
    ) -> ModulResult<SwapchainImageViewsDefaultRtPkg> {
        let format_op = swapchain_default_rt_pkg.surface_format_op.format;
        let image_views_extrl = swapchain_default_rt_pkg
            .images_extrl
            .iter()
            .map(|&image_extrl| {
                vk::ImageView::handled_assemble(
                    &device_default_rt_pkg.device_extrl,
                    image_extrl,
                    format_op,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(SwapchainImageViewsDefaultRtPkg {
            image_views_extrl,
            desc: "swapchain_image_views",
        })
    }
}
