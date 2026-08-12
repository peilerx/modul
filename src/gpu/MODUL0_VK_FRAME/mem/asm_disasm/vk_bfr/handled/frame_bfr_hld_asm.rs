//! Catalog — Handled *Bfr seed from FIF *Stp knobs (FIX-129 · FIX-131).

use crate::gpu::MODUL0_VK_FRAME::mem::base::embedded::buffer::FrameBfr;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::setup::frame_fif_default_stp_pkg::FrameFifDefaultStpPkg;

/// Handled warehouse seed · FIF knobs already on Bfr.
pub trait FrameBfrHandled: Sized {
    fn handled_assemble(frame_fif_default_stp_pkg: FrameFifDefaultStpPkg) -> Self;
}

impl FrameBfrHandled for FrameBfr {
    fn handled_assemble(frame_fif_default_stp_pkg: FrameFifDefaultStpPkg) -> Self {
        Self {
            frame_fif_default_stp_pkg: Some(frame_fif_default_stp_pkg),
            frame_sync_default_rt_pkg: None,
            frame_render_default_rt_pkg: None,
            cargo_rt: None,
        }
    }
}
