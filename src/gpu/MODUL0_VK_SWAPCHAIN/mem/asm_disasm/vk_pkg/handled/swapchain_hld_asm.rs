//! vk_pkg **handled** — KHR swapchain product from **package fields** + Intent|*Stp.
//!
//! ¬ free extrl soup in port · pass TransportMem packages · Intent → DirectVk inside.
//! Handled Stp path: every `vk::Format` / `PresentModeKHR` already on `SwapchainDefaultStpPkg`.

use ash::vk;

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk::handled::swapchain_khr_hld_asm::SwapchainKhrHandled;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_pkg::auto::swapchain_prt_at_asm::swapchain_prt_format_present;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::prt::swapchain_prt::SwapchainPrt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::PhysicalDeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SurfaceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SwapchainLoaderDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SwapchainDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::setup::present_res_intsct_stp_pkgs::SwapchainDefaultStpPkg;
use crate::ModulResult;

/// Catalog — materialize KHR via **vk** rank · packages in · Intent or *Stp in.
pub trait SwapchainDefaultHandled {
    /// Auto/PortMatch path · `SwapchainPrt` → DirectVk format + present mode.
    fn handled_assemble(
        surface_default_rt_pkg: &SurfaceDefaultRtPkg,
        physical_device_default_rt_pkg: &PhysicalDeviceDefaultRtPkg,
        swapchain_loader_default_rt_pkg: &SwapchainLoaderDefaultRtPkg,
        present_intent: SwapchainPrt,
        extent_width_stp: u32,
        extent_height_stp: u32,
    ) -> ModulResult<SwapchainDefaultRtPkg>;

    /// Handled path · full `SwapchainDefaultStpPkg` (vk::Format · PresentModeKHR · extent).
    fn handled_assemble_from_stp(
        surface_default_rt_pkg: &SurfaceDefaultRtPkg,
        physical_device_default_rt_pkg: &PhysicalDeviceDefaultRtPkg,
        swapchain_loader_default_rt_pkg: &SwapchainLoaderDefaultRtPkg,
        swapchain_default_stp_pkg: &SwapchainDefaultStpPkg,
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
        let (surface_format_op, present_mode_op, desc) =
            swapchain_prt_format_present(present_intent);
        Self::handled_assemble_from_stp(
            surface_default_rt_pkg,
            physical_device_default_rt_pkg,
            swapchain_loader_default_rt_pkg,
            &SwapchainDefaultStpPkg {
                extent_width_stp,
                extent_height_stp,
                surface_format_op,
                present_mode_op,
                image_usage_op: vk::ImageUsageFlags::COLOR_ATTACHMENT,
                composite_alpha_op: vk::CompositeAlphaFlagsKHR::OPAQUE,
                desc,
            },
        )
    }

    fn handled_assemble_from_stp(
        surface_default_rt_pkg: &SurfaceDefaultRtPkg,
        physical_device_default_rt_pkg: &PhysicalDeviceDefaultRtPkg,
        swapchain_loader_default_rt_pkg: &SwapchainLoaderDefaultRtPkg,
        swapchain_default_stp_pkg: &SwapchainDefaultStpPkg,
    ) -> ModulResult<SwapchainDefaultRtPkg> {
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
            swapchain_default_stp_pkg.extent_width_stp,
            swapchain_default_stp_pkg.extent_height_stp,
            swapchain_default_stp_pkg.surface_format_op,
            swapchain_default_stp_pkg.present_mode_op,
            swapchain_default_stp_pkg.image_usage_op,
            swapchain_default_stp_pkg.composite_alpha_op,
        )?;
        Ok(Self {
            surface_format_op,
            extent_rt,
            swapchain_extrl,
            images_extrl,
            desc: swapchain_default_stp_pkg.desc,
        })
    }
}
