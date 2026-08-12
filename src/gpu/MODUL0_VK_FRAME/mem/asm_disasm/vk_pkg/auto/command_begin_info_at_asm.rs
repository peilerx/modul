use ash::vk;

use crate::gpu::MODUL0_VK_FRAME::mem::base::embedded::runtime::frame_info_res_intsct_rt::FrameCommandBeginInfoDefaultRt;

/// Catalog — auto branch; Strategy=Default (FIX-089).
pub trait CommandBeginInfoDefaultAuto {
    fn auto_assemble() -> FrameCommandBeginInfoDefaultRt;
}

impl CommandBeginInfoDefaultAuto for FrameCommandBeginInfoDefaultRt {
    fn auto_assemble() -> FrameCommandBeginInfoDefaultRt {
        Self {
            buffer_usage_flags_op: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            desc: "command_begin_info",
        }
    }
}

/// `auto_vk_cmd_begin` — function (auto vk cmd begin).
/// Auto-rank assemble/disassemble entry.
/// Belongs to: frames-in-flight MCG.
pub fn auto_vk_cmd_begin(rt: &FrameCommandBeginInfoDefaultRt) -> vk::CommandBufferBeginInfo<'_> {
    vk::CommandBufferBeginInfo::default().flags(rt.buffer_usage_flags_op)
}