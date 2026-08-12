//! `IntentProtocol` — display present principle (W · FIX-115/117/118).
//!
//! **`IntentOwner`:** `MODUL0_VK_DISPLAY` · enum only · data · PortMatch ∈ `asm_disasm`.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DisplayPresentPrt {
    /// Default present path (geometry bind).
    #[default]
    DEFAULT_PRESENT,
    /// Clear-only record path.
    CLEAR_COLOR_ONLY,
    /// Full triangle record path.
    RECORD_TRIANGLE,
}
