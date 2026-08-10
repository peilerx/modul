use ash::khr::surface;
use ash::vk;

/// Queue family with graphics + present (raw vulkan inputs · used by rank **vk**).
pub(crate) fn update_physical_device_queue_family_extrl(
    instance_extrl: &ash::Instance,
    surface_loader_extrl: &surface::Instance,
    surface_extrl: vk::SurfaceKHR,
    physical_device_extrl: vk::PhysicalDevice,
) -> Option<u32> {
    let queue_family_properties_extrl =
        unsafe { instance_extrl.get_physical_device_queue_family_properties(physical_device_extrl) };
    queue_family_properties_extrl
        .iter()
        .enumerate()
        .find_map(|(index, info)| {
            let graphics = info.queue_flags.contains(vk::QueueFlags::GRAPHICS);
            let idx = u32::try_from(index).ok()?;
            let present = unsafe {
                surface_loader_extrl
                    .get_physical_device_surface_support(physical_device_extrl, idx, surface_extrl)
                    .ok()?
            };
            if graphics && present {
                Some(idx)
            } else {
                None
            }
        })
}