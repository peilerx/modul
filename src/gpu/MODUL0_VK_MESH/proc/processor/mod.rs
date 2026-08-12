//! Processor module `gpu/MODUL0_VK_MESH/proc/processor`.
//!
//! DomainMath + upload/destroy. Host SoA lattice ∈ `cpu::MODUL0_MESH::proc`.

pub mod line_gpu_counts;
pub mod line_gpu_destroy;
pub mod line_gpu_rt_at_asm;
pub mod mesh_draw_pack;
pub mod mesh_gpu_destroy;
pub mod mesh_gpu_rt_at_asm;
pub mod mesh_instance_upload;
pub mod mesh_push_rt_at_asm;
pub mod mesh_upload_prep;
