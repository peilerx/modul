use ash::vk;

use crate::ModulResult;

/// `update_repeat_lanes` — function (update repeat lanes).
/// Public API entry for this module.
/// Belongs to: swapchain / device bootstrap MCG.
pub(crate) fn update_repeat_lanes<F>(
    image_count_stp: usize,
    mut lane_stp: F,
) -> ModulResult<(Vec<vk::Image>, Vec<vk::DeviceMemory>, Vec<vk::ImageView>)>
where
    F: FnMut() -> ModulResult<(vk::Image, vk::DeviceMemory, vk::ImageView)>,
{
    let mut images_extrl = Vec::with_capacity(image_count_stp);
    let mut device_memories_extrl = Vec::with_capacity(image_count_stp);
    let mut image_views_extrl = Vec::with_capacity(image_count_stp);
    for _ in 0..image_count_stp {
        let (image_extrl, memory_extrl, view_extrl) = lane_stp()?;
        images_extrl.push(image_extrl);
        device_memories_extrl.push(memory_extrl);
        image_views_extrl.push(view_extrl);
    }
    Ok((images_extrl, device_memories_extrl, image_views_extrl))
}