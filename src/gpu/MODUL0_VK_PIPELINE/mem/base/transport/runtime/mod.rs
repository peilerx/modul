//! PIPELINE transport runtime — **render_res_intsct** (full pipeline surface).
pub mod render_res_intsct_rt_pkgs;

pub use render_res_intsct_rt_pkgs::{
    ComputePipelineDefaultRtPkg, DescriptorPoolDefaultRtPkg, DescriptorSetLayoutDefaultRtPkg,
    DescriptorSetsDefaultRtPkg, GraphicsPipelineDefaultRtPkg, PipelineCacheDefaultRtPkg,
    PipelineLayoutDefaultRtPkg, PipelineTriangleRtPkg, RenderPassDefaultRtPkg,
    RenderPassTriangleRtPkg, RendererDefaultRtCrg, SamplerDefaultRtPkg, ShaderModulesDefaultRtPkg,
    ShadersTriangleRtPkg,
};
