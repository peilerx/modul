//! Port — subject ports + legacy free import/export.

pub mod export;
/// Submodule `import`.
/// Part of `gpu/MODUL0_VK_MESH/conv/port` under the mem/conv/proc MCG canon.
pub mod import;
/// Submodule `mesh_gpu_port`.
/// Part of `gpu/MODUL0_VK_MESH/conv/port` under the mem/conv/proc MCG canon.
pub mod mesh_gpu_port;

pub use mesh_gpu_port::{MeshGpuTransportable, IMPORT_FOR_ASM_FACTORY_LINE_N};
pub use crate::gpu::MODUL0_VK_MESH::mem::base::embedded::buffer::MeshGpuBfr;
pub use crate::gpu::MODUL0_VK_MESH::mem::asm_disasm::vk_bfr::auto::mesh_gpu_bfr_at_asm::MeshGpuBfrAuto;
