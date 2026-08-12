use ash::vk;

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SampleCountDefaultRtPkg;

/// Catalog — conv unpacks `sample_count_op` (FIX-090/097).
pub trait SampleCountDefaultHandled {
    fn handled_assemble(sample_count_op: vk::SampleCountFlags) -> SampleCountDefaultRtPkg;
}

impl SampleCountDefaultHandled for SampleCountDefaultRtPkg {
    fn handled_assemble(sample_count_op: vk::SampleCountFlags) -> SampleCountDefaultRtPkg {
        Self {
            sample_count_op,
            desc: "sample_count",
        }
    }
}