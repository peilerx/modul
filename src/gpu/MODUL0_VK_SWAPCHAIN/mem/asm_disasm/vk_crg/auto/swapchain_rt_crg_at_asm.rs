//! vk_crg — pack bootstrap cargo (no child creates).

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::EntryDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::InstanceDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::DeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::PhysicalDeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SurfaceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SwapchainCommandPoolDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SwapchainLoaderDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SwapchainRtCrg;

/// Catalog — pack already-materialized atoms into `SwapchainRtCrg`.
pub trait SwapchainRtCrgAuto {
    fn auto_assemble(
        entry_default_rt: EntryDefaultRt,
        instance_default_rt: InstanceDefaultRt,
        surface_default_rt_pkg: SurfaceDefaultRtPkg,
        physical_device_default_rt_pkg: PhysicalDeviceDefaultRtPkg,
        device_default_rt_pkg: DeviceDefaultRtPkg,
        swapchain_command_pool_default_rt_pkg: SwapchainCommandPoolDefaultRtPkg,
        swapchain_loader_default_rt_pkg: SwapchainLoaderDefaultRtPkg,
    ) -> SwapchainRtCrg;
}

impl SwapchainRtCrgAuto for SwapchainRtCrg {
    fn auto_assemble(
        entry_default_rt: EntryDefaultRt,
        instance_default_rt: InstanceDefaultRt,
        surface_default_rt_pkg: SurfaceDefaultRtPkg,
        physical_device_default_rt_pkg: PhysicalDeviceDefaultRtPkg,
        device_default_rt_pkg: DeviceDefaultRtPkg,
        swapchain_command_pool_default_rt_pkg: SwapchainCommandPoolDefaultRtPkg,
        swapchain_loader_default_rt_pkg: SwapchainLoaderDefaultRtPkg,
    ) -> SwapchainRtCrg {
        SwapchainRtCrg {
            entry_default_rt,
            instance_default_rt,
            surface_default_rt_pkg,
            physical_device_default_rt_pkg,
            device_default_rt_pkg,
            swapchain_command_pool_default_rt_pkg,
            swapchain_loader_default_rt_pkg,
            desc: "swapchain_rt_crg",
        }
    }
}
