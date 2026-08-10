//! Port module `gpu/MODUL0_VK_PIPELINE/mem/base/transport`.
//!
//! PTP import/export free functions (FIX-128/130).

pub mod prt;
/// Submodule `runtime`.
/// Part of `gpu/MODUL0_VK_PIPELINE/mem/base/transport` under the mem/conv/proc MCG canon.
pub mod runtime;
/// Submodule `setup`.
/// Part of `gpu/MODUL0_VK_PIPELINE/mem/base/transport` under the mem/conv/proc MCG canon.
pub mod setup;

pub use prt::RenderLanePrt;
pub use runtime::{
    ComputePipelineDefaultRtPkg, DescriptorPoolDefaultRtPkg, DescriptorSetLayoutDefaultRtPkg,
    DescriptorSetsDefaultRtPkg, GraphicsPipelineDefaultRtPkg, PipelineCacheDefaultRtPkg,
    PipelineLayoutDefaultRtPkg, PipelineTriangleRtPkg, RenderPassDefaultRtPkg,
    RenderPassTriangleRtPkg, RendererDefaultRtCrg, SamplerDefaultRtPkg, ShaderModulesDefaultRtPkg,
    ShadersTriangleRtPkg,
};
pub use setup::{
    RenderPassAttachmentLayoutStpPkgOp,
    DescriptorPoolDefaultStpPkg, DescriptorSetLayoutDefaultStpPkg, GraphicsPipelineDefaultStpPkg,
    PipelineTriangleStpPkg, RenderPassDefaultStpPkg, RenderPassTriangleStpPkg, SamplerDefaultStpPkg,
};
