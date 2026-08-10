use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_draw_default_rt_pkg::MeshDrawDefaultRtPkg;

/// Mesh draw cargo product.
pub struct MeshDrawDefaultRtCrg {
    /// Nested package bag field `mesh_draw_default_rt_pkg`.
    pub mesh_draw_default_rt_pkg: MeshDrawDefaultRtPkg,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}
