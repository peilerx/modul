//! # `MODUL0_VK_MESH` — mesh upload + solid draw
//!
//! Host mesh peels → GPU buffers:
//!
//! - Unit cuboid / instanced lattice (`MeshSoaRtBfr`)
//! - Steel solid path: interleaved pos+nrm VBO, indices, instance buffer
//! - Push constants [`MeshPushRt`](crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::MeshPushRt)
//!   (MVP, light, material, pulse knobs in `look3`)
//!
//! ## Layers
//!
//! - `mem` — mesh bags + assembler
//! - `conv` — `MeshGpuBfr` port / `MeshDrawPrt`
//! - `proc` — pack / upload helpers
//!
//! ## App usage
//!
//! `MeshGpuBfr::import_for_asm1(MeshDrawPrt::Solid, device, instance, phys, &mesh)`.
pub mod conv;
/// Submodule `mem`.
/// Part of `gpu/MODUL0_VK_MESH` under the mem/conv/proc MCG canon.
pub mod mem;
/// Submodule `proc`.
/// Part of `gpu/MODUL0_VK_MESH` under the mem/conv/proc MCG canon.
pub mod proc;
