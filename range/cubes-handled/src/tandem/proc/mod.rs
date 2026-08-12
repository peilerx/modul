//! Processing · winit shell · session assemble · pulse · ship log.

pub mod run_tandem_pulse;
pub mod session_log;
pub mod shell;
pub mod sys_stats;
pub mod tandem_session_asm;

pub use modul::tandem::MODUL0_TANDEM::{free_tandem, recreate_presentation_extent};
pub use run_tandem_pulse::run_tandem_pulse;
pub use shell::run_shell_handled;
pub use tandem_session_asm::assemble_tandem_session;
