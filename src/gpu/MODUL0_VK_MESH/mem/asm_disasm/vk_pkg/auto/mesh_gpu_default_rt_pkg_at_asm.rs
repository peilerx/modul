//! Auto seed · `MeshGpuDefaultRtPkg` zero bag (no device · no peels).
//!
//! Constructor of the runtime bag lives here — not in `proc/` (FIX-136).

use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::MeshGpuDefaultRtPkg;
use ash::vk;

/// Catalog — empty mesh GPU runtime bag (handles null · counts 0).
pub trait MeshGpuDefaultRtAuto {
    fn auto_assemble() -> Self;
}

impl MeshGpuDefaultRtAuto for MeshGpuDefaultRtPkg {
    fn auto_assemble() -> Self {
        Self {
            vertex_buffer_extrl: vk::Buffer::null(),
            index_buffer_extrl: vk::Buffer::null(),
            vertex_memory_extrl: vk::DeviceMemory::null(),
            index_memory_extrl: vk::DeviceMemory::null(),
            instance_buffer_extrl: vk::Buffer::null(),
            instance_memory_extrl: vk::DeviceMemory::null(),
            soa_world_buffer_extrl: vk::Buffer::null(),
            soa_world_memory_extrl: vk::DeviceMemory::null(),
            vertex_count_rt: 0,
            index_count_rt: 0,
            instance_count_rt: 0,
            instance_capacity_rt: 0,
            triangle_count_rt: 0,
            mode_rt: 0,
            base_r_rt: 0.70,
            base_g_rt: 0.725,
            base_b_rt: 0.765,
            bounds_min_rt: [0.0; 3],
            bounds_max_rt: [0.0; 3],
            ready_rt: false,
            desc: "mesh_gpu_empty",
        }
    }
}
