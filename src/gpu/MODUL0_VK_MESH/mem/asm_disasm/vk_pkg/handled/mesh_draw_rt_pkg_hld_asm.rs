//! Mesh draw runtime bag from setup peels.

use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_draw_default_rt_pkg::MeshDrawDefaultRtPkg;
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::setup::mesh_draw_default_stp_pkg::MeshDrawDefaultStpPkg;
use crate::gpu::MODUL0_VK_MESH::proc::processor::mesh_draw_pack::pack_mesh_draw_default_rt;

/// `MeshDrawDefaultRtPkgHandled` — trait (mesh draw default rt pkg handled).
///
/// Handled-assemble catalog trait: construction from imported knobs / peels (FIX-129 Handled rank).
/// Belongs to: mesh upload / solid draw MCG.
/// Module path context: `gpu/MODUL0_VK_MESH/mem/asm_disasm/vk_pkg/handled`.
pub trait MeshDrawDefaultRtPkgHandled {
    fn handled_assemble(mesh_draw_default_stp_pkg: &MeshDrawDefaultStpPkg) -> Self;
}

impl MeshDrawDefaultRtPkgHandled for MeshDrawDefaultRtPkg {
    fn handled_assemble(mesh_draw_default_stp_pkg: &MeshDrawDefaultStpPkg) -> Self {
        let mode_stp = mesh_draw_default_stp_pkg.mode_stp;
        pack_mesh_draw_default_rt(mesh_draw_default_stp_pkg, mode_stp)
    }
}
