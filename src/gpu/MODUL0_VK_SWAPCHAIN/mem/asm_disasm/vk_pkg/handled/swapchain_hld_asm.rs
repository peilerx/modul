//! vk_pkg **handled** — KHR swapchain product from **package fields** + Intent.
//!
//! ¬ free extrl soup in port · pass TransportMem packages · Intent → DirectVk inside.

use ash::vk;

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk::handled::swapchain_khr_hld_asm::SwapchainKhrHandled;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::prt::swapchain_prt::SwapchainPrt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::PhysicalDeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SurfaceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SwapchainLoaderDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SwapchainDefaultRtPkg;
use crate::ModulResult;

/// Catalog — materialize KHR via **vk** rank · packages in · Intent in.
pub trait SwapchainDefaultHandled {
    fn handled_assemble(
        surface_default_rt_pkg: &SurfaceDefaultRtPkg,
        physical_device_default_rt_pkg: &PhysicalDeviceDefaultRtPkg,
        swapchain_loader_default_rt_pkg: &SwapchainLoaderDefaultRtPkg,
        present_intent: SwapchainPrt,
        extent_width_stp: u32,
        extent_height_stp: u32,
    ) -> ModulResult<SwapchainDefaultRtPkg>;
}

impl SwapchainDefaultHandled for SwapchainDefaultRtPkg {
    fn handled_assemble(
        surface_default_rt_pkg: &SurfaceDefaultRtPkg,
        physical_device_default_rt_pkg: &PhysicalDeviceDefaultRtPkg,
        swapchain_loader_default_rt_pkg: &SwapchainLoaderDefaultRtPkg,
        present_intent: SwapchainPrt,
        extent_width_stp: u32,
        extent_height_stp: u32,
    ) -> ModulResult<SwapchainDefaultRtPkg> {
        let (surface_format_op, present_mode_op, desc) = match present_intent {
            SwapchainPrt::SrgbMailbox => (
                vk::Format::B8G8R8A8_SRGB,
                vk::PresentModeKHR::MAILBOX,
                "swapchain_srgb_mailbox",
            ),
            SwapchainPrt::SrgbFifo => (
                vk::Format::B8G8R8A8_SRGB,
                vk::PresentModeKHR::FIFO,
                "swapchain_srgb_fifo",
            ),
            SwapchainPrt::UnormMailbox => (
                vk::Format::B8G8R8A8_UNORM,
                vk::PresentModeKHR::MAILBOX,
                "swapchain_unorm_mailbox",
            ),
        };

        let (swapchain_extrl, images_extrl, surface_format_op, extent_rt) = <(
            vk::SwapchainKHR,
            Vec<vk::Image>,
            vk::SurfaceFormatKHR,
            vk::Extent2D,
        ) as SwapchainKhrHandled>::handled_assemble(
            surface_default_rt_pkg.surface_extrl,
            &surface_default_rt_pkg.surface_loader_extrl,
            physical_device_default_rt_pkg.physical_device_extrl,
            &swapchain_loader_default_rt_pkg.swapchain_loader_extrl,
            extent_width_stp,
            extent_height_stp,
            surface_format_op,
            present_mode_op,
        )?;
        Ok(SwapchainDefaultRtPkg {
            surface_format_op,
            extent_rt,
            swapchain_extrl,
            images_extrl,
            desc,
        })
    }
}
