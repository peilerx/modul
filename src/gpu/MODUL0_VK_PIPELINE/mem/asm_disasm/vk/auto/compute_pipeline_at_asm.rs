//! vk brick: `vk::Pipeline` (compute).

use ash::vk;
use ash::Device;

use crate::ModulResult;

/// Catalog — create one compute pipeline.
pub trait ComputePipelineAuto {
    fn auto_assemble(
        device_extrl: &Device,
        pipeline_cache_extrl: vk::PipelineCache,
        create_info_extrl: &vk::ComputePipelineCreateInfo<'_>,
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

impl ComputePipelineAuto for vk::Pipeline {
    fn auto_assemble(
        device_extrl: &Device,
        pipeline_cache_extrl: vk::PipelineCache,
        create_info_extrl: &vk::ComputePipelineCreateInfo<'_>,
    ) -> ModulResult<Self> {
        let pipelines = unsafe {
            device_extrl.create_compute_pipelines(
                pipeline_cache_extrl,
                std::slice::from_ref(create_info_extrl),
                None,
            )
        }
        .map_err(|e| format!("compute pipeline creation failed: {:?}", e.1))?;
        pipelines
            .into_iter()
            .next()
            .ok_or_else(|| "create_compute_pipelines returned empty".into())
    }
}
