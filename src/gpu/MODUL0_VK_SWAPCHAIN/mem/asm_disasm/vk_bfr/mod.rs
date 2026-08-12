//! Rank **vk_bfr** — Auto empty seed · Handled *Stp seed for embedded *Bfr.
//! Type: `mem/base/embedded/buffer/`.

pub mod auto;
pub mod handled;

pub use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::buffer::SwapchainBfr;
pub use handled::{PresentationBfrHandled, SwapchainBfrHandled};
