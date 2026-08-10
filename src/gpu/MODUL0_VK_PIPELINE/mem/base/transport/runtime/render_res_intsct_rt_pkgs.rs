//! Base **render** resource intersection (M.BASE_RES_INTSCT · full pipeline surface · FIX-120).
//!
//! General pipeline bags + triangle aliases for live assemble compatibility.
//! ¬ cross-file bag imports.

// ── General pipeline / pass / shaders ───────────────────────────────────────

/// Shader modules (any stage set).
pub struct ShaderModulesDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `shader_modules_extrl` (`shader_modules` peel).
    pub shader_modules_extrl: Vec<ash::vk::ShaderModule>,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Render pass handle.
pub struct RenderPassDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `render_pass_extrl` (`render_pass` peel).
    pub render_pass_extrl: ash::vk::RenderPass,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Pipeline layout (descriptor set layouts + push constants binding).
pub struct PipelineLayoutDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `pipeline_layout_extrl` (`pipeline_layout` peel).
    pub pipeline_layout_extrl: ash::vk::PipelineLayout,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Graphics pipeline + layout used to create it.
pub struct GraphicsPipelineDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `pipeline_extrl` (`pipeline` peel).
    pub pipeline_extrl: ash::vk::Pipeline,
    /// External / raw Vulkan handle or host pointer field `pipeline_layout_extrl` (`pipeline_layout` peel).
    pub pipeline_layout_extrl: ash::vk::PipelineLayout,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Compute pipeline + layout.
pub struct ComputePipelineDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `pipeline_extrl` (`pipeline` peel).
    pub pipeline_extrl: ash::vk::Pipeline,
    /// External / raw Vulkan handle or host pointer field `pipeline_layout_extrl` (`pipeline_layout` peel).
    pub pipeline_layout_extrl: ash::vk::PipelineLayout,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Optional pipeline cache.
pub struct PipelineCacheDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `pipeline_cache_extrl` (`pipeline_cache` peel).
    pub pipeline_cache_extrl: ash::vk::PipelineCache,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

// ── Descriptor / sampler family (same intersection: bind to layout/pipeline) ─

/// Descriptor set layout.
pub struct DescriptorSetLayoutDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `descriptor_set_layout_extrl` (`descriptor_set_layout` peel).
    pub descriptor_set_layout_extrl: ash::vk::DescriptorSetLayout,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Descriptor pool.
pub struct DescriptorPoolDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `descriptor_pool_extrl` (`descriptor_pool` peel).
    pub descriptor_pool_extrl: ash::vk::DescriptorPool,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Allocated descriptor sets.
pub struct DescriptorSetsDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `descriptor_sets_extrl` (`descriptor_sets` peel).
    pub descriptor_sets_extrl: Vec<ash::vk::DescriptorSet>,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Sampler.
pub struct SamplerDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `sampler_extrl` (`sampler` peel).
    pub sampler_extrl: ash::vk::Sampler,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

// ── Capstone cargo (triangle proof path fields; general names preferred long-term) ─

/// Conv-assembled renderer cargo — triangle etalon + product cad_steel solid.
pub struct RendererDefaultRtCrg {
    /// Nested package bag field `shaders_triangle_rt_pkg`.
    pub shaders_triangle_rt_pkg: ShaderModulesDefaultRtPkg,
    /// Nested package bag field `render_pass_triangle_rt_pkg`.
    pub render_pass_triangle_rt_pkg: RenderPassDefaultRtPkg,
    /// Nested package bag field `pipeline_triangle_rt_pkg`.
    pub pipeline_triangle_rt_pkg: GraphicsPipelineDefaultRtPkg,
    /// Product CAD solid (pos+nrm VBO · cad_steel.spv · push constants).
    pub shaders_steel_rt_pkg: Option<ShaderModulesDefaultRtPkg>,
    /// Nested package bag field `pipeline_steel_rt_pkg`.
    pub pipeline_steel_rt_pkg: Option<GraphicsPipelineDefaultRtPkg>,
    /// Grid / sketch / outline lines (pos VBO · cad_line.spv).
    pub shaders_line_rt_pkg: Option<ShaderModulesDefaultRtPkg>,
    /// Nested package bag field `pipeline_line_rt_pkg`.
    pub pipeline_line_rt_pkg: Option<GraphicsPipelineDefaultRtPkg>,
    /// Thick outline ribbons (TRIANGLE_LIST · same cad_line shaders).
    pub pipeline_line_tris_rt_pkg: Option<GraphicsPipelineDefaultRtPkg>,
    /// Optional descriptor surface (empty until product wires it).
    pub descriptor_set_layout_default_rt_pkg: Option<DescriptorSetLayoutDefaultRtPkg>,
    /// Nested package bag field `descriptor_pool_default_rt_pkg`.
    pub descriptor_pool_default_rt_pkg: Option<DescriptorPoolDefaultRtPkg>,
    /// Nested package bag field `descriptor_sets_default_rt_pkg`.
    pub descriptor_sets_default_rt_pkg: Option<DescriptorSetsDefaultRtPkg>,
    /// Nested package bag field `sampler_default_rt_pkg`.
    pub sampler_default_rt_pkg: Option<SamplerDefaultRtPkg>,
    /// Nested package bag field `pipeline_cache_default_rt_pkg`.
    pub pipeline_cache_default_rt_pkg: Option<PipelineCacheDefaultRtPkg>,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

// ── Triangle aliases (live assemble / protocol re-exports) ───────────────────

/// `ShadersTriangleRtPkg` — type alias (shaders triangle rt pkg).
/// Defined in `gpu/MODUL0_VK_PIPELINE/mem/base/transport/runtime`.
pub type ShadersTriangleRtPkg = ShaderModulesDefaultRtPkg;
/// `RenderPassTriangleRtPkg` — type alias (render pass triangle rt pkg).
/// Defined in `gpu/MODUL0_VK_PIPELINE/mem/base/transport/runtime`.
pub type RenderPassTriangleRtPkg = RenderPassDefaultRtPkg;
/// `PipelineTriangleRtPkg` — type alias (pipeline triangle rt pkg).
/// Defined in `gpu/MODUL0_VK_PIPELINE/mem/base/transport/runtime`.
pub type PipelineTriangleRtPkg = GraphicsPipelineDefaultRtPkg;
