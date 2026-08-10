//! vk resource: `ash::khr::swapchain::Device`.

use ash::khr::swapchain;

/// Catalog — KHR swapchain device loader.
pub trait SwapchainLoaderAuto {
    fn auto_assemble(instance_extrl: &ash::Instance, device_extrl: &ash::Device) -> Self;
}

impl SwapchainLoaderAuto for swapchain::Device {
    fn auto_assemble(instance_extrl: &ash::Instance, device_extrl: &ash::Device) -> Self {
        swapchain::Device::new(instance_extrl, device_extrl)
    }
}
