//! Catalog — empty seed + **slot** accessors · trait `FrameBfrAuto`.
//! Type: `mem/base/embedded/buffer/frame_bfr.rs`.

use crate::gpu::MODUL0_VK_FRAME::mem::base::embedded::buffer::FrameBfr;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::FrameDefaultRtCrg;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::FrameRenderDefaultRtPkg;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::FrameSyncDefaultRtPkg;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::setup::frame_fif_default_stp_pkg::FrameFifDefaultStpPkg;
use crate::ModulResult;

/// Catalog Auto · empty seed + slot API on subject *Bfr.
pub trait FrameBfrAuto: Sized {
    fn auto_assemble() -> Self;

    fn slot_ref<'a, T>(slot: &'a Option<T>, name: &'static str) -> ModulResult<&'a T> {
        slot.as_ref()
            .ok_or_else(|| format!("frame_bfr: slot `{name}` empty"))
    }

    fn slot_take<T>(slot: &mut Option<T>, name: &'static str) -> ModulResult<T> {
        slot.take()
            .ok_or_else(|| format!("frame_bfr: slot `{name}` empty (take)"))
    }

    fn fif_stp(&self) -> ModulResult<&FrameFifDefaultStpPkg>;
    fn sync(&self) -> ModulResult<&FrameSyncDefaultRtPkg>;
    fn render(&self) -> ModulResult<&FrameRenderDefaultRtPkg>;
    fn cargo(&self) -> ModulResult<&FrameDefaultRtCrg>;
}

impl FrameBfrAuto for FrameBfr {
    fn auto_assemble() -> Self {
        Self {
            frame_fif_default_stp_pkg: None,
            frame_sync_default_rt_pkg: None,
            frame_render_default_rt_pkg: None,
            cargo_rt: None,
        }
    }

    fn fif_stp(&self) -> ModulResult<&FrameFifDefaultStpPkg> {
        Self::slot_ref(&self.frame_fif_default_stp_pkg, "frame_fif_default_stp_pkg")
    }

    fn sync(&self) -> ModulResult<&FrameSyncDefaultRtPkg> {
        Self::slot_ref(&self.frame_sync_default_rt_pkg, "frame_sync_default_rt_pkg")
    }

    fn render(&self) -> ModulResult<&FrameRenderDefaultRtPkg> {
        Self::slot_ref(
            &self.frame_render_default_rt_pkg,
            "frame_render_default_rt_pkg",
        )
    }

    fn cargo(&self) -> ModulResult<&FrameDefaultRtCrg> {
        Self::slot_ref(&self.cargo_rt, "cargo_rt")
    }
}
