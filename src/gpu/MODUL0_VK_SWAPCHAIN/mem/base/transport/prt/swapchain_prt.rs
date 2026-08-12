//! # `SwapchainPrt` — present / surface format intent
//!
//! Intent enum only · DirectVk peel ∈ `asm_disasm` (W.DirectVk).

/// Swapchain present picture (module intention).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SwapchainPrt {
    /// `B8G8R8A8_SRGB` + **MAILBOX** (uncapped; handled may fall back to FIFO).
    #[default]
    SRGB_MAILBOX,
    /// `B8G8R8A8_SRGB` + **FIFO** (vertical sync).
    SRGB_FIFO,
    /// `B8G8R8A8_UNORM` + MAILBOX.
    UNORM_MAILBOX,
}
