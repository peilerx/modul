//! `FrameBfr` — atom slots + cargo (type only).
//! Slot accessors / Auto seed: `asm_disasm/vk_bfr/auto/frame_bfr_at_asm.rs`.

use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::FrameDefaultRtCrg;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::FrameRenderDefaultRtPkg;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::FrameSyncDefaultRtPkg;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::setup::frame_fif_default_stp_pkg::FrameFifDefaultStpPkg;

/// Working store: FIF setup + atom slots + packed cargo.
pub struct FrameBfr {
    /// Nested package bag field `frame_fif_default_stp_pkg`.
    pub frame_fif_default_stp_pkg: Option<FrameFifDefaultStpPkg>,
    /// Nested package bag field `frame_sync_default_rt_pkg`.
    pub frame_sync_default_rt_pkg: Option<FrameSyncDefaultRtPkg>,
    /// Nested package bag field `frame_render_default_rt_pkg`.
    pub frame_render_default_rt_pkg: Option<FrameRenderDefaultRtPkg>,
    /// Runtime phase field `cargo_rt`.
    pub cargo_rt: Option<FrameDefaultRtCrg>,
}
