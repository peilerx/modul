//! Export peels from FrameDefaultRtCrg (base ¬ impl).

use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::{
    FrameDefaultRtCrg, FrameRenderDefaultRtPkg,
};

/// Export render-policy peel from asmed cargo.
#[must_use]
#[inline]
pub const fn frame_export_asmed_render1(crg: &FrameDefaultRtCrg) -> &FrameRenderDefaultRtPkg {
    &crg.frame_render_default_rt_pkg
}
