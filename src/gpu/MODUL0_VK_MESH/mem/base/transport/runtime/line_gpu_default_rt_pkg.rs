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
    /// Line segment count (`vertex_count` / 2) when `LINE_LIST`.
    pub line_count_rt: u32,
    /// Runtime phase field `color_rt`.
    pub color_rt: [f32; 4],
    /// true → draw as `TRIANGLE_LIST` (solid thick outline quads).
    pub as_tris_rt: bool,
    /// Runtime phase field `ready_rt`.
    pub ready_rt: bool,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}


/// `line` push constants (mat4 mvp + vec4 color = 80 bytes).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LinePushRt {
    /// Public field `mvp`.
    pub mvp: [f32; 16],
    /// Public field `color`.
    pub color: [f32; 4],
}

