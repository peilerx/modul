//! Transport runtime bags — **res_intsct** co-location (M.BASE_RES_INTSCT · FIX-120).
//!
//! - `boot_res_intsct_rt_pkgs` — device line + `SwapchainRtCrg` (no foreign bag imports)
//! - `present_res_intsct_rt_pkgs` — full presentation MCU (swapchain·views·depth·msaa·fb·crg)
//!
//! Rule: *Pkg that **use each other** (fields / same cargo) → **same file** · ¬ cross-res_intsct imports.

pub mod boot_res_intsct_rt_pkgs;
/// Submodule `present_res_intsct_rt_pkgs`.
/// Part of `gpu/MODUL0_VK_SWAPCHAIN/mem/base/transport/runtime` under the mem/conv/proc MCG canon.
pub mod present_res_intsct_rt_pkgs;

pub use boot_res_intsct_rt_pkgs::{
    DeviceDefaultRtPkg, PhysicalDeviceDefaultRtPkg, SurfaceDefaultRtPkg,
    SwapchainCommandPoolDefaultRtPkg, SwapchainLoaderDefaultRtPkg, SwapchainRtCrg,
};
pub use present_res_intsct_rt_pkgs::{
    DepthImagesDefaultRtPkg, FramebufferDefaultRtPkg, MsaaColorDefaultRtPkg,
    PresentationDefaultRtCrg, SampleCountDefaultRtPkg, SwapchainDefaultRtPkg,
    SwapchainImageViewsDefaultRtPkg,
};
