//! Catalog — empty seed + slots · trait `PresentationBfrAuto`.

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::buffer::PresentationBfr;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::DepthImagesDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::FramebufferDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::MsaaColorDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::PresentationDefaultRtCrg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SampleCountDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SwapchainDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SwapchainImageViewsDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::setup::present_res_intsct_stp_pkgs::PresentationDefaultStpPkg;
use crate::ModulResult;

/// `PresentationBfrAuto` — trait (presentation bfr auto).
///
/// Auto-assemble catalog trait: pure construction without external peels (FIX-129 Auto rank).
/// Belongs to: swapchain / device bootstrap MCG.
/// Module path context: `gpu/MODUL0_VK_SWAPCHAIN/mem/asm_disasm/vk_bfr/auto`.
pub trait PresentationBfrAuto: Sized {
    fn auto_assemble() -> Self;

    fn slot_ref<'a, T>(slot: &'a Option<T>, name: &'static str) -> ModulResult<&'a T> {
        slot.as_ref()
            .ok_or_else(|| format!("presentation_bfr: slot `{name}` empty"))
    }

    fn slot_take<T>(slot: &mut Option<T>, name: &'static str) -> ModulResult<T> {
        slot.take()
            .ok_or_else(|| format!("presentation_bfr: slot `{name}` empty (take)"))
    }

    fn stp(&self) -> ModulResult<&PresentationDefaultStpPkg>;
    fn khr(&self) -> ModulResult<&SwapchainDefaultRtPkg>;
    fn views(&self) -> ModulResult<&SwapchainImageViewsDefaultRtPkg>;
    fn sample(&self) -> ModulResult<&SampleCountDefaultRtPkg>;
    fn depth(&self) -> ModulResult<&DepthImagesDefaultRtPkg>;
    fn msaa(&self) -> ModulResult<&MsaaColorDefaultRtPkg>;
    fn framebuffer(&self) -> ModulResult<&FramebufferDefaultRtPkg>;
    fn cargo(&self) -> ModulResult<&PresentationDefaultRtCrg>;
}

impl PresentationBfrAuto for PresentationBfr {
    fn auto_assemble() -> Self {
        Self {
            presentation_default_stp_pkg: None,
            swapchain_default_rt_pkg: None,
            swapchain_image_views_default_rt_pkg: None,
            sample_count_default_rt_pkg: None,
            depth_images_default_rt_pkg: None,
            msaa_color_default_rt_pkg: None,
            framebuffer_default_rt_pkg: None,
            cargo_rt: None,
        }
    }

    fn stp(&self) -> ModulResult<&PresentationDefaultStpPkg> {
        Self::slot_ref(
            &self.presentation_default_stp_pkg,
            "presentation_default_stp_pkg",
        )
    }
    fn khr(&self) -> ModulResult<&SwapchainDefaultRtPkg> {
        Self::slot_ref(&self.swapchain_default_rt_pkg, "swapchain_default_rt_pkg")
    }
    fn views(&self) -> ModulResult<&SwapchainImageViewsDefaultRtPkg> {
        Self::slot_ref(
            &self.swapchain_image_views_default_rt_pkg,
            "swapchain_image_views_default_rt_pkg",
        )
    }
    fn sample(&self) -> ModulResult<&SampleCountDefaultRtPkg> {
        Self::slot_ref(
            &self.sample_count_default_rt_pkg,
            "sample_count_default_rt_pkg",
        )
    }
    fn depth(&self) -> ModulResult<&DepthImagesDefaultRtPkg> {
        Self::slot_ref(
            &self.depth_images_default_rt_pkg,
            "depth_images_default_rt_pkg",
        )
    }
    fn msaa(&self) -> ModulResult<&MsaaColorDefaultRtPkg> {
        Self::slot_ref(&self.msaa_color_default_rt_pkg, "msaa_color_default_rt_pkg")
    }
    fn framebuffer(&self) -> ModulResult<&FramebufferDefaultRtPkg> {
        Self::slot_ref(
            &self.framebuffer_default_rt_pkg,
            "framebuffer_default_rt_pkg",
        )
    }
    fn cargo(&self) -> ModulResult<&PresentationDefaultRtCrg> {
        Self::slot_ref(&self.cargo_rt, "cargo_rt")
    }
}
