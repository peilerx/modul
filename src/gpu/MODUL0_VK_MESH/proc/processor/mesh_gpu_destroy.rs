//! Mesh GPU buffer destroy (P · null checks).

use ash::vk;
use ash::Device;

use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::MeshGpuDefaultRtPkg;

/// `destroy_mesh_gpu_buffers` — function (destroy mesh gpu buffers).
/// Public API entry for this module.
/// Belongs to: mesh upload / solid draw MCG.
pub fn destroy_mesh_gpu_buffers(
    device_extrl: &Device,
    mesh_gpu_default_rt_pkg: &mut MeshGpuDefaultRtPkg,
) {
    if mesh_gpu_default_rt_pkg.vertex_buffer_extrl != vk::Buffer::null() {
        unsafe {
            device_extrl.destroy_buffer(mesh_gpu_default_rt_pkg.vertex_buffer_extrl, None);
            device_extrl.free_memory(mesh_gpu_default_rt_pkg.vertex_memory_extrl, None);
        }
    }
    if mesh_gpu_default_rt_pkg.index_buffer_extrl != vk::Buffer::null() {
        unsafe {
            device_extrl.destroy_buffer(mesh_gpu_default_rt_pkg.index_buffer_extrl, None);
            device_extrl.free_memory(mesh_gpu_default_rt_pkg.index_memory_extrl, None);
        }
    }
    if mesh_gpu_default_rt_pkg.instance_buffer_extrl != vk::Buffer::null() {
        unsafe {
            device_extrl.destroy_buffer(mesh_gpu_default_rt_pkg.instance_buffer_extrl, None);
            device_extrl.free_memory(mesh_gpu_default_rt_pkg.instance_memory_extrl, None);
        }
    }
    *mesh_gpu_default_rt_pkg = MeshGpuDefaultRtPkg::empty();
}
