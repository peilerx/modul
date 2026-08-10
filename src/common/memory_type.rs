//! Physical device memory type selection.

use ash::vk;
use ash::Instance;

/// Find a memory type index matching `type_filter` and required `properties`.
///
/// Returns `None` if no type satisfies both the filter bits and property flags
/// (e.g. `HOST_VISIBLE | HOST_COHERENT` for staging).
#[must_use]
pub fn find_vk_memory_type(
    instance_extrl: &Instance,
    physical_device_extrl: vk::PhysicalDevice,
    type_filter: u32,
    properties: vk::MemoryPropertyFlags,
) -> Option<u32> {
    let mem_properties =
        unsafe { instance_extrl.get_physical_device_memory_properties(physical_device_extrl) };
    for (i, mem_type) in mem_properties.memory_types.iter().enumerate() {
        if (type_filter & (1 << i)) != 0 && mem_type.property_flags.contains(properties) {
            return u32::try_from(i).ok();
        }
    }
    None
}
