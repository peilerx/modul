//! SWAPCHAIN transport setup — boot/present res_intsct.
pub mod boot_res_intsct_stp_pkgs;
/// Submodule `present_res_intsct_stp_pkgs`.
///
/// Part of `gpu/MODUL0_VK_SWAPCHAIN/mem/base/transport/setup` under the mem/conv/proc MCG canon.
pub mod present_res_intsct_stp_pkgs;

pub use boot_res_intsct_stp_pkgs::{SurfaceWindowStpPkg, SwapchainAssemblyDefaultStpPkg};
pub use present_res_intsct_stp_pkgs::{PresentationDefaultStpPkg, SwapchainDefaultStpPkg};
