//! Port — subject `*_port.rs` · factory-line order (swapchain calque).
//!
//! Legacy free import/export kept for peers mid-migration.

pub mod export;
/// Submodule `frame_port`.
/// Part of `gpu/MODUL0_VK_FRAME/conv/port` under the mem/conv/proc MCG canon.
pub mod frame_port;
/// Submodule `import`.
/// Part of `gpu/MODUL0_VK_FRAME/conv/port` under the mem/conv/proc MCG canon.
pub mod import;

pub use frame_port::{FrameTransportable, IMPORT_FOR_ASM_FACTORY_LINE_N};
pub use crate::gpu::MODUL0_VK_FRAME::mem::base::embedded::buffer::FrameBfr;
pub use crate::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_bfr::auto::frame_bfr_at_asm::FrameBfrAuto;
