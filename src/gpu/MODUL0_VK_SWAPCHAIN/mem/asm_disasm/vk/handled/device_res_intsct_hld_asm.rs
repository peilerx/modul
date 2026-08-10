//! MCU **device** — `ash::Device` + `vk::Queue` (local only · FIX-120).

use ash::khr::swapchain;
use ash::vk;

use crate::{map_vk, ModulResult};

/// Catalog — logical device (swapchain extension · one queue family).
pub trait DeviceHandled {
    fn handled_assemble(
        instance_extrl: &ash::Instance,
        physical_device_extrl: vk::PhysicalDevice,
        queue_family_index_stp: u32,
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

/// Catalog — device queue by family + index.
pub trait DeviceQueueHandled {
    fn handled_assemble(
        device_extrl: &ash::Device,
        queue_family_index_stp: u32,
        queue_index_stp: u32,
    ) -> Self;
}

impl DeviceHandled for ash::Device {
    fn handled_assemble(
        instance_extrl: &ash::Instance,
        physical_device_extrl: vk::PhysicalDevice,
        queue_family_index_stp: u32,
    ) -> ModulResult<Self> {
        let queue_priorities_stp = [1.0f32];
        let queue_create_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index_stp)
            .queue_priorities(&queue_priorities_stp);
        let extension_names = [swapchain::NAME.as_ptr()];
        let device_create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(std::slice::from_ref(&queue_create_info))
            .enabled_extension_names(&extension_names);
        map_vk(unsafe {
            instance_extrl.create_device(physical_device_extrl, &device_create_info, None)
        })
    }
}

impl DeviceQueueHandled for vk::Queue {
    fn handled_assemble(
        device_extrl: &ash::Device,
        queue_family_index_stp: u32,
        queue_index_stp: u32,
    ) -> Self {
        unsafe { device_extrl.get_device_queue(queue_family_index_stp, queue_index_stp) }
    }
}
