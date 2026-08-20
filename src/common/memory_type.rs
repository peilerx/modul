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

/// Prefer `DEVICE_LOCAL` without `HOST_VISIBLE` (VRAM), then any `DEVICE_LOCAL`, then GTT.
#[must_use]
pub fn pick_vk_memory_type_vram_then_host(
    instance_extrl: &Instance,
    physical_device_extrl: vk::PhysicalDevice,
    type_bits: u32,
    need: vk::DeviceSize,
) -> Option<(u32, &'static str)> {
    let props =
        unsafe { instance_extrl.get_physical_device_memory_properties(physical_device_extrl) };
    let host = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
    let mut visible_vram: Option<u32> = None;
    let mut fallback_gtt: Option<u32> = None;
    for (i, mem_type) in props.memory_types.iter().enumerate() {
        let bit = 1u32 << i;
        if type_bits & bit == 0 {
            continue;
        }
        let heap = props.memory_heaps[mem_type.heap_index as usize];
        if heap.size < need {
            continue;
        }
        let flags = mem_type.property_flags;
        let Ok(idx) = u32::try_from(i) else {
            continue;
        };
        if flags.contains(vk::MemoryPropertyFlags::DEVICE_LOCAL)
            && !flags.contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
        {
            return Some((idx, "DEVICE_LOCAL VRAM"));
        }
        if flags.contains(vk::MemoryPropertyFlags::DEVICE_LOCAL) && visible_vram.is_none() {
            visible_vram = Some(idx);
        }
        if flags.contains(host) && fallback_gtt.is_none() {
            fallback_gtt = Some(idx);
        }
    }
    visible_vram
        .map(|idx| (idx, "DEVICE_LOCAL visible VRAM"))
        .or_else(|| fallback_gtt.map(|idx| (idx, "HOST_VISIBLE RAM/GTT")))
}
