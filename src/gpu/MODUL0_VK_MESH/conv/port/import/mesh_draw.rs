//! Intent in-port — `PortMatch` `MeshDrawPrt` → write setup bag (FIX-128 · v5.1).

use crate::gpu::MODUL0_VK_MESH::mem::base::transport::prt::mesh_draw_prt::MeshDrawPrt;
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::setup::mesh_draw_default_stp_pkg::MeshDrawDefaultStpPkg;

/// `PortMatch` mesh draw picture → write setup bag · never returns.
pub const fn import_mesh_draw_for_asm(
    mesh_draw_prt: MeshDrawPrt,
    vertex_count_stp: u32,
    index_count_stp: u32,
    mesh_draw_default_stp_pkg: &mut MeshDrawDefaultStpPkg,
) {
    *mesh_draw_default_stp_pkg = match mesh_draw_prt {
        MeshDrawPrt::TriangleList => MeshDrawDefaultStpPkg {
            mode_stp: 1,
            vertex_count_stp,
            index_count_stp,
            base_r_stp: 0.58,
            base_g_stp: 0.62,
            base_b_stp: 0.68,
            desc: "mesh_draw_triangle_list",
        },
        MeshDrawPrt::Solid => MeshDrawDefaultStpPkg {
            mode_stp: 3,
            vertex_count_stp,
            index_count_stp,
            base_r_stp: 0.58,
            base_g_stp: 0.62,
            base_b_stp: 0.68,
            desc: "mesh_draw_steel_solid",
        },
        MeshDrawPrt::Wireframe => MeshDrawDefaultStpPkg {
            mode_stp: 2,
            vertex_count_stp,
            index_count_stp,
            base_r_stp: 0.58,
            base_g_stp: 0.62,
            base_b_stp: 0.68,
            desc: "mesh_draw_wireframe",
        },
        MeshDrawPrt::Disabled => MeshDrawDefaultStpPkg {
            mode_stp: 0,
            vertex_count_stp: 0,
            index_count_stp: 0,
            base_r_stp: 0.58,
            base_g_stp: 0.62,
            base_b_stp: 0.68,
            desc: "mesh_draw_disabled",
        },
    };
}
