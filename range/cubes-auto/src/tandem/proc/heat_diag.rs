//! Host readback of the heat SoA. OPTIMAL image3D is not HOST_VISIBLE.

use crate::tandem::proc::session_log;
use modul::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayDefaultRtCrg;
use modul::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::DeviceDefaultRtPkg;

/// Host histogram. OPTIMAL 3D image is not HOST_VISIBLE — log only.
#[allow(dead_code, reason = "optional host histogram, not on the pulse path")]
pub fn dump_heat(
    device: &DeviceDefaultRtPkg,
    display: &DisplayDefaultRtCrg,
    tag: &str,
) {
    let _ = (device, display);
    session_log::log(&format!(
        "HEAT dump skipped · image3D OPTIMAL not HOST_VISIBLE · {tag}"
    ));
}
