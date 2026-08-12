//! PortMatch Auto · `MeshDrawPrt` ↔ mode_stp peel.

use crate::gpu::MODUL0_VK_MESH::mem::base::transport::prt::mesh_draw_prt::MeshDrawPrt;

/// `MeshDrawDefaultStpPkg.mode_stp`: 1=list · 2=wire · 3=solid · 0=disabled.
#[must_use]
#[inline]
pub const fn mesh_draw_mode_stp(mesh_draw_prt: MeshDrawPrt) -> u32 {
    match mesh_draw_prt {
        MeshDrawPrt::TRIANGLE_LIST => 1,
        MeshDrawPrt::WIREFRAME => 2,
        MeshDrawPrt::SOLID => 3,
        MeshDrawPrt::DISABLED => 0,
    }
}

/// Inverse of [`mesh_draw_mode_stp`].
#[must_use]
#[inline]
pub const fn mesh_draw_prt_from_mode_stp(mode_stp: u32) -> MeshDrawPrt {
    match mode_stp {
        1 => MeshDrawPrt::TRIANGLE_LIST,
        2 => MeshDrawPrt::WIREFRAME,
        3 => MeshDrawPrt::SOLID,
        _ => MeshDrawPrt::DISABLED,
    }
}
