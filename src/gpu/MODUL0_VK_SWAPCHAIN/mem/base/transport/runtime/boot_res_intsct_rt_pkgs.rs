//! Base **boot** resource intersection (M.BASE_RES_INTSCT · N.RES_INTSCT · FIX-120).
//!
//! Co-created device line · always packed into `SwapchainRtCrg`.
//! One file · multi-struct · co-recreate with bootstrap assemble.

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::EntryDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::InstanceDefaultRt;

/// KHR surface + loader.
pub struct SurfaceDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `surface_extrl` (`surface` peel).
    pub surface_extrl: ash::vk::SurfaceKHR,
    /// External / raw Vulkan handle or host pointer field `surface_loader_extrl` (`surface_loader` peel).
    pub surface_loader_extrl: ash::khr::surface::Instance,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// GPU pick + queue family.
pub struct PhysicalDeviceDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `physical_device_extrl` (`physical_device` peel).
    pub physical_device_extrl: ash::vk::PhysicalDevice,
    /// Runtime phase field `queue_family_index_rt`.
    pub queue_family_index_rt: u32,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Logical device + queues.
pub struct DeviceDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `device_extrl` (`device` peel).
    pub device_extrl: ash::Device,
    /// External / raw Vulkan handle or host pointer field `graphics_queue_extrl` (`graphics_queue` peel).
    pub graphics_queue_extrl: ash::vk::Queue,
    /// External / raw Vulkan handle or host pointer field `present_queue_extrl` (`present_queue` peel).
    pub present_queue_extrl: ash::vk::Queue,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Command pool for swapchain / frame command buffers.
pub struct SwapchainCommandPoolDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `command_pool_extrl` (`command_pool` peel).
    pub command_pool_extrl: ash::vk::CommandPool,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// KHR swapchain device loader.
pub struct SwapchainLoaderDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `swapchain_loader_extrl` (`swapchain_loader` peel).
    pub swapchain_loader_extrl: ash::khr::swapchain::Device,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Conv-assembled swapchain device-line cargo (ex-PLATFORM · FIX-079).
pub struct SwapchainRtCrg {
    /// Runtime phase field `entry_default_rt`.
    pub entry_default_rt: EntryDefaultRt,
    /// Runtime phase field `instance_default_rt`.
    pub instance_default_rt: InstanceDefaultRt,
    /// Nested package bag field `surface_default_rt_pkg`.
    pub surface_default_rt_pkg: SurfaceDefaultRtPkg,
    /// Nested package bag field `physical_device_default_rt_pkg`.
    pub physical_device_default_rt_pkg: PhysicalDeviceDefaultRtPkg,
    /// Nested package bag field `device_default_rt_pkg`.
    pub device_default_rt_pkg: DeviceDefaultRtPkg,
    /// Nested package bag field `swapchain_command_pool_default_rt_pkg`.
    pub swapchain_command_pool_default_rt_pkg: SwapchainCommandPoolDefaultRtPkg,
    /// Nested package bag field `swapchain_loader_default_rt_pkg`.
    pub swapchain_loader_default_rt_pkg: SwapchainLoaderDefaultRtPkg,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}
