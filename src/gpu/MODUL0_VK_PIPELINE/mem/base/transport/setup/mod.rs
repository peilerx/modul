//! PIPELINE transport setup — render_res_intsct + op.
pub mod op;
/// Submodule `render_res_intsct_stp_pkgs`.
/// Part of `gpu/MODUL0_VK_PIPELINE/mem/base/transport/setup` under the mem/conv/proc MCG canon.
pub mod render_res_intsct_stp_pkgs;

pub use op::RenderPassAttachmentLayoutStpPkgOp;
pub use render_res_intsct_stp_pkgs::{
    DescriptorPoolDefaultStpPkg, DescriptorSetLayoutDefaultStpPkg, GraphicsPipelineDefaultStpPkg,
    PipelineTriangleStpPkg, RenderPassDefaultStpPkg, RenderPassTriangleStpPkg, SamplerDefaultStpPkg,
};
