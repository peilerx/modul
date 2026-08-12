use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::InstanceDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::DeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SwapchainLoaderDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk::auto::swapchain_loader_at_asm::SwapchainLoaderAuto;
use ash::khr::swapchain;

/// Catalog — pack KHR swapchain device loader (FIX-120).
pub trait SwapchainLoaderDefaultAuto {
    fn auto_assemble(
        instance_default_rt: &InstanceDefaultRt,
        device_default_rt_pkg: &DeviceDefaultRtPkg,
    ) -> SwapchainLoaderDefaultRtPkg;
}

impl SwapchainLoaderDefaultAuto for SwapchainLoaderDefaultRtPkg {
    fn auto_assemble(
        instance_default_rt: &InstanceDefaultRt,
        device_default_rt_pkg: &DeviceDefaultRtPkg,
    ) -> SwapchainLoaderDefaultRtPkg {
        Self {
            swapchain_loader_extrl: swapchain::Device::auto_assemble(
                &instance_default_rt.instance_extrl,
                &device_default_rt_pkg.device_extrl,
            ),
            desc: "swapchain_loader",
        }
    }
}
