//! Validation layer preference for session boot.

/// Whether to request Khronos validation at instance create.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ValidationPreferPrt {
    /// No validation layers (ship FPS default).
    #[default]
    NoValidation,
    /// Prefer `VK_LAYER_KHRONOS_validation`; fall back if missing.
    PreferValidation,
}
