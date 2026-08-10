//! `PresentationBfr` — present-lane atom slots + cargo (type only).

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::DepthImagesDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::FramebufferDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::MsaaColorDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::PresentationDefaultRtCrg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SampleCountDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SwapchainDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SwapchainImageViewsDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::setup::present_res_intsct_stp_pkgs::PresentationDefaultStpPkg;

/// `PresentationBfr` — buffer / warehouse bag (presentation bfr).
/// Memory-layer bag: owned fields, no product control flow.
/// Belongs to: swapchain / device bootstrap MCG.
/// Module path context: `gpu/MODUL0_VK_SWAPCHAIN/mem/base/embedded/buffer`.
pub struct PresentationBfr {
    /// Nested package bag field `presentation_default_stp_pkg`.
    pub presentation_default_stp_pkg: Option<PresentationDefaultStpPkg>,
    /// Nested package bag field `swapchain_default_rt_pkg`.
    pub swapchain_default_rt_pkg: Option<SwapchainDefaultRtPkg>,
    /// Nested package bag field `swapchain_image_views_default_rt_pkg`.
    pub swapchain_image_views_default_rt_pkg: Option<SwapchainImageViewsDefaultRtPkg>,
    /// Nested package bag field `sample_count_default_rt_pkg`.
    pub sample_count_default_rt_pkg: Option<SampleCountDefaultRtPkg>,
    /// Nested package bag field `depth_images_default_rt_pkg`.
    pub depth_images_default_rt_pkg: Option<DepthImagesDefaultRtPkg>,
    /// Nested package bag field `msaa_color_default_rt_pkg`.
    pub msaa_color_default_rt_pkg: Option<MsaaColorDefaultRtPkg>,
    /// Nested package bag field `framebuffer_default_rt_pkg`.
    pub framebuffer_default_rt_pkg: Option<FramebufferDefaultRtPkg>,
    /// Runtime phase field `cargo_rt`.
    pub cargo_rt: Option<PresentationDefaultRtCrg>,
}
