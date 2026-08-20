//! # Common — shared helpers (all lanes)
//!
//! Non-MCG utilities used by GPU factories and apps:
//!
//! - `ModulResult` / `map_vk` / `from_err` — error surface without `dyn`
//! - `assemble_shader_spv` — create a `vk::ShaderModule` from SPIR-V bytes
//! - `find_vk_memory_type` — pick a device memory type index
//! - `protocol` — shared protocol peels / GPU protocol re-exports
//! - `trace_rt` — optional host-side tracing (env-gated)
//!
//! These symbols are also re-exported at the crate root for ergonomic `use modul::…`.

/// Vulkan memory type selection.
pub mod memory_type;
/// Result alias and Vulkan error mapping.
pub mod modul_result;
/// Shared protocol / GPU protocol types.
pub mod protocol;
/// SPIR-V → `ShaderModule` helper.
pub mod shader_spv;
/// Optional debug tracing runtime.
pub mod trace_rt;

pub use memory_type::{find_vk_memory_type, pick_vk_memory_type_vram_then_host};
pub use modul_result::{from_err, map_vk, ModulResult};
pub use shader_spv::assemble_shader_spv;
pub use trace_rt::{
    trace_deep, trace_emit, trace_enabled, trace_init_from_env, trace_mesh_stats, trace_paint_diag,
    trace_set_enabled, trace_sketch_loop, trace_throttle, trace_zbuffer_coverage,
};
