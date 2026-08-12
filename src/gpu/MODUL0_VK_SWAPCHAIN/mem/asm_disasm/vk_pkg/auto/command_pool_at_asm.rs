use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::DeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::PhysicalDeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SwapchainCommandPoolDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk::handled::command_pool_hld_asm::CommandPoolHandled;
use crate::ModulResult;
use ash::vk;

/// Catalog — pack `vk::CommandPool` (FIX-120).
pub trait CommandPoolDefaultAuto {
    fn auto_assemble(
        device_default_rt_pkg: &DeviceDefaultRtPkg,
        physical_device_default_rt_pkg: &PhysicalDeviceDefaultRtPkg,
    ) -> ModulResult<SwapchainCommandPoolDefaultRtPkg>;
}

impl CommandPoolDefaultAuto for SwapchainCommandPoolDefaultRtPkg {
    fn auto_assemble(
        device_default_rt_pkg: &DeviceDefaultRtPkg,
        physical_device_default_rt_pkg: &PhysicalDeviceDefaultRtPkg,
    ) -> ModulResult<SwapchainCommandPoolDefaultRtPkg> {
        let command_pool_extrl = vk::CommandPool::handled_assemble(
            &device_default_rt_pkg.device_extrl,
            physical_device_default_rt_pkg.queue_family_index_rt,
        )?;
        Ok(Self {
            command_pool_extrl,
            desc: "vulkan_command_pool",
        })
    }
}
