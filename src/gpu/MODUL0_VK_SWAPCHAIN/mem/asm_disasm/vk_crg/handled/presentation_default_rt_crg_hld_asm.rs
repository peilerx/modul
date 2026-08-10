//! vk_crg **handled** — pack presentation cargo from **bfr slots** (takes inside).

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_bfr::auto::presentation_bfr_at_asm::PresentationBfrAuto;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::buffer::PresentationBfr;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::PresentationDefaultRtCrg;
use crate::ModulResult;

/// `PresentationDefaultRtCrgHandled` — trait (presentation default rt crg handled).
/// Handled-assemble catalog trait: construction from imported knobs / peels (FIX-129 Handled rank).
/// Belongs to: swapchain / device bootstrap MCG.
/// Module path context: `gpu/MODUL0_VK_SWAPCHAIN/mem/asm_disasm/vk_crg/handled`.
pub trait PresentationDefaultRtCrgHandled {
    fn handled_assemble(bfr: &mut PresentationBfr) -> ModulResult<PresentationDefaultRtCrg>;
}

impl PresentationDefaultRtCrgHandled for PresentationDefaultRtCrg {
    fn handled_assemble(bfr: &mut PresentationBfr) -> ModulResult<PresentationDefaultRtCrg> {
        Ok(PresentationDefaultRtCrg {
            swapchain_default_rt_pkg: <PresentationBfr as PresentationBfrAuto>::slot_take(
                &mut bfr.swapchain_default_rt_pkg,
                "swapchain_default_rt_pkg",
            )?,
            swapchain_image_views_default_rt_pkg: <PresentationBfr as PresentationBfrAuto>::slot_take(
                &mut bfr.swapchain_image_views_default_rt_pkg,
                "swapchain_image_views_default_rt_pkg",
            )?,
            sample_count_default_rt_pkg: <PresentationBfr as PresentationBfrAuto>::slot_take(
                &mut bfr.sample_count_default_rt_pkg,
                "sample_count_default_rt_pkg",
            )?,
            depth_images_default_rt_pkg: <PresentationBfr as PresentationBfrAuto>::slot_take(
                &mut bfr.depth_images_default_rt_pkg,
                "depth_images_default_rt_pkg",
            )?,
            msaa_color_default_rt_pkg: <PresentationBfr as PresentationBfrAuto>::slot_take(
                &mut bfr.msaa_color_default_rt_pkg,
                "msaa_color_default_rt_pkg",
            )?,
            framebuffer_default_rt_pkg: <PresentationBfr as PresentationBfrAuto>::slot_take(
                &mut bfr.framebuffer_default_rt_pkg,
                "framebuffer_default_rt_pkg",
            )?,
            desc: "presentation_lane",
        })
    }
}
