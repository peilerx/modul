//! vk resource: `(vk::PhysicalDevice, queue_family_index)`.

use ash::khr::surface;
use ash::vk;

use crate::gpu::MODUL0_VK_SWAPCHAIN::proc::processor::physical_device::update_physical_device_queue_family_extrl;
use crate::{map_vk, ModulResult};

/// Catalog — pick physical device + graphics/present queue family.
pub trait PhysicalDeviceSelectAuto {
    fn auto_assemble(
        instance_extrl: &ash::Instance,
        surface_loader_extrl: &surface::Instance,
        surface_extrl: vk::SurfaceKHR,
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

impl PhysicalDeviceSelectAuto for (vk::PhysicalDevice, u32) {
    fn auto_assemble(
        instance_extrl: &ash::Instance,
        surface_loader_extrl: &surface::Instance,
        surface_extrl: vk::SurfaceKHR,
    ) -> ModulResult<Self> {
        let physical_devices_extrl =
            map_vk(unsafe { instance_extrl.enumerate_physical_devices() })?;
        physical_devices_extrl
            .into_iter()
            .find_map(|physical_device_extrl| {
                update_physical_device_queue_family_extrl(
                    instance_extrl,
                    surface_loader_extrl,
                    surface_extrl,
                    physical_device_extrl,
                )
                .map(|queue_family_index_stp| (physical_device_extrl, queue_family_index_stp))
            })
            .ok_or_else(|| "No suitable physical device found".to_string())
    }
}
