//! # GPU session stub
//!
//! Product **Session** is snapshot-oriented (FIX-127): the app shell owns the
//! hub (`TandemBfr` in `range/cubes`), not a long-lived “GPU god object”.
//!
//! This module remains so old call sites compile. Prefer:
//!
//! - `assemble_tandem_session` / `run_tandem_pulse` / `free_tandem` in the app
//! - Per-MCG `conv::port` import/export for PTP wiring

use crate::ModulResult;

/// Placeholder: returns an error directing callers to app T.Hub assemble.
///
/// # Errors
///
/// Always returns `Err` with a guidance string.
pub fn assemble_gpu_session() -> ModulResult<()> {
    Err(
        "gpu/session: product Session = snapshots only (FIX-127) · \
         app-shell uses MODUL0_VK_* Auto|Handled cargo + run_tandem_pulse (T.Hub · FIX-080/129)"
            .into(),
    )
}
