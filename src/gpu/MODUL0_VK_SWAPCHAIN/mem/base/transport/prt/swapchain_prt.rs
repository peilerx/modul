//! # `SwapchainPrt` — present / surface format intent
//!
//! PortMatch on this enum writes Vulkan format + present mode **directly**
//! (no parallel “vk mirror” enums). Prefer **`SrgbFifo`** for vsync.

/// Swapchain present picture (module intention).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SwapchainPrt {
    /// `B8G8R8A8_SRGB` + **MAILBOX** (uncapped; handled may fall back to FIFO).
    #[default]
    SrgbMailbox,
    /// `B8G8R8A8_SRGB` + **FIFO** (vertical sync).
    SrgbFifo,
    /// `B8G8R8A8_UNORM` + MAILBOX.
    UnormMailbox,
}
