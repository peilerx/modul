//! # `MODUL0_VK_MESH` — GPU mesh upload + solid draw
//!
//! **GPU-only** MCG: device VBO/IBO/instance buffers + push constants.
//! Host SoA lattice / byte pack lives in [`crate::cpu::MODUL0_MESH`].
//!
//! - Import host [`MeshSoaRtBfr`](crate::cpu::MODUL0_MESH::mem::base::transport::runtime::mesh_soa_rt_bfr::MeshSoaRtBfr)
//!   (re-exported under `mem::base::transport` for convenience)
//! - Steel solid path: interleaved pos+nrm VBO, indices, instance buffer
//! - Push constants [`MeshPushRt`](crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::MeshPushRt)
//!
//! ## Layers
//!
//! - `mem` — GPU bags + assembler (host SoA is re-export only)
//! - `conv` — `MeshGpuBfr` port / `MeshDrawPrt`
//! - `proc` — upload / destroy / draw-pack (host pack via CPU MCG)
//!
//! ## App usage
//!
//! `MeshGpuBfr::import_for_asm1(MeshDrawPrt::SOLID, device, instance, phys, &mesh)`.
pub mod conv;
/// Submodule `mem`.
/// Part of `gpu/MODUL0_VK_MESH` under the mem/conv/proc MCG canon.
pub mod mem;
/// Submodule `proc`.
/// Part of `gpu/MODUL0_VK_MESH` under the mem/conv/proc MCG canon.
pub mod proc;
