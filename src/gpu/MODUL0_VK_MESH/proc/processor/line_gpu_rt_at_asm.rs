//! LineGpu / LinePush peels · P.Processor.

use ash::vk;

use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::line_gpu_default_rt_pkg::{
    LineGpuDefaultRtPkg, LinePushRt,
};

/// Push constant block size for line shaders.
pub const LINE_PUSH_RT_SIZE: u32 = core::mem::size_of::<LinePushRt>() as u32;

impl LineGpuDefaultRtPkg {
    #[must_use]
    pub const fn empty(desc: &'static str) -> Self {
        Self {
            vertex_buffer_extrl: vk::Buffer::null(),
            vertex_memory_extrl: vk::DeviceMemory::null(),
            vertex_count_rt: 0,
            line_count_rt: 0,
            color_rt: [0.55, 0.58, 0.62, 1.0],
            as_tris_rt: false,
            ready_rt: false,
            desc,
        }
    }
}

impl LinePushRt {
    #[must_use]
    pub const fn from_mvp_color(mvp: [f32; 16], color: [f32; 4]) -> Self {
        Self { mvp, color }
    }
}
