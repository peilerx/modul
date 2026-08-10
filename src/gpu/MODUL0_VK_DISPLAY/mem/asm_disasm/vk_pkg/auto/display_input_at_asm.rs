use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayInputDefaultRtPkg;

/// Catalog — Strategy=Default (FIX-089).
pub trait DisplayInputDefaultAuto {
    fn auto_assemble() -> DisplayInputDefaultRtPkg;
}

impl DisplayInputDefaultAuto for DisplayInputDefaultRtPkg {
    fn auto_assemble() -> DisplayInputDefaultRtPkg {
        DisplayInputDefaultRtPkg {
            cursor_x_rt: 0.0,
            desc: "display_input",
        }
    }
}