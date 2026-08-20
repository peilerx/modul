use std::ffi::CStr;

use ash::khr::surface;
use ash::vk;

/// Queue family with graphics + compute + present (raw vulkan inputs · used by rank **vk**).
///
/// `vkCmdDispatch` is recorded on this same `VkQueue`. A family with GRAPHICS
/// but no COMPUTE cannot legally run the soa-vulkan heat path.
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
            let compute = info.queue_flags.contains(vk::QueueFlags::COMPUTE);
            let idx = u32::try_from(index).ok()?;
            let present = unsafe {
                surface_loader_extrl
                    .get_physical_device_surface_support(physical_device_extrl, idx, surface_extrl)
                    .ok()?
            };
            if graphics && compute && present {
                Some(idx)
            } else {
                None
            }
        })
}

const fn physical_device_type_rank(ty: vk::PhysicalDeviceType) -> u32 {
    match ty {
        vk::PhysicalDeviceType::DISCRETE_GPU => 0,
        vk::PhysicalDeviceType::INTEGRATED_GPU => 1,
        vk::PhysicalDeviceType::VIRTUAL_GPU => 2,
        vk::PhysicalDeviceType::OTHER => 3,
        vk::PhysicalDeviceType::CPU => 4,
        _ => 5,
    }
}

fn physical_device_name(props: &vk::PhysicalDeviceProperties) -> String {
    unsafe { CStr::from_ptr(props.device_name.as_ptr()) }
        .to_string_lossy()
        .into_owned()
}

fn physical_device_local_heap_bytes(
    instance_extrl: &ash::Instance,
    physical_device_extrl: vk::PhysicalDevice,
) -> u64 {
    let mem = unsafe { instance_extrl.get_physical_device_memory_properties(physical_device_extrl) };
    (0..mem.memory_heap_count as usize)
        .map(|i| mem.memory_heaps[i])
        .filter(|heap| heap.flags.contains(vk::MemoryHeapFlags::DEVICE_LOCAL))
        .map(|heap| heap.size)
        .max()
        .unwrap_or(0)
}

/// Pick a presentable GPU. Discrete first, then largest DEVICE_LOCAL heap.
///
/// `CUBES_GPU` override:
/// `0`/`1`/… = index in `vkEnumeratePhysicalDevices`;
/// `discrete` = only `DISCRETE_GPU`;
/// otherwise a case-insensitive substring of `deviceName` (`nvidia`, `amd`, `intel`).
pub(crate) fn pick_physical_device_queue_family_extrl(
    instance_extrl: &ash::Instance,
    surface_loader_extrl: &surface::Instance,
    surface_extrl: vk::SurfaceKHR,
    physical_devices_extrl: &[vk::PhysicalDevice],
) -> Option<(vk::PhysicalDevice, u32)> {
    struct Cand {
        index: usize,
        device: vk::PhysicalDevice,
        queue: u32,
        rank: u32,
        heap: u64,
        name: String,
        ty: vk::PhysicalDeviceType,
    }
    let mut candidates = Vec::new();
    for (index, &physical_device_extrl) in physical_devices_extrl.iter().enumerate() {
        let Some(queue) = update_physical_device_queue_family_extrl(
            instance_extrl,
            surface_loader_extrl,
            surface_extrl,
            physical_device_extrl,
        ) else {
            continue;
        };
        let props = unsafe { instance_extrl.get_physical_device_properties(physical_device_extrl) };
        candidates.push(Cand {
            index,
            device: physical_device_extrl,
            queue,
            rank: physical_device_type_rank(props.device_type),
            heap: physical_device_local_heap_bytes(instance_extrl, physical_device_extrl),
            name: physical_device_name(&props),
            ty: props.device_type,
        });
    }
    if candidates.is_empty() {
        return None;
    }

    if let Ok(want) = std::env::var("CUBES_GPU") {
        let needle = want.trim();
        if !needle.is_empty() {
            if let Ok(idx) = needle.parse::<usize>() {
                return candidates
                    .iter()
                    .find(|c| c.index == idx)
                    .map(|c| (c.device, c.queue));
            }
            let low = needle.to_ascii_lowercase();
            if low == "discrete" {
                let mut discrete: Vec<&Cand> = candidates
                    .iter()
                    .filter(|c| c.ty == vk::PhysicalDeviceType::DISCRETE_GPU)
                    .collect();
                if !discrete.is_empty() {
                    discrete.sort_by(|a, b| b.heap.cmp(&a.heap).then_with(|| a.index.cmp(&b.index)));
                    return discrete.first().map(|c| (c.device, c.queue));
                }
            } else if let Some(hit) = candidates
                .iter()
                .find(|c| c.name.to_ascii_lowercase().contains(&low))
            {
                return Some((hit.device, hit.queue));
            }
        }
    }

    candidates.sort_by(|a, b| {
        a.rank
            .cmp(&b.rank)
            .then_with(|| b.heap.cmp(&a.heap))
            .then_with(|| a.index.cmp(&b.index))
    });
    candidates.first().map(|c| (c.device, c.queue))
}