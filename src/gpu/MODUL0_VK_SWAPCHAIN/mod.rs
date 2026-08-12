//! # `MODUL0_VK_SWAPCHAIN` — bootstrap + presentation
//!
//! First MCG in the direct product path.
//!
//! ## Responsibilities
//!
//! - Vulkan **instance / physical device / logical device**
//! - **Surface** from a window handle (winit)
//! - **Swapchain** images, image views, depth/MSAA attachments as needed
//! - Present mode selection ([`SwapchainPrt`](crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::prt::SwapchainPrt):
//!   e.g. `SRGB_FIFO` for vsync, `SRGB_MAILBOX` for uncapped)
//!
//! ## Layers
//!
//! - `mem` — bags + `asm_disasm` Auto|Handled cargo
//! - `conv` — port import/export (PTP for apps)
//! - `proc` — device-side processors where needed
//!
//! ## App usage
//!
//! Prefer `conv::port` (`SwapchainBfr`, presentation ports) as in `range/cubes`.
pub mod conv;
/// Submodule `mem`.
/// Part of `gpu/MODUL0_VK_SWAPCHAIN` under the mem/conv/proc MCG canon.
pub mod mem;
/// Submodule `proc`.
/// Part of `gpu/MODUL0_VK_SWAPCHAIN` under the mem/conv/proc MCG canon.
pub mod proc;
