//! IntentProtocol — swapchain assembly picture (W · FIX-115/117/118).
//! **IntentOwner:** `MODUL0_VK_SWAPCHAIN` · enum only · PortMatch ∈ `conv/port/import`.
//! Module picture → port `import_for_asm8` factory-line (7 atom + 1 pack *Crg) · Intent lever.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SwapchainAssemblyPrt {
    /// Graphics+present queues, validation layers on (etalon).
    #[default]
    GRAPHICS_PRESENT_VALIDATION,
    /// Same queues, no validation layers.
    GRAPHICS_PRESENT_NO_VALIDATION,
}
