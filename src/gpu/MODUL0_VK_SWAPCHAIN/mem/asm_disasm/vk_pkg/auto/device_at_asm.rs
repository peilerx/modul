//! vk_pkg device — one import: **vk::device** MCU.

use ash::vk;

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::InstanceDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::DeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::PhysicalDeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk::handled::device_res_intsct_hld_asm::{
    DeviceHandled, DeviceQueueHandled,
};
use crate::ModulResult;

/// Catalog — pack logical device + queues.
pub trait DeviceDefaultAuto {
    fn auto_assemble(
        physical_device_default_rt_pkg: &PhysicalDeviceDefaultRtPkg,
        instance_default_rt: &InstanceDefaultRt,
    ) -> ModulResult<DeviceDefaultRtPkg>;
}

impl DeviceDefaultAuto for DeviceDefaultRtPkg {
    fn auto_assemble(
        physical_device_default_rt_pkg: &PhysicalDeviceDefaultRtPkg,
        instance_default_rt: &InstanceDefaultRt,
    ) -> ModulResult<DeviceDefaultRtPkg> {
        let queue_family_index_stp = physical_device_default_rt_pkg.queue_family_index_rt;
        let device_extrl = ash::Device::handled_assemble(
            &instance_default_rt.instance_extrl,
            physical_device_default_rt_pkg.physical_device_extrl,
            queue_family_index_stp,
        )?;
        let graphics_queue_extrl =
            vk::Queue::handled_assemble(&device_extrl, queue_family_index_stp, 0);
        let present_queue_extrl =
            vk::Queue::handled_assemble(&device_extrl, queue_family_index_stp, 0);
        Ok(DeviceDefaultRtPkg {
            device_extrl,
            graphics_queue_extrl,
            present_queue_extrl,
            desc: "vulkan_logical_device",
        })
    }
}
