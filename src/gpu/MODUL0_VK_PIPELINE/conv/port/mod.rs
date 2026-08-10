//! Port — subject ports + legacy free import/export.

pub mod export;
/// Submodule `import`.
/// Part of `gpu/MODUL0_VK_PIPELINE/conv/port` under the mem/conv/proc MCG canon.
pub mod import;
/// Submodule `renderer_port`.
/// Part of `gpu/MODUL0_VK_PIPELINE/conv/port` under the mem/conv/proc MCG canon.
pub mod renderer_port;

pub use renderer_port::{RendererTransportable, IMPORT_FOR_ASM_FACTORY_LINE_N};
pub use crate::gpu::MODUL0_VK_PIPELINE::mem::base::embedded::buffer::RendererBfr;
pub use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_bfr::auto::renderer_bfr_at_asm::RendererBfrAuto;
