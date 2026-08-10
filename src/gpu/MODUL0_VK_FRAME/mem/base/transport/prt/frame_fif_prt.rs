//! IntentProtocol — frame session FIF principle (W · FIX-115/117/118).
//! **IntentOwner:** `MODUL0_VK_FRAME` · enum only · PortMatch ∈ `conv/port/intent`.
//! Module picture · closed gestalt → full `FrameFifDefaultStpPkg` per arm.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FrameFifPrt {
    /// Triple buffering (3 frames in flight).
    #[default]
    TripleBuffered,
    /// Double buffering (2 frames in flight).
    DoubleBuffered,
    /// Single frame in flight.
    SingleBuffered,
}
