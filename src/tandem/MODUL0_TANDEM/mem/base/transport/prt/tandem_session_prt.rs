//! High-level session pictures (aggregate child MCG intents).
//!
//! Variant names are **CAPS_SNAKE_CASE**. Expand ∈ `asm_disasm` (base ¬ impl).

/// Closed gestalt arms for the cubes/product tandem session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TandemSessionPrt {
    /// Telegram ship: MAILBOX · prefer AA4 · no validation · double FIF · 1M cubes.
    #[default]
    SHIP_MAILBOX_AA4_NO_VALIDATION,
    /// Same as ship but prefer Khronos validation (debug ship).
    SHIP_MAILBOX_AA4_PREFER_VALIDATION,
    /// Dev: FIFO vsync · force 1× · validation · double FIF.
    DEV_FIFO_AA1_PREFER_VALIDATION,
    /// Benchmark: MAILBOX · force 1× · no validation.
    BENCHMARK_MAILBOX_AA1_NO_VALIDATION,
    /// Low-end: FIFO · force 1× · no validation.
    LOW_END_FIFO_AA1_NO_VALIDATION,
}
