//! MeshGpuDefaultRtPkg peels (empty · center · radius) · P.Processor.

use ash::vk;

use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::MeshGpuDefaultRtPkg;


impl MeshGpuDefaultRtPkg {
    /// `empty` — function (empty).
    /// Public API entry for this module.
    /// Belongs to: mesh upload / solid draw MCG.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            vertex_buffer_extrl: vk::Buffer::null(),
            index_buffer_extrl: vk::Buffer::null(),
            vertex_memory_extrl: vk::DeviceMemory::null(),
            index_memory_extrl: vk::DeviceMemory::null(),
            instance_buffer_extrl: vk::Buffer::null(),
            instance_memory_extrl: vk::DeviceMemory::null(),
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

    /// `center_rt` — function (center rt).
    /// Public API entry for this module.
    /// Belongs to: mesh upload / solid draw MCG.
    #[inline]
    #[must_use]
    pub fn center_rt(&self) -> [f32; 3] {
        [
            0.5 * (self.bounds_min_rt[0] + self.bounds_max_rt[0]),
            0.5 * (self.bounds_min_rt[1] + self.bounds_max_rt[1]),
            0.5 * (self.bounds_min_rt[2] + self.bounds_max_rt[2]),
        ]
    }

    /// `radius_rt` — function (radius rt).
    /// Public API entry for this module.
    /// Belongs to: mesh upload / solid draw MCG.
    #[inline]
    #[must_use]
    pub fn radius_rt(&self) -> f32 {
        let c = self.center_rt();
        let dx = (self.bounds_max_rt[0] - c[0]).abs();
        let dy = (self.bounds_max_rt[1] - c[1]).abs();
        let dz = (self.bounds_max_rt[2] - c[2]).abs();
        dz.mul_add(dz, dy.mul_add(dy, dx * dx)).sqrt().max(0.5)
    }
}
