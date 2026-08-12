//! Pack MeshDrawDefaultRtPkg fields (P · arithmetic).

use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_draw_default_rt_pkg::MeshDrawDefaultRtPkg;
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::setup::mesh_draw_default_stp_pkg::MeshDrawDefaultStpPkg;

/// `pack_mesh_draw_default_rt` — function (pack mesh draw default rt).
/// Public API entry for this module.
/// Belongs to: mesh upload / solid draw MCG.
#[must_use]
pub const fn pack_mesh_draw_default_rt(
    mesh_draw_default_stp_pkg: &MeshDrawDefaultStpPkg,
    mode_stp: u32,
) -> MeshDrawDefaultRtPkg {
    MeshDrawDefaultRtPkg {
        ready_rt: mode_stp != 0 && mesh_draw_default_stp_pkg.index_count_stp > 0,
        mode_rt: mode_stp,
        vertex_count_rt: mesh_draw_default_stp_pkg.vertex_count_stp,
        index_count_rt: mesh_draw_default_stp_pkg.index_count_stp,
        triangle_count_rt: mesh_draw_default_stp_pkg.index_count_stp / 3,
        base_r_rt: mesh_draw_default_stp_pkg.base_r_stp,
        base_g_rt: mesh_draw_default_stp_pkg.base_g_stp,
        base_b_rt: mesh_draw_default_stp_pkg.base_b_stp,
        desc: "mesh_draw_default_rt_pkg",
    }
}
