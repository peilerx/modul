//! PortMatch Auto · `DisplayPresentPrt` → `DisplayPresentDefaultStpPkg`.

use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::prt::display_present_prt::DisplayPresentPrt;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::setup::display_present_default_stp_pkg::DisplayPresentDefaultStpPkg;

/// Catalog — closed-gestalt display present setup from intent.
pub trait DisplayPresentDefaultStpAuto {
    fn auto_assemble(
        display_present_prt: DisplayPresentPrt,
        frames_in_flight_stp: u32,
    ) -> Self;
}

impl DisplayPresentDefaultStpAuto for DisplayPresentDefaultStpPkg {
    fn auto_assemble(
        display_present_prt: DisplayPresentPrt,
        frames_in_flight_stp: u32,
    ) -> Self {
        match display_present_prt {
            DisplayPresentPrt::DEFAULT_PRESENT => Self {
                frames_in_flight_stp,
                clear_only_stp: false,
                bind_geometry_stp: true,
                desc: "display_present_default",
            },
            DisplayPresentPrt::CLEAR_COLOR_ONLY => Self {
                frames_in_flight_stp,
                clear_only_stp: true,
                bind_geometry_stp: false,
                desc: "display_present_clear_only",
            },
            DisplayPresentPrt::RECORD_TRIANGLE => Self {
                frames_in_flight_stp,
                clear_only_stp: false,
                bind_geometry_stp: true,
                desc: "display_present_record_triangle",
            },
        }
    }
}
