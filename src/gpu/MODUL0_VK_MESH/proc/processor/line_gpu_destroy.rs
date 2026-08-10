//! Line GPU buffer destroy (P · null checks).

use ash::vk;
use ash::Device;

use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::line_gpu_default_rt_pkg::LineGpuDefaultRtPkg;

/// `destroy_line_gpu_buffers` — function (destroy line gpu buffers).
/// Public API entry for this module.
/// Belongs to: mesh upload / solid draw MCG.
pub fn destroy_line_gpu_buffers(
    device_extrl: &Device,
    line_gpu_default_rt_pkg: &mut LineGpuDefaultRtPkg,
) {
    if line_gpu_default_rt_pkg.vertex_buffer_extrl != vk::Buffer::null() {
        unsafe {
            device_extrl.destroy_buffer(line_gpu_default_rt_pkg.vertex_buffer_extrl, None);
            device_extrl.free_memory(line_gpu_default_rt_pkg.vertex_memory_extrl, None);
        }
        *line_gpu_default_rt_pkg = LineGpuDefaultRtPkg::empty(line_gpu_default_rt_pkg.desc);
    }
}
