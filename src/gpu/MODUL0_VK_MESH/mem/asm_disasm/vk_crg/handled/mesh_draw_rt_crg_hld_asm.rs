use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_draw_default_rt_crg::MeshDrawDefaultRtCrg;
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_draw_default_rt_pkg::MeshDrawDefaultRtPkg;
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::setup::mesh_draw_default_stp_pkg::MeshDrawDefaultStpPkg;
use crate::gpu::MODUL0_VK_MESH::mem::asm_disasm::vk_pkg::handled::mesh_draw_rt_pkg_hld_asm::MeshDrawDefaultRtPkgHandled;

/// `MeshDrawDefaultRtCrgHandled` — trait (mesh draw default rt crg handled).
///
/// Handled-assemble catalog trait: construction from imported knobs / peels (FIX-129 Handled rank).
/// Belongs to: mesh upload / solid draw MCG.
/// Module path context: `gpu/MODUL0_VK_MESH/mem/asm_disasm/vk_crg/handled`.
pub trait MeshDrawDefaultRtCrgHandled {
    fn handled_assemble(mesh_draw_default_stp_pkg: &MeshDrawDefaultStpPkg) -> Self;
}

impl MeshDrawDefaultRtCrgHandled for MeshDrawDefaultRtCrg {
    fn handled_assemble(mesh_draw_default_stp_pkg: &MeshDrawDefaultStpPkg) -> Self {
        let mesh_draw_default_rt_pkg = MeshDrawDefaultRtPkg::handled_assemble(mesh_draw_default_stp_pkg);
        Self {
            mesh_draw_default_rt_pkg,
            desc: "mesh_draw_default_rt_crg",
        }
    }
}
