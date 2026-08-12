//! Export peels from `FrameDefaultRtCrg` (no Exportable trait · FIX-130).

use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::FrameDefaultRtCrg;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::FrameRenderDefaultRtPkg;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::FrameFifPrt;

/// Export render policy peel (clear color cache).
#[must_use]
pub const fn export_asmed_frame_render(frame_default_rt_crg: &FrameDefaultRtCrg) -> FrameRenderDefaultRtPkg {
    FrameRenderDefaultRtPkg {
        clear_color_rt: frame_default_rt_crg.frame_render_default_rt_pkg.clear_color_rt,
        desc: frame_default_rt_crg.frame_render_default_rt_pkg.desc,
    }
}

/// FIF count peel from picture (barter-export · returns value · FIX-128).
#[must_use]
pub const fn export_frame_fif_count(frame_fif_prt: FrameFifPrt) -> u32 {
    match frame_fif_prt {
        FrameFifPrt::TripleBuffered => 3,
        FrameFifPrt::DoubleBuffered => 2,
        FrameFifPrt::SingleBuffered => 1,
    }
}
