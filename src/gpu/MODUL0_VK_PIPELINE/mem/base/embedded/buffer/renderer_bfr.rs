//! `RendererBfr` — render-lane atom slots + cargo (type only).

use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::ComputePipelineDefaultRtPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::DescriptorPoolDefaultRtPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::DescriptorSetLayoutDefaultRtPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::DescriptorSetsDefaultRtPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::GraphicsPipelineDefaultRtPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::RenderPassDefaultRtPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::RendererDefaultRtCrg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::ShaderModulesDefaultRtPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::render_res_intsct_stp_pkgs::PipelineTriangleStpPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::render_res_intsct_stp_pkgs::RenderPassTriangleStpPkg;

/// `RendererBfr` — buffer / warehouse bag (renderer bfr).
///
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
    /// Nested package bag field `shaders_mesh_solid_rt_pkg`.
    pub shaders_mesh_solid_rt_pkg: Option<ShaderModulesDefaultRtPkg>,
    /// Nested package bag field `pipeline_mesh_solid_rt_pkg`.
    pub pipeline_mesh_solid_rt_pkg: Option<GraphicsPipelineDefaultRtPkg>,
    /// Nested package bag field `shaders_line_rt_pkg`.
    pub shaders_line_rt_pkg: Option<ShaderModulesDefaultRtPkg>,
    /// Nested package bag field `pipeline_line_rt_pkg`.
    pub pipeline_line_rt_pkg: Option<GraphicsPipelineDefaultRtPkg>,
    /// Nested package bag field `pipeline_line_tris_rt_pkg`.
    pub pipeline_line_tris_rt_pkg: Option<GraphicsPipelineDefaultRtPkg>,
    /// SoA pulse compute (`vkCmdDispatch`).
    pub pipeline_mesh_soa_comp_rt_pkg: Option<ComputePipelineDefaultRtPkg>,
    pub pipeline_soa_heat_comp_rt_pkg: Option<ComputePipelineDefaultRtPkg>,
    /// SoA STORAGE set layout (rest + world).
    pub descriptor_set_layout_default_rt_pkg: Option<DescriptorSetLayoutDefaultRtPkg>,
    /// SoA descriptor pool.
    pub descriptor_pool_default_rt_pkg: Option<DescriptorPoolDefaultRtPkg>,
    /// SoA descriptor sets.
    pub descriptor_sets_default_rt_pkg: Option<DescriptorSetsDefaultRtPkg>,
    /// Runtime phase field `cargo_rt`.
    pub cargo_rt: Option<RendererDefaultRtCrg>,
}
