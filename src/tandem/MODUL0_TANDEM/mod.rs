//! # `MODUL0_TANDEM` — product session warehouse (compose GPU MCGs)
//!
//! - **mem** — `TandemBfr`, session Prt/Stp, Handled free/recreate  
//! - App etalon (`range/cubes`) owns winit shell, pulse, logging

/// Submodule `mem`.
pub mod mem;

pub use mem::asm_disasm::vk_pkg::handled::tandem_hld_asm::{
    free_tandem, recreate_presentation_extent, TandemDefaultHandled,
};
pub use mem::base::transport::prt::{
    SampleCountPreferPrt, TandemSessionPrt, ValidationPreferPrt,
};
pub use mem::base::transport::setup::TandemSessionStpPkg;
pub use mem::tandem_bfr::TandemBfr;
