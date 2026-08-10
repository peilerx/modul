//! SPIR-V shader module assembly.

use ash::vk;
use ash::Device;

use super::{map_vk, ModulResult};

/// Create a [`vk::ShaderModule`] from raw SPIR-V **bytes** (little-endian words).
///
/// Used by pipeline catalog leaves that `include_bytes!` product shaders
/// (`shader/cubes.vert.spv`, `shader/cubes.frag.spv`).
///
/// # Errors
///
/// - Empty code
/// - Length not a multiple of 4
/// - Vulkan `create_shader_module` failure
#[expect(clippy::cast_ptr_alignment, reason = "SPIR-V bytecode alignment per Vulkan spec")]
#[inline]
pub fn assemble_shader_spv(device_extrl: &Device, code: &[u8]) -> ModulResult<vk::ShaderModule> {
    if code.is_empty() {
        return Err("Shader code empty".into());
    }
    if !code.len().is_multiple_of(4) {
        return Err("Shader code length not multiple of 4".into());
    }
    let words: Vec<u32> = code
        .chunks_exact(4)
        .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect();
    let info = vk::ShaderModuleCreateInfo::default().code(&words);
    map_vk(unsafe { device_extrl.create_shader_module(&info, None) })
}
