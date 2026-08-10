//! Port module `gpu/MODUL0_VK_FRAME/mem/base/transport`.
//!
//! PTP import/export free functions (FIX-128/130).

pub mod prt;
/// Submodule `runtime`.
/// Part of `gpu/MODUL0_VK_FRAME/mem/base/transport` under the mem/conv/proc MCG canon.
pub mod runtime;
/// Submodule `setup`.
/// Part of `gpu/MODUL0_VK_FRAME/mem/base/transport` under the mem/conv/proc MCG canon.
pub mod setup;

pub use prt::FrameFifPrt;
pub use runtime::{
    FrameRenderDefaultRtPkg, FrameDefaultRtCrg, FrameSlotDefaultRtPkg, FrameSyncDefaultRtPkg,
};
pub use setup::frame_fif_default_stp_pkg::FrameFifDefaultStpPkg;
