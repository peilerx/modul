//! vk_crg — pack presentation lane cargo (no child creates).

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::DepthImagesDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::FramebufferDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::MsaaColorDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::PresentationDefaultRtCrg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SampleCountDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SwapchainDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SwapchainImageViewsDefaultRtPkg;

/// Catalog — pack already-materialized packages into `PresentationDefaultRtCrg`.
pub trait PresentationDefaultRtCrgAuto {
    fn auto_assemble(
        swapchain_default_rt_pkg: SwapchainDefaultRtPkg,
        swapchain_image_views_default_rt_pkg: SwapchainImageViewsDefaultRtPkg,
        sample_count_default_rt_pkg: SampleCountDefaultRtPkg,
        depth_images_default_rt_pkg: DepthImagesDefaultRtPkg,
        msaa_color_default_rt_pkg: MsaaColorDefaultRtPkg,
        framebuffer_default_rt_pkg: FramebufferDefaultRtPkg,
    ) -> PresentationDefaultRtCrg;
}

impl PresentationDefaultRtCrgAuto for PresentationDefaultRtCrg {
    fn auto_assemble(
        swapchain_default_rt_pkg: SwapchainDefaultRtPkg,
        swapchain_image_views_default_rt_pkg: SwapchainImageViewsDefaultRtPkg,
        sample_count_default_rt_pkg: SampleCountDefaultRtPkg,
        depth_images_default_rt_pkg: DepthImagesDefaultRtPkg,
        msaa_color_default_rt_pkg: MsaaColorDefaultRtPkg,
        framebuffer_default_rt_pkg: FramebufferDefaultRtPkg,
    ) -> PresentationDefaultRtCrg {
        PresentationDefaultRtCrg {
            swapchain_default_rt_pkg,
            swapchain_image_views_default_rt_pkg,
            sample_count_default_rt_pkg,
            depth_images_default_rt_pkg,
            msaa_color_default_rt_pkg,
            framebuffer_default_rt_pkg,
            desc: "presentation_lane",
        }
    }
}
