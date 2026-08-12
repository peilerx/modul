//! `IntentProtocol` — frame session FIF principle (W · FIX-115/117/118).
//!
//! **`IntentOwner`:** `MODUL0_VK_FRAME` · enum only · data · PortMatch ∈ `asm_disasm`.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FrameFifPrt {
    /// Triple buffering (3 frames in flight).
    #[default]
    TRIPLE_BUFFERED,
    /// Double buffering (2 frames in flight).
    DOUBLE_BUFFERED,
    /// Single frame in flight.
    SINGLE_BUFFERED,
}
