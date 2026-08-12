//! Port — subject `*_port.rs` only (swapchain calque · ¬ import/export folders).

pub mod renderer_port;

pub use renderer_port::{RendererTransportable, IMPORT_FOR_ASM_FACTORY_LINE_N};
pub use crate::gpu::MODUL0_VK_PIPELINE::mem::base::embedded::buffer::RendererBfr;
pub use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_bfr::auto::renderer_bfr_at_asm::RendererBfrAuto;
pub use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_bfr::handled::RendererBfrHandled;
