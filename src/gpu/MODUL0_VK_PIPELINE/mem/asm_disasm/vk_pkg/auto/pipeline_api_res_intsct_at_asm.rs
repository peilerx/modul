//! `vk_pkg` — pack full pipeline API bags from **vk** primitives (FIX-120).
//! Flexible surface beyond triangle: descriptors · sampler · cache · compute.

use ash::vk;
use ash::Device;

use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::{
    ComputePipelineDefaultRtPkg, DescriptorPoolDefaultRtPkg, DescriptorSetLayoutDefaultRtPkg,
    DescriptorSetsDefaultRtPkg, PipelineCacheDefaultRtPkg, PipelineLayoutDefaultRtPkg,
    SamplerDefaultRtPkg, ShaderModulesDefaultRtPkg,
};
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk::auto::compute_pipeline_at_asm::ComputePipelineAuto;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk::handled::descriptor_pool_hld_asm::DescriptorPoolHandled;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk::auto::descriptor_set_layout_at_asm::DescriptorSetLayoutAuto;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk::auto::descriptor_sets_at_asm::DescriptorSetsAllocateAuto;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk::auto::pipeline_cache_res_intsct_at_asm::PipelineCacheAuto;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk::auto::pipeline_layout_res_intsct_at_asm::PipelineLayoutAuto;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk::handled::sampler_hld_asm::SamplerHandled;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk::auto::shader_spv_at_asm::ShaderSpvAuto;
use crate::ModulResult;

// ── Traits hot → cold ───────────────────────────────────────────────────────

/// Pack N shader modules from SPIR-V slices (any stages).
pub trait ShaderModulesDefaultAuto {
    fn auto_assemble(device_extrl: &Device, spirv_codes_extrl: &[&[u8]]) -> ModulResult<Self>
    where
        Self: Sized;
}

