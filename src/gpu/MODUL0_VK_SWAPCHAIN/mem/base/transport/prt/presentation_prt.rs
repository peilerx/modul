//! IntentProtocol — presentation sample/depth principle (W · FIX-115/117/118).
//! **IntentOwner:** `MODUL0_VK_SWAPCHAIN` · enum only · PortMatch ∈ `conv/port/intent`.
//! Module picture · PortMatch writes sample count + depth format **direct**.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PresentationPrt {
    /// Color + depth, sample count 1 (etalon).
    #[default]
    SimpleDepth,
    /// Color + depth + 4× MSAA.
    DepthMsaa4,
    /// Color + depth + 8× MSAA.
    DepthMsaa8,
}
