//! Intent in-port — PortMatch `DisplayPresentPrt` → write setup bag (FIX-128 · v5.1).
//!
//! **Closed gestalt:** each arm writes **every** *Stp lever.
//! `frames_in_flight_stp` = external alignment with FRAME (param).

use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::setup::display_present_default_stp_pkg::DisplayPresentDefaultStpPkg;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::DisplayPresentPrt;

/// PortMatch display picture → write setup bag (`import_*_for_asm` · never returns bag).
pub fn import_display_present_for_asm(
    display_present_prt: DisplayPresentPrt,
    frames_in_flight_stp: u32,
    display_present_default_stp_pkg: &mut DisplayPresentDefaultStpPkg,
) {
    *display_present_default_stp_pkg = match display_present_prt {
        DisplayPresentPrt::DefaultPresent => DisplayPresentDefaultStpPkg {
            frames_in_flight_stp,
            clear_only_stp: false,
            bind_geometry_stp: true,
            desc: "display_present_default",
        },
        DisplayPresentPrt::ClearColorOnly => DisplayPresentDefaultStpPkg {
            frames_in_flight_stp,
            clear_only_stp: true,
            bind_geometry_stp: false,
            desc: "display_present_clear_only",
        },
        DisplayPresentPrt::RecordTriangle => DisplayPresentDefaultStpPkg {
            frames_in_flight_stp,
            clear_only_stp: false,
            bind_geometry_stp: true,
            desc: "display_present_record_triangle",
        },
    };
}
