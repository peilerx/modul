//! Catalog — Handled *Bfr seed from display present *Stp knobs (FIX-129 · FIX-131).

use crate::gpu::MODUL0_VK_DISPLAY::mem::base::embedded::buffer::DisplayBfr;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::setup::display_present_default_stp_pkg::DisplayPresentDefaultStpPkg;

/// Handled warehouse seed · present knobs already on Bfr.
pub trait DisplayBfrHandled: Sized {
    fn handled_assemble(display_present_default_stp_pkg: DisplayPresentDefaultStpPkg) -> Self;
}

impl DisplayBfrHandled for DisplayBfr {
    fn handled_assemble(display_present_default_stp_pkg: DisplayPresentDefaultStpPkg) -> Self {
        Self {
            display_present_default_stp_pkg: Some(display_present_default_stp_pkg),
            display_input_default_rt_pkg: None,
            command_rt: None,
            display_render_default_rt: None,
            vulkan_display_default_rt: None,
            cargo_rt: None,
        }
    }
}
