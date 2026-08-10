//! T.Hub — cubes direct (assemble · pulse · free).

mod mem;
mod proc;

pub use mem::tandem_bfr::TandemBfr;
pub use proc::assemble_tandem_session::assemble_tandem_session;
pub use proc::free_tandem::free_tandem;
pub use proc::run_tandem_pulse::run_tandem_pulse;
