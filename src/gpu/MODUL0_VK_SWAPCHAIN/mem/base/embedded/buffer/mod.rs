//! Embedded buffer warehouses (*Bfr).
pub mod presentation_bfr;
/// Submodule `swapchain_bfr`.
///
/// Part of `gpu/MODUL0_VK_SWAPCHAIN/mem/base/embedded/buffer` under the mem/conv/proc MCG canon.
pub mod swapchain_bfr;

pub use presentation_bfr::PresentationBfr;
pub use swapchain_bfr::SwapchainBfr;
