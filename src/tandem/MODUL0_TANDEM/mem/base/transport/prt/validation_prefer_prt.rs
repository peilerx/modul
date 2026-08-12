//! Validation layer preference for session boot.
//! Intent enum only · peel ∈ `asm_disasm`.

/// Whether to request Khronos validation at instance create.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ValidationPreferPrt {
    /// No validation layers (ship FPS default).
    #[default]
    NO_VALIDATION,
    /// Prefer `VK_LAYER_KHRONOS_validation`; fall back if missing.
    PREFER_VALIDATION,
}
