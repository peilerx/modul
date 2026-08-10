//! Port module `gpu/MODUL0_VK_SWAPCHAIN/mem/base/transport/prt`.
//!
//! PTP import/export free functions (FIX-128/130).

pub mod presentation_prt;
/// Submodule `swapchain_assembly_prt`.
/// Part of `gpu/MODUL0_VK_SWAPCHAIN/mem/base/transport/prt` under the mem/conv/proc MCG canon.
pub mod swapchain_assembly_prt;
/// Submodule `swapchain_prt`.
/// Part of `gpu/MODUL0_VK_SWAPCHAIN/mem/base/transport/prt` under the mem/conv/proc MCG canon.
pub mod swapchain_prt;
pub use presentation_prt::PresentationPrt;
pub use swapchain_assembly_prt::SwapchainAssemblyPrt;
pub use swapchain_prt::SwapchainPrt;
