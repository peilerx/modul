//! Port — subject ports + legacy free import/export.

pub mod display_port;
/// Submodule `export`.
/// Part of `gpu/MODUL0_VK_DISPLAY/conv/port` under the mem/conv/proc MCG canon.
pub mod export;
/// Submodule `import`.
/// Part of `gpu/MODUL0_VK_DISPLAY/conv/port` under the mem/conv/proc MCG canon.
pub mod import;

pub use display_port::{DisplayTransportable, IMPORT_FOR_ASM_FACTORY_LINE_N};
pub use crate::gpu::MODUL0_VK_DISPLAY::mem::base::embedded::buffer::DisplayBfr;
pub use crate::gpu::MODUL0_VK_DISPLAY::mem::asm_disasm::vk_bfr::auto::display_bfr_at_asm::DisplayBfrAuto;
