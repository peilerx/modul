//! Prepare steel mesh host bytes + mode (P · no vk create).

use crate::cpu::MODUL0_MESH::mem::base::transport::runtime::mesh_soa_rt_bfr::MeshSoaRtBfr;
use crate::cpu::MODUL0_MESH::proc::processor::pack_index_bytes::{
    mesh_solid_buffer_counts, pack_u32_indices_to_bytes,
};
use crate::cpu::MODUL0_MESH::proc::processor::pack_steel_interleaved::pack_steel_flat_from_mesh;
use crate::gpu::MODUL0_VK_MESH::mem::asm_disasm::vk_pkg::auto::mesh_draw_prt_at_asm::mesh_draw_mode_stp;
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::prt::mesh_draw_prt::MeshDrawPrt;

/// Host peels ready for catalog buffer create.
pub struct MeshUploadPrep {
    /// 0 = skip create (disabled / too few indices).
    pub empty_stp: u32,
    /// Runtime phase field `mode_rt`.
    pub mode_rt: u32,
    /// External / raw Vulkan handle or host pointer field `vert_bytes_extrl` (`vert_bytes` peel).
    pub vert_bytes_extrl: Vec<u8>,
    /// External / raw Vulkan handle or host pointer field `idx_bytes_extrl` (`idx_bytes` peel).
    pub idx_bytes_extrl: Vec<u8>,
    /// Runtime phase field `vertex_count_rt`.
    pub vertex_count_rt: u32,
    /// Runtime phase field `index_count_rt`.
    pub index_count_rt: u32,
    /// Runtime phase field `triangle_count_rt`.
    pub triangle_count_rt: u32,
    /// Runtime phase field `bounds_min_rt`.
    pub bounds_min_rt: [f32; 3],
    /// Runtime phase field `bounds_max_rt`.
    pub bounds_max_rt: [f32; 3],
}

/// `prepare_mesh_upload` — function (prepare mesh upload).
/// Public API entry for this module.
/// Belongs to: mesh upload / solid draw MCG.
#[must_use]
pub fn prepare_mesh_upload(
    mesh_soa_rt_bfr: &MeshSoaRtBfr,
    mesh_draw_prt: MeshDrawPrt,
) -> MeshUploadPrep {
    if matches!(mesh_draw_prt, MeshDrawPrt::DISABLED) || mesh_soa_rt_bfr.indices.len() < 3 {
        return MeshUploadPrep {
            empty_stp: 1,
            mode_rt: 0,
            vert_bytes_extrl: Vec::new(),
            idx_bytes_extrl: Vec::new(),
            vertex_count_rt: 0,
            index_count_rt: 0,
            triangle_count_rt: 0,
            bounds_min_rt: [0.0; 3],
            bounds_max_rt: [0.0; 3],
        };
    }
    let (vert_bytes_extrl, indices_extrl, bounds_min_rt, bounds_max_rt) =
        pack_steel_flat_from_mesh(mesh_soa_rt_bfr);
    let mode_rt = mesh_draw_mode_stp(mesh_draw_prt);
    let idx_bytes_extrl = pack_u32_indices_to_bytes(&indices_extrl);
    let (vertex_count_rt, index_count_rt, triangle_count_rt) =
        mesh_solid_buffer_counts(vert_bytes_extrl.len(), indices_extrl.len());
    MeshUploadPrep {
        empty_stp: 0,
        mode_rt,
        vert_bytes_extrl,
        idx_bytes_extrl,
        vertex_count_rt,
        index_count_rt,
        triangle_count_rt,
        bounds_min_rt,
        bounds_max_rt,
    }
}
