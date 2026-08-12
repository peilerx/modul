//! Intent in-port — `PortMatch` `FrameFifPrt` → write `FrameFifDefaultStpPkg` (FIX-128 · v5.1).
//!
//! **Closed gestalt:** each arm writes **every** *Stp lever into dest.
//! import never returns a bag · factory-line name `import_frame_fif_for_asm`.

use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::setup::frame_fif_default_stp_pkg::FrameFifDefaultStpPkg;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::FrameFifPrt;

/// `PortMatch` frame FIF picture → write setup bag (near-port yard knobs).
pub const fn import_frame_fif_for_asm(
    frame_fif_prt: FrameFifPrt,
    frame_fif_default_stp_pkg: &mut FrameFifDefaultStpPkg,
) {
    *frame_fif_default_stp_pkg = match frame_fif_prt {
        FrameFifPrt::TripleBuffered => FrameFifDefaultStpPkg {
            frames_in_flight_stp: 3,
            fences_signaled_stp: true,
            primary_command_buffers_stp: true,
            desc: "frame_fif_triple_buffered",
        },
        FrameFifPrt::DoubleBuffered => FrameFifDefaultStpPkg {
            frames_in_flight_stp: 2,
            fences_signaled_stp: true,
            primary_command_buffers_stp: true,
            desc: "frame_fif_double_buffered",
        },
        FrameFifPrt::SingleBuffered => FrameFifDefaultStpPkg {
            frames_in_flight_stp: 1,
            fences_signaled_stp: true,
            primary_command_buffers_stp: true,
            desc: "frame_fif_single_buffered",
        },
    };
}
