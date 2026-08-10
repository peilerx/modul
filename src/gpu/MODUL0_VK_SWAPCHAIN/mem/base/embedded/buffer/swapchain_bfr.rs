//! `SwapchainBfr` — atom slots + cargo/present products (type only).
//! Slot accessors / Auto seed: `asm_disasm/vk_bfr/auto/swapchain_bfr_at_asm.rs`.

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::EntryDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::InstanceDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::DeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::PhysicalDeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SurfaceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SwapchainCommandPoolDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SwapchainLoaderDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SwapchainRtCrg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SwapchainDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::setup::boot_res_intsct_stp_pkgs::SurfaceWindowStpPkg;

/// Working store: atom slots (asm state) + packed cargo + present product.
pub struct SwapchainBfr {
    /// Nested package bag field `surface_window_stp_pkg`.
    pub surface_window_stp_pkg: Option<SurfaceWindowStpPkg>,
    /// Runtime phase field `entry_default_rt`.
    pub entry_default_rt: Option<EntryDefaultRt>,
    /// Runtime phase field `instance_default_rt`.
    pub instance_default_rt: Option<InstanceDefaultRt>,
    /// Nested package bag field `surface_default_rt_pkg`.
    pub surface_default_rt_pkg: Option<SurfaceDefaultRtPkg>,
    /// Nested package bag field `physical_device_default_rt_pkg`.
    pub physical_device_default_rt_pkg: Option<PhysicalDeviceDefaultRtPkg>,
    /// Nested package bag field `device_default_rt_pkg`.
    pub device_default_rt_pkg: Option<DeviceDefaultRtPkg>,
    /// Nested package bag field `swapchain_command_pool_default_rt_pkg`.
    pub swapchain_command_pool_default_rt_pkg: Option<SwapchainCommandPoolDefaultRtPkg>,
    /// Nested package bag field `swapchain_loader_default_rt_pkg`.
    pub swapchain_loader_default_rt_pkg: Option<SwapchainLoaderDefaultRtPkg>,
    /// Runtime phase field `cargo_rt`.
    pub cargo_rt: Option<SwapchainRtCrg>,
    /// Nested package bag field `swapchain_default_rt_pkg`.
    pub swapchain_default_rt_pkg: Option<SwapchainDefaultRtPkg>,
}
