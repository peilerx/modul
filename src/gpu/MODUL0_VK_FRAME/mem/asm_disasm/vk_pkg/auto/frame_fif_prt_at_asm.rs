//! PortMatch Auto · `FrameFifPrt` → `FrameFifDefaultStpPkg` (FIX-129 · base ¬ impl).

use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::prt::frame_fif_prt::FrameFifPrt;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::setup::frame_fif_default_stp_pkg::FrameFifDefaultStpPkg;

/// Catalog — closed-gestalt FIF setup bag from intent picture.
pub trait FrameFifDefaultStpAuto {
    fn auto_assemble(frame_fif_prt: FrameFifPrt) -> Self;
}

impl FrameFifDefaultStpAuto for FrameFifDefaultStpPkg {
    fn auto_assemble(frame_fif_prt: FrameFifPrt) -> Self {
        match frame_fif_prt {
            FrameFifPrt::TRIPLE_BUFFERED => Self {
                frames_in_flight_stp: 3,
                fences_signaled_stp: true,
                primary_command_buffers_stp: true,
                desc: "frame_fif_triple_buffered",
            },
            FrameFifPrt::DOUBLE_BUFFERED => Self {
                frames_in_flight_stp: 2,
                fences_signaled_stp: true,
                primary_command_buffers_stp: true,
                desc: "frame_fif_double_buffered",
            },
            FrameFifPrt::SINGLE_BUFFERED => Self {
                frames_in_flight_stp: 1,
                fences_signaled_stp: true,
                primary_command_buffers_stp: true,
                desc: "frame_fif_single_buffered",
            },
        }
    }
}

/// Resolved frames-in-flight count (1 · 2 · 3).
#[must_use]
#[inline]
pub const fn frame_fif_frames_in_flight(frame_fif_prt: FrameFifPrt) -> u32 {
    match frame_fif_prt {
        FrameFifPrt::TRIPLE_BUFFERED => 3,
        FrameFifPrt::DOUBLE_BUFFERED => 2,
        FrameFifPrt::SINGLE_BUFFERED => 1,
    }
}
