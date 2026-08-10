use ash::vk;

use crate::ModulResult;

/// `update_framebuffer_attachment_layout` — function (update framebuffer attachment layout).
/// Public API entry for this module.
/// Belongs to: swapchain / device bootstrap MCG.
pub(crate) fn update_framebuffer_attachment_layout<F, G>(
    sample_count_op: vk::SampleCountFlags,
    simple_stp: F,
    msaa_stp: G,
) -> ModulResult<Vec<vk::ImageView>>
where
    F: FnOnce() -> Vec<vk::ImageView>,
    G: FnOnce() -> ModulResult<Vec<vk::ImageView>>,
{
    if sample_count_op == vk::SampleCountFlags::TYPE_1 {
        Ok(simple_stp())
    } else {
        msaa_stp()
    }
}