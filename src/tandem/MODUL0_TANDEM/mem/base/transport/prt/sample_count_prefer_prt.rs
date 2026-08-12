//! MSAA sample preference (MCG pick resolves to concrete flags).
//! Intent enum only · peel ∈ `asm_disasm`.

/// Preference only — actual `SampleCountFlags` from device caps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SampleCountPreferPrt {
    /// Prefer 4× when color∩depth allow; else 1×.
    #[default]
    PREFER_4_ELSE_1,
    /// Always 1× (low-end / debug).
    FORCE_1,
    /// Prefer 8× then 4× then 1×.
    PREFER_8_ELSE_4_ELSE_1,
}
