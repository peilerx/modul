//! GPU mesh cargo — VBO/IBO for **cubes/steel** solid (pos+nrm interleaved).

use ash::vk;

/// Device mesh buffers for the solid instanced path.
///
/// Holds vertex/index/instance buffers, draw counts, AABB, and steel albedo.
/// Built via `MeshGpuBfr` port + `MeshDrawPrt::SOLID`.
pub struct MeshGpuDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `vertex_buffer_extrl` (`vertex_buffer` peel).
    pub vertex_buffer_extrl: vk::Buffer,
    /// External / raw Vulkan handle or host pointer field `index_buffer_extrl` (`index_buffer` peel).
    pub index_buffer_extrl: vk::Buffer,
    /// External / raw Vulkan handle or host pointer field `vertex_memory_extrl` (`vertex_memory` peel).
    pub vertex_memory_extrl: vk::DeviceMemory,
    /// External / raw Vulkan handle or host pointer field `index_memory_extrl` (`index_memory` peel).
    pub index_memory_extrl: vk::DeviceMemory,
    /// SoA rest `x[n]|y[n]|z[n]|lod[n]` · STORAGE · compute read.
    pub instance_buffer_extrl: vk::Buffer,
    /// External / raw Vulkan handle or host pointer field `instance_memory_extrl` (`instance_memory` peel).
    pub instance_memory_extrl: vk::DeviceMemory,
    /// SoA world after `vkCmdDispatch` · STORAGE · vertex read.
    pub soa_world_buffer_extrl: vk::Buffer,
    /// Device memory of `soa_world_buffer_extrl`.
    pub soa_world_memory_extrl: vk::DeviceMemory,
    /// Interleaved pos+nrm vertices uploaded.
    pub vertex_count_rt: u32,
    /// Runtime phase field `index_count_rt`.
    pub index_count_rt: u32,
    /// `cmd_draw_indexed` instanceCount (≥1 when ready).
    pub instance_count_rt: u32,
    /// Capacity of instance buffer in instances (host re-upload ≤ this).
    pub instance_capacity_rt: u32,
    /// Runtime phase field `triangle_count_rt`.
    pub triangle_count_rt: u32,
    /// 3 = `Solid` · 1 = raw tri list · 0 = empty.
    pub mode_rt: u32,
    /// Runtime phase field `base_r_rt`.
    pub base_r_rt: f32,
    /// Runtime phase field `base_g_rt`.
    pub base_g_rt: f32,
    /// Runtime phase field `base_b_rt`.
    pub base_b_rt: f32,
    /// World AABB of uploaded mesh (orbit / fit).
    pub bounds_min_rt: [f32; 3],
    /// Runtime phase field `bounds_max_rt`.
    pub bounds_max_rt: [f32; 3],
    /// Runtime phase field `ready_rt`.
    pub ready_rt: bool,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}


/// Push constants for `shader/cubes.{vert,frag}` — **160 bytes**.
///
/// Layout must match the GLSL `Pc` block (mat4 + 6×vec4).
///
/// | Field | Vertex pulse (`look3`) | Fragment lighting |
/// |-------|------------------------|-------------------|
/// | `mvp` | transform | — |
/// | `light_dir` | — | key dir + intensity |
/// | `base_color` | — | albedo + roughness |
/// | `cam_pos` | eye for shading | exposure in `w` |
/// | `look` / `look2` | — | GGX look knobs |
/// | `look3` | **time, sep_max, y_half, period** | frag uses fixed cavity defaults |
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MeshPushRt {
    /// Column-major 4×4 MVP.
    pub mvp: [f32; 16],
    /// xyz key dir · w key intensity.
    pub light_dir: [f32; 4],
    /// rgb albedo · w roughness.
    pub base_color: [f32; 4],
    /// xyz eye · w exposure.
    pub cam_pos: [f32; 4],
    /// f0, specular, env, fill.
    pub look: [f32; 4],
    /// rim, brush, film, contrast.
    pub look2: [f32; 4],
    /// **Pulse** (vert): time, `sep_max`, `y_half`, period — not GGX cavity in the cubes path.
    pub look3: [f32; 4],
}
