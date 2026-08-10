//! vk brick: `vk::ShaderModule` from SPIR-V bytes.

use ash::vk;
use ash::Device;

use crate::{assemble_shader_spv, ModulResult};

/// Catalog — create one shader module from SPIR-V.
pub trait ShaderSpvAuto {
    fn auto_assemble(device_extrl: &Device, spirv_code_extrl: &[u8]) -> ModulResult<Self>
    where
        Self: Sized;
}

impl ShaderSpvAuto for vk::ShaderModule {
    fn auto_assemble(device_extrl: &Device, spirv_code_extrl: &[u8]) -> ModulResult<Self> {
        assemble_shader_spv(device_extrl, spirv_code_extrl)
    }
}
