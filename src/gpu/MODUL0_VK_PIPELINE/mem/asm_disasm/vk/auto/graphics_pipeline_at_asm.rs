//! vk brick: `vk::Pipeline` (graphics) from full create-info.

use ash::vk;
use ash::Device;

use crate::ModulResult;

/// Catalog — create one graphics pipeline (flexible create info + optional cache).
pub trait GraphicsPipelineAuto {
    fn auto_assemble(
        device_extrl: &Device,
        pipeline_cache_extrl: vk::PipelineCache,
        create_info_extrl: &vk::GraphicsPipelineCreateInfo<'_>,
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

impl GraphicsPipelineAuto for vk::Pipeline {
    fn auto_assemble(
        device_extrl: &Device,
        pipeline_cache_extrl: vk::PipelineCache,
        create_info_extrl: &vk::GraphicsPipelineCreateInfo<'_>,
    ) -> ModulResult<Self> {
        let pipelines = unsafe {
            device_extrl.create_graphics_pipelines(
                pipeline_cache_extrl,
                std::slice::from_ref(create_info_extrl),
                None,
            )
        }
        .map_err(|e| format!("graphics pipeline creation failed: {:?}", e.1))?;
        pipelines
            .into_iter()
            .next()
            .ok_or_else(|| "create_graphics_pipelines returned empty".into())
    }
}