/// Pack pipeline layout from set layouts + push ranges.
pub trait PipelineLayoutDefaultAuto {
    fn auto_assemble(
        device_extrl: &Device,
        set_layouts_extrl: &[vk::DescriptorSetLayout],
        push_constant_ranges_extrl: &[vk::PushConstantRange],
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

/// Pack descriptor set layout.
pub trait DescriptorSetLayoutDefaultAuto {
    fn auto_assemble(
        device_extrl: &Device,
        bindings_extrl: &[vk::DescriptorSetLayoutBinding<'_>],
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

/// Pack descriptor pool.
pub trait DescriptorPoolDefaultHandled {
    fn handled_assemble(
        device_extrl: &Device,
        max_sets_stp: u32,
        pool_sizes_extrl: &[vk::DescriptorPoolSize],
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

/// Pack allocated descriptor sets.
pub trait DescriptorSetsDefaultAuto {
    fn auto_assemble(
        device_extrl: &Device,
        descriptor_pool_extrl: vk::DescriptorPool,
        set_layouts_extrl: &[vk::DescriptorSetLayout],
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

/// Pack sampler.
pub trait SamplerDefaultHandled {
    fn handled_assemble(
        device_extrl: &Device,
        mag_filter_op: vk::Filter,
        min_filter_op: vk::Filter,
        address_mode_u_op: vk::SamplerAddressMode,
        address_mode_v_op: vk::SamplerAddressMode,
        address_mode_w_op: vk::SamplerAddressMode,
        anisotropy_enable_stp: bool,
        max_anisotropy_stp: f32,
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

/// Pack empty or seeded pipeline cache.
pub trait PipelineCacheDefaultAuto {
    fn auto_assemble(device_extrl: &Device) -> ModulResult<Self>
    where
        Self: Sized;
}

/// Pack pipeline cache from bytes.
pub trait PipelineCacheDefaultSeededAuto {
    fn auto_assemble(device_extrl: &Device, initial_data_extrl: &[u8]) -> ModulResult<Self>
    where
        Self: Sized;
}

/// Pack compute pipeline.
pub trait ComputePipelineDefaultAuto {
    fn auto_assemble(
        device_extrl: &Device,
        pipeline_cache_extrl: vk::PipelineCache,
        create_info_extrl: &vk::ComputePipelineCreateInfo<'_>,
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

// ── Impls ───────────────────────────────────────────────────────────────────

impl ShaderModulesDefaultAuto for ShaderModulesDefaultRtPkg {
    fn auto_assemble(device_extrl: &Device, spirv_codes_extrl: &[&[u8]]) -> ModulResult<Self> {
        let shader_modules_extrl = spirv_codes_extrl
            .iter()
            .map(|code| vk::ShaderModule::auto_assemble(device_extrl, code))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            shader_modules_extrl,
            desc: "shader_modules",
        })
    }
}

impl PipelineLayoutDefaultAuto for PipelineLayoutDefaultRtPkg {
    fn auto_assemble(
        device_extrl: &Device,
        set_layouts_extrl: &[vk::DescriptorSetLayout],
        push_constant_ranges_extrl: &[vk::PushConstantRange],
    ) -> ModulResult<Self> {
        let pipeline_layout_extrl = vk::PipelineLayout::auto_assemble(
            device_extrl,
            set_layouts_extrl,
            push_constant_ranges_extrl,
        )?;
        Ok(Self {
            pipeline_layout_extrl,
            desc: "pipeline_layout",
        })
    }
}

impl DescriptorSetLayoutDefaultAuto for DescriptorSetLayoutDefaultRtPkg {
    fn auto_assemble(
        device_extrl: &Device,
        bindings_extrl: &[vk::DescriptorSetLayoutBinding<'_>],
    ) -> ModulResult<Self> {
        let descriptor_set_layout_extrl =
            vk::DescriptorSetLayout::auto_assemble(device_extrl, bindings_extrl)?;
        Ok(Self {
            descriptor_set_layout_extrl,
            desc: "descriptor_set_layout",
        })
    }
}

impl DescriptorPoolDefaultHandled for DescriptorPoolDefaultRtPkg {
    fn handled_assemble(
        device_extrl: &Device,
        max_sets_stp: u32,
        pool_sizes_extrl: &[vk::DescriptorPoolSize],
    ) -> ModulResult<Self> {
        let descriptor_pool_extrl =
            vk::DescriptorPool::handled_assemble(device_extrl, max_sets_stp, pool_sizes_extrl)?;
        Ok(Self {
            descriptor_pool_extrl,
            desc: "descriptor_pool",
        })
    }
}

impl DescriptorSetsDefaultAuto for DescriptorSetsDefaultRtPkg {
    fn auto_assemble(
        device_extrl: &Device,
        descriptor_pool_extrl: vk::DescriptorPool,
        set_layouts_extrl: &[vk::DescriptorSetLayout],
    ) -> ModulResult<Self> {
        let descriptor_sets_extrl = <Vec<vk::DescriptorSet> as DescriptorSetsAllocateAuto>::auto_assemble(
            device_extrl,
            descriptor_pool_extrl,
            set_layouts_extrl,
        )?;
        Ok(Self {
            descriptor_sets_extrl,
            desc: "descriptor_sets",
        })
    }
}

impl SamplerDefaultHandled for SamplerDefaultRtPkg {
    fn handled_assemble(
        device_extrl: &Device,
        mag_filter_op: vk::Filter,
        min_filter_op: vk::Filter,
        address_mode_u_op: vk::SamplerAddressMode,
        address_mode_v_op: vk::SamplerAddressMode,
        address_mode_w_op: vk::SamplerAddressMode,
        anisotropy_enable_stp: bool,
        max_anisotropy_stp: f32,
    ) -> ModulResult<Self> {
        {
            let sampler_extrl = vk::Sampler::handled_assemble(
                device_extrl,
                mag_filter_op,
                min_filter_op,
                address_mode_u_op,
                address_mode_v_op,
                address_mode_w_op,
                anisotropy_enable_stp,
                max_anisotropy_stp,
            )?;
            Ok(Self {
                sampler_extrl,
                desc: "sampler",
            })
        }
    }
}

impl PipelineCacheDefaultAuto for PipelineCacheDefaultRtPkg {
    fn auto_assemble(device_extrl: &Device) -> ModulResult<Self> {
        let pipeline_cache_extrl = vk::PipelineCache::auto_assemble(device_extrl)?;
        Ok(Self {
            pipeline_cache_extrl,
            desc: "pipeline_cache",
        })
    }
}

impl PipelineCacheDefaultSeededAuto for PipelineCacheDefaultRtPkg {
    fn auto_assemble(device_extrl: &Device, initial_data_extrl: &[u8]) -> ModulResult<Self> {
        let pipeline_cache_extrl =
            <vk::PipelineCache as crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk::auto::pipeline_cache_res_intsct_at_asm::PipelineCacheSeededAuto>::auto_assemble(
                device_extrl,
                initial_data_extrl,
            )?;
        Ok(Self {
            pipeline_cache_extrl,
            desc: "pipeline_cache_seeded",
        })
    }
}



impl ComputePipelineDefaultAuto for ComputePipelineDefaultRtPkg {
    fn auto_assemble(
        device_extrl: &Device,
        pipeline_cache_extrl: vk::PipelineCache,
        create_info_extrl: &vk::ComputePipelineCreateInfo<'_>,
    ) -> ModulResult<Self> {
        let pipeline_extrl =
            vk::Pipeline::auto_assemble(device_extrl, pipeline_cache_extrl, create_info_extrl)?;
        Ok(Self {
            pipeline_extrl,
            pipeline_layout_extrl: create_info_extrl.layout,
            desc: "compute_pipeline",
        })
    }
}
