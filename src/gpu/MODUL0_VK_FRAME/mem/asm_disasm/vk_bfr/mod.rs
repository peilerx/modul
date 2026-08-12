//! Rank **vk_bfr** — Auto empty seed · Handled *Stp seed for embedded `FrameBfr`.
//! Type: `mem/base/embedded/buffer/`.

pub mod auto;
pub mod handled;

pub use crate::gpu::MODUL0_VK_FRAME::mem::base::embedded::buffer::FrameBfr;
pub use handled::FrameBfrHandled;
