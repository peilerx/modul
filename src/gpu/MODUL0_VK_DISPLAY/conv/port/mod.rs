//! Port — subject `*_port.rs` only (swapchain calque · ¬ import/export folders).

pub mod display_port;

pub use display_port::{DisplayTransportable, IMPORT_FOR_ASM_FACTORY_LINE_N};
pub use crate::gpu::MODUL0_VK_DISPLAY::mem::base::embedded::buffer::DisplayBfr;
pub use crate::gpu::MODUL0_VK_DISPLAY::mem::asm_disasm::vk_bfr::auto::display_bfr_at_asm::DisplayBfrAuto;
pub use crate::gpu::MODUL0_VK_DISPLAY::mem::asm_disasm::vk_bfr::handled::DisplayBfrHandled;
