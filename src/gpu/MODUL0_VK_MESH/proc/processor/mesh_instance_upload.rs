//! Re-upload instance buffer (host-visible) · Viewsor draw set (xyz + lod in w).

use ash::Device;

use crate::gpu::MODUL0_VK_MESH::mem::asm_disasm::vk::handled::buffer_hld_asm::handled_upload_host_visible;
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::MeshGpuDefaultRtPkg;
use crate::ModulResult;

/// Pack xyz + `lod_w` → xyzw stride 16 and upload · clamps to capacity.
pub fn upload_instance_xyz_lod(
    device_extrl: &Device,
    mesh: &mut MeshGpuDefaultRtPkg,
    xyz: &[[f32; 3]],
    lod_w: &[f32],
) -> ModulResult<()> {
    if !mesh.ready_rt || mesh.instance_memory_extrl == ash::vk::DeviceMemory::null() {
        return Ok(());
    }
    let cap = mesh.instance_capacity_rt as usize;
    if cap == 0 {
        mesh.instance_count_rt = 0;
        return Ok(());
    }
    // Pack up to capacity; missing lod_w entries default to 0.0 (full-detail solid).
    let n = xyz.len().min(cap);
    let mut bytes = Vec::with_capacity(n * 16);
    for (i, p) in xyz.iter().take(n).enumerate() {
        let w = lod_w.get(i).copied().unwrap_or(0.0);
        for f in [p[0], p[1], p[2], w] {
            bytes.extend_from_slice(&f.to_ne_bytes());
        }
    }
    handled_upload_host_visible(device_extrl, mesh.instance_memory_extrl, &bytes)?;
    mesh.instance_count_rt = n as u32;
    let local_tris = mesh.index_count_rt / 3;
    mesh.triangle_count_rt = local_tris.saturating_mul(mesh.instance_count_rt);
    Ok(())
}

/// Backward-compatible upload (lod = 0 full cube).
pub fn upload_instance_xyz(
    device_extrl: &Device,
    mesh: &mut MeshGpuDefaultRtPkg,
    xyz: &[[f32; 3]],
) -> ModulResult<()> {
    let lod = vec![0.0f32; xyz.len()];
    upload_instance_xyz_lod(device_extrl, mesh, xyz, &lod)
}
