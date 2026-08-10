//! `RendererBfr` — render-lane atom slots + cargo (type only).

use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::GraphicsPipelineDefaultRtPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::RenderPassDefaultRtPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::RendererDefaultRtCrg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::ShaderModulesDefaultRtPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::render_res_intsct_stp_pkgs::PipelineTriangleStpPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::render_res_intsct_stp_pkgs::RenderPassTriangleStpPkg;

/// `RendererBfr` — buffer / warehouse bag (renderer bfr).
/// Memory-layer bag: owned fields, no product control flow.
/// Belongs to: render-pass / graphics pipeline MCG.
/// Module path context: `gpu/MODUL0_VK_PIPELINE/mem/base/embedded/buffer`.
pub struct RendererBfr {
    /// Nested package bag field `render_pass_triangle_stp_pkg`.
    pub render_pass_triangle_stp_pkg: Option<RenderPassTriangleStpPkg>,
    /// Nested package bag field `pipeline_triangle_stp_pkg`.
    pub pipeline_triangle_stp_pkg: Option<PipelineTriangleStpPkg>,
    /// Nested package bag field `shaders_triangle_rt_pkg`.
    pub shaders_triangle_rt_pkg: Option<ShaderModulesDefaultRtPkg>,
    /// Nested package bag field `render_pass_triangle_rt_pkg`.
    pub render_pass_triangle_rt_pkg: Option<RenderPassDefaultRtPkg>,
    /// Nested package bag field `pipeline_triangle_rt_pkg`.
    pub pipeline_triangle_rt_pkg: Option<GraphicsPipelineDefaultRtPkg>,
    /// Nested package bag field `shaders_steel_rt_pkg`.
    pub shaders_steel_rt_pkg: Option<ShaderModulesDefaultRtPkg>,
    /// Nested package bag field `pipeline_steel_rt_pkg`.
    pub pipeline_steel_rt_pkg: Option<GraphicsPipelineDefaultRtPkg>,
    /// Nested package bag field `shaders_line_rt_pkg`.
    pub shaders_line_rt_pkg: Option<ShaderModulesDefaultRtPkg>,
    /// Nested package bag field `pipeline_line_rt_pkg`.
    pub pipeline_line_rt_pkg: Option<GraphicsPipelineDefaultRtPkg>,
    /// Nested package bag field `pipeline_line_tris_rt_pkg`.
    pub pipeline_line_tris_rt_pkg: Option<GraphicsPipelineDefaultRtPkg>,
    /// Runtime phase field `cargo_rt`.
    pub cargo_rt: Option<RendererDefaultRtCrg>,
}
