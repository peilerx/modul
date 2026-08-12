//! Port — subject `*_port.rs` only (swapchain calque · ¬ import/export folders).

pub mod frame_port;

pub use frame_port::{FrameTransportable, IMPORT_FOR_ASM_FACTORY_LINE_N};
pub use crate::gpu::MODUL0_VK_FRAME::mem::base::embedded::buffer::FrameBfr;
pub use crate::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_bfr::auto::frame_bfr_at_asm::FrameBfrAuto;
pub use crate::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_bfr::handled::FrameBfrHandled;
