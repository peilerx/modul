//! DISPLAY transport runtime — **display_res_intsct**.
pub mod display_res_intsct_rt_pkgs;
/// Submodule `record_line_layers_rt`.
/// Part of `gpu/MODUL0_VK_DISPLAY/mem/base/transport/runtime` under the mem/conv/proc MCG canon.
pub mod record_line_layers_rt;

pub use display_res_intsct_rt_pkgs::{
    DisplayCommandDefaultRt, DisplayDefaultRtCrg, DisplayInputDefaultRtPkg, DisplayRenderDefaultRt,
    VulkanDisplayDefaultRt,
};
pub use record_line_layers_rt::RecordLineLayersRt;
