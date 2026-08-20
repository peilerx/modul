//! MeshGpuDefaultRtPkg DomainMath (center · radius) · P.Processor.
//! Empty bag constructor ∈ `mem/asm_disasm/vk_pkg/auto/mesh_gpu_default_rt_pkg_at_asm.rs`.

use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::MeshGpuDefaultRtPkg;

/// AABB center of the uploaded mesh.
#[inline]
#[must_use]
pub fn mesh_gpu_center_rt(mesh_gpu: &MeshGpuDefaultRtPkg) -> [f32; 3] {
    [
        0.5 * (mesh_gpu.bounds_min_rt[0] + mesh_gpu.bounds_max_rt[0]),
        0.5 * (mesh_gpu.bounds_min_rt[1] + mesh_gpu.bounds_max_rt[1]),
        0.5 * (mesh_gpu.bounds_min_rt[2] + mesh_gpu.bounds_max_rt[2]),
    ]
}

/// Orbit radius from AABB half-diagonal.
#[inline]
#[must_use]
pub fn mesh_gpu_radius_rt(mesh_gpu: &MeshGpuDefaultRtPkg) -> f32 {
    let c = mesh_gpu_center_rt(mesh_gpu);
    let dx = (mesh_gpu.bounds_max_rt[0] - c[0]).abs();
    let dy = (mesh_gpu.bounds_max_rt[1] - c[1]).abs();
    let dz = (mesh_gpu.bounds_max_rt[2] - c[2]).abs();
    dz.mul_add(dz, dy.mul_add(dy, dx * dx)).sqrt().max(0.5)
}
