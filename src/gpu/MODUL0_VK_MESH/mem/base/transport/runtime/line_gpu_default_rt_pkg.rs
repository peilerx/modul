//! GPU line list cargo (grid · sketch · outline) — pos.xyz VBO.

use ash::vk;

/// Host-visible line/outline buffer (stride 12 · R32G32B32).
pub struct LineGpuDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `vertex_buffer_extrl` (`vertex_buffer` peel).
    pub vertex_buffer_extrl: vk::Buffer,
    /// External / raw Vulkan handle or host pointer field `vertex_memory_extrl` (`vertex_memory` peel).
    pub vertex_memory_extrl: vk::DeviceMemory,
    /// Runtime phase field `vertex_count_rt`.
    pub vertex_count_rt: u32,
    /// Line segment count (vertex_count / 2) when LINE_LIST.
    pub line_count_rt: u32,
    /// Runtime phase field `color_rt`.
    pub color_rt: [f32; 4],
    /// true → draw as TRIANGLE_LIST (solid thick outline quads).
    pub as_tris_rt: bool,
    /// Runtime phase field `ready_rt`.
    pub ready_rt: bool,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

impl LineGpuDefaultRtPkg {
    /// `empty` — function (empty).
    /// Public API entry for this module.
    /// Belongs to: mesh upload / solid draw MCG.
    pub fn empty(desc: &'static str) -> Self {
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

/// cad_line push constants (mat4 mvp + vec4 color = 80 bytes).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CadLinePushRt {
    /// Public field `mvp`.
    pub mvp: [f32; 16],
    /// Public field `color`.
    pub color: [f32; 4],
}

impl CadLinePushRt {
    /// `SIZE` — const (SIZE).
    /// Module path context: `gpu/MODUL0_VK_MESH/mem/base/transport/runtime`.
    pub const SIZE: u32 = 80;

    /// `from_mvp_color` — function (from mvp color).
    /// Conversion / mapping helper.
    /// Belongs to: mesh upload / solid draw MCG.
    pub fn from_mvp_color(mvp: [f32; 16], color: [f32; 4]) -> Self {
        Self { mvp, color }
    }
}
