//! Auto seed · `LineGpuDefaultRtPkg` / `LinePushRt` (no device).
//!
//! Constructor of the runtime bags lives here — not in `proc/` (FIX-136).

use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::line_gpu_default_rt_pkg::{
    LineGpuDefaultRtPkg, LinePushRt,
};
use ash::vk;

/// Catalog — empty line GPU runtime bag (handles null · counts 0).
pub trait LineGpuDefaultRtAuto {
    fn auto_assemble(desc: &'static str) -> Self;
}

impl LineGpuDefaultRtAuto for LineGpuDefaultRtPkg {
    fn auto_assemble(desc: &'static str) -> Self {
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

/// Catalog — line push from MVP + color peels (Rt-only · Auto).
pub trait LinePushDefaultAuto {
    fn auto_assemble(mvp_rt: [f32; 16], color_rt: [f32; 4]) -> Self;
}

impl LinePushDefaultAuto for LinePushRt {
    fn auto_assemble(mvp_rt: [f32; 16], color_rt: [f32; 4]) -> Self {
        Self {
            mvp: mvp_rt,
            color: color_rt,
        }
    }
}
