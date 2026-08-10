//! Port module `gpu/MODUL0_VK_DISPLAY/mem/base/transport`.
//!
//! PTP import/export free functions (FIX-128/130).

pub mod prt;
/// Submodule `runtime`.
/// Part of `gpu/MODUL0_VK_DISPLAY/mem/base/transport` under the mem/conv/proc MCG canon.
pub mod runtime;
/// Submodule `setup`.
/// Part of `gpu/MODUL0_VK_DISPLAY/mem/base/transport` under the mem/conv/proc MCG canon.
pub mod setup;

pub use prt::DisplayPresentPrt;
pub use runtime::{
    DisplayCommandDefaultRt, DisplayInputDefaultRtPkg, DisplayRenderDefaultRt,
    DisplayDefaultRtCrg, VulkanDisplayDefaultRt,
};
pub use setup::display_present_default_stp_pkg::DisplayPresentDefaultStpPkg;
