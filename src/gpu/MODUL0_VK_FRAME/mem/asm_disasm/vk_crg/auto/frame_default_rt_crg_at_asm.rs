//! `vk_crg` — pack `FrameDefaultRtCrg` (FIX-120).

use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::{
    FrameRenderDefaultRtPkg, FrameDefaultRtCrg, FrameSyncDefaultRtPkg,
};

/// Catalog — pack already-built frame sync + render packages.
pub trait FrameDefaultRtCrgAuto {
    fn auto_assemble(
        frame_sync_default_rt_pkg: FrameSyncDefaultRtPkg,
        frame_render_default_rt_pkg: FrameRenderDefaultRtPkg,
    ) -> FrameDefaultRtCrg;
}

impl FrameDefaultRtCrgAuto for FrameDefaultRtCrg {
    fn auto_assemble(
        frame_sync_default_rt_pkg: FrameSyncDefaultRtPkg,
        frame_render_default_rt_pkg: FrameRenderDefaultRtPkg,
    ) -> FrameDefaultRtCrg {
        Self {
            frame_sync_default_rt_pkg,
            frame_render_default_rt_pkg,
            desc: "frame_rt",
        }
    }
}
