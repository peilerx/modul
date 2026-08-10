//! IntentProtocol — display present principle (W · FIX-115/117/118).
//! **IntentOwner:** `MODUL0_VK_DISPLAY` · enum only · PortMatch ∈ `conv/port/intent`.
//! Module picture · closed gestalt → full `DisplayPresentDefaultStpPkg` per arm.
//! Record-time path may also match this enum in proc (DirectVk levers there).

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DisplayPresentPrt {
    /// Default present path (geometry bind).
    #[default]
    DefaultPresent,
    /// Clear-only record path.
    ClearColorOnly,
    /// Full triangle record path.
    RecordTriangle,
}
