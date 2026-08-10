use ash::vk;

use crate::gpu::MODUL0_VK_FRAME::mem::base::embedded::runtime::frame_info_res_intsct_rt::SubmitInfoDefaultRt;

/// Catalog — Strategy=Default (FIX-089).
pub trait SubmitInfoDefaultAuto {
    fn auto_assemble(
        wait_semaphore_extrl: vk::Semaphore,
        command_buffer_extrl: vk::CommandBuffer,
        signal_semaphore_extrl: vk::Semaphore,
    ) -> SubmitInfoDefaultRt;
}

trait SubmitInfoDefaultWaitStagesAuto {
    fn auto_assemble() -> Vec<vk::PipelineStageFlags>;
}

trait SubmitInfoDefaultVkAuto {
    fn auto_assemble(rt: &SubmitInfoDefaultRt) -> vk::SubmitInfo<'_>;
}

impl SubmitInfoDefaultWaitStagesAuto for SubmitInfoDefaultRt {
    fn auto_assemble() -> Vec<vk::PipelineStageFlags> {
        vec![vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT]
    }
}

impl SubmitInfoDefaultAuto for SubmitInfoDefaultRt {
    fn auto_assemble(
        wait_semaphore_extrl: vk::Semaphore,
        command_buffer_extrl: vk::CommandBuffer,
        signal_semaphore_extrl: vk::Semaphore,
    ) -> SubmitInfoDefaultRt {
        SubmitInfoDefaultRt {
            wait_semaphore_extrl,
            wait_dst_stage_mask_op:
                <SubmitInfoDefaultRt as SubmitInfoDefaultWaitStagesAuto>::auto_assemble(),
            command_buffer_extrl,
            signal_semaphore_extrl,
            desc: "submit_info",
        }
    }
}

impl SubmitInfoDefaultVkAuto for SubmitInfoDefaultRt {
    fn auto_assemble(rt: &SubmitInfoDefaultRt) -> vk::SubmitInfo<'_> {
        vk::SubmitInfo::default()
            .wait_semaphores(std::slice::from_ref(&rt.wait_semaphore_extrl))
            .wait_dst_stage_mask(&rt.wait_dst_stage_mask_op)
            .command_buffers(std::slice::from_ref(&rt.command_buffer_extrl))
            .signal_semaphores(std::slice::from_ref(&rt.signal_semaphore_extrl))
    }
}

/// `auto_vk_submit` — function (auto vk submit).
/// Auto-rank assemble/disassemble entry.
/// Belongs to: frames-in-flight MCG.
#[must_use]
pub fn auto_vk_submit<'a>(rt: &'a SubmitInfoDefaultRt) -> vk::SubmitInfo<'a> {
    <SubmitInfoDefaultRt as SubmitInfoDefaultVkAuto>::auto_assemble(rt)
}