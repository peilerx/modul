//! Catalog — Handled *Bfr seed from presentation *Stp knobs (FIX-129 · FIX-131).

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::buffer::PresentationBfr;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::setup::present_res_intsct_stp_pkgs::PresentationDefaultStpPkg;

/// Handled warehouse seed · sample/depth knobs already on Bfr.
pub trait PresentationBfrHandled: Sized {
    fn handled_assemble(presentation_default_stp_pkg: PresentationDefaultStpPkg) -> Self;
}

impl PresentationBfrHandled for PresentationBfr {
    fn handled_assemble(presentation_default_stp_pkg: PresentationDefaultStpPkg) -> Self {
        Self {
            presentation_default_stp_pkg: Some(presentation_default_stp_pkg),
            swapchain_default_rt_pkg: None,
            swapchain_image_views_default_rt_pkg: None,
            sample_count_default_rt_pkg: None,
            depth_images_default_rt_pkg: None,
            msaa_color_default_rt_pkg: None,
            framebuffer_default_rt_pkg: None,
            cargo_rt: None,
        }
    }
}
