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
pub mod mesh_soa_bind;
pub mod mesh_push_rt_at_asm;
pub mod mesh_upload_prep;

pub use mesh_gpu_rt_at_asm::{mesh_gpu_center_rt, mesh_gpu_radius_rt};
pub use mesh_push_rt_at_asm::{
    mesh_push_apply_view3d_look, mesh_push_from_orbit, mesh_push_identity_steel,
};
