use ash::vk;

use crate::gpu::MODUL0_VK_FRAME::mem::base::embedded::runtime::frame_info_res_intsct_rt::PresentInfoDefaultRt;

/// Catalog — Strategy=Default (FIX-089).
pub trait PresentInfoDefaultAuto {
    fn auto_assemble(
        wait_semaphore_extrl: vk::Semaphore,
        swapchain_extrl: vk::SwapchainKHR,
        image_index_rt: u32,
    ) -> PresentInfoDefaultRt;
}

trait PresentInfoDefaultVkAuto {
    fn auto_assemble(rt: &PresentInfoDefaultRt) -> vk::PresentInfoKHR<'_>;
}

impl PresentInfoDefaultAuto for PresentInfoDefaultRt {
    fn auto_assemble(
        wait_semaphore_extrl: vk::Semaphore,
        swapchain_extrl: vk::SwapchainKHR,
        image_index_rt: u32,
    ) -> PresentInfoDefaultRt {
        PresentInfoDefaultRt {
            wait_semaphore_extrl,
            swapchain_extrl,
            image_index_rt: image_index_rt,
            desc: "present_info",
        }
    }
}

impl PresentInfoDefaultVkAuto for PresentInfoDefaultRt {
    fn auto_assemble(rt: &PresentInfoDefaultRt) -> vk::PresentInfoKHR<'_> {
        vk::PresentInfoKHR::default()
            .wait_semaphores(std::slice::from_ref(&rt.wait_semaphore_extrl))
            .swapchains(std::slice::from_ref(&rt.swapchain_extrl))
            .image_indices(std::slice::from_ref(&rt.image_index_rt))
    }
}

/// `auto_vk_present` — function (auto vk present).
/// Auto-rank assemble/disassemble entry.
/// Belongs to: frames-in-flight MCG.
#[must_use]
pub fn auto_vk_present<'a>(rt: &'a PresentInfoDefaultRt) -> vk::PresentInfoKHR<'a> {
    <PresentInfoDefaultRt as PresentInfoDefaultVkAuto>::auto_assemble(rt)
}