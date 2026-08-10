//! vk_crg **handled** — pack frame cargo from **bfr slots** (takes inside).
//! Atoms: FrameSync handled · FrameRender auto · ordered in port factory-line.

use crate::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_bfr::auto::frame_bfr_at_asm::FrameBfrAuto;
use crate::gpu::MODUL0_VK_FRAME::mem::base::embedded::buffer::FrameBfr;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::FrameDefaultRtCrg;
use crate::ModulResult;

/// Catalog — pack multi-pkg frame cargo from bfr (asm pack step).
pub trait FrameDefaultRtCrgHandled {
    fn handled_assemble(bfr: &mut FrameBfr) -> ModulResult<FrameDefaultRtCrg>;
}

impl FrameDefaultRtCrgHandled for FrameDefaultRtCrg {
    fn handled_assemble(bfr: &mut FrameBfr) -> ModulResult<FrameDefaultRtCrg> {
        Ok(FrameDefaultRtCrg {
            frame_sync_default_rt_pkg: <FrameBfr as FrameBfrAuto>::slot_take(
                &mut bfr.frame_sync_default_rt_pkg,
                "frame_sync_default_rt_pkg",
            )?,
            frame_render_default_rt_pkg: <FrameBfr as FrameBfrAuto>::slot_take(
                &mut bfr.frame_render_default_rt_pkg,
                "frame_render_default_rt_pkg",
            )?,
            desc: "frame_rt",
        })
    }
}
