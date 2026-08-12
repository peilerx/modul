//! MSAA sample preference (MCG pick resolves to concrete flags).

/// Preference only — actual `SampleCountFlags` from device caps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SampleCountPreferPrt {
    /// Prefer 4× when color∩depth allow; else 1×.
    #[default]
    Prefer4Else1,
    /// Always 1× (low-end / debug).
    Force1,
    /// Prefer 8× then 4× then 1×.
    Prefer8Else4Else1,
}
