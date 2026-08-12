//! Catalog — Handled *Bfr seed from surface *Stp (FIX-129 · FIX-131).

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::buffer::SwapchainBfr;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::setup::boot_res_intsct_stp_pkgs::SurfaceWindowStpPkg;

/// Handled warehouse seed · surface window knobs already on Bfr.
pub trait SwapchainBfrHandled: Sized {
    fn handled_assemble(surface_window_stp_pkg: SurfaceWindowStpPkg) -> Self;
}

impl SwapchainBfrHandled for SwapchainBfr {
    fn handled_assemble(surface_window_stp_pkg: SurfaceWindowStpPkg) -> Self {
        Self {
            surface_window_stp_pkg: Some(surface_window_stp_pkg),
            entry_default_rt: None,
            instance_default_rt: None,
            surface_default_rt_pkg: None,
            physical_device_default_rt_pkg: None,
            device_default_rt_pkg: None,
            swapchain_command_pool_default_rt_pkg: None,
            swapchain_loader_default_rt_pkg: None,
            cargo_rt: None,
            swapchain_default_rt_pkg: None,
        }
    }
}
