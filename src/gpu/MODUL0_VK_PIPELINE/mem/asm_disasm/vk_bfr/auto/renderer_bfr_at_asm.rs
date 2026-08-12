//! Catalog — empty seed + slots · `RendererBfrAuto`.

use crate::gpu::MODUL0_VK_PIPELINE::mem::base::embedded::buffer::RendererBfr;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::GraphicsPipelineDefaultRtPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::RenderPassDefaultRtPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::RendererDefaultRtCrg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::ShaderModulesDefaultRtPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::render_res_intsct_stp_pkgs::PipelineTriangleStpPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::render_res_intsct_stp_pkgs::RenderPassTriangleStpPkg;
use crate::ModulResult;

/// `RendererBfrAuto` — trait (renderer bfr auto).
///
/// Auto-assemble catalog trait: pure construction without external peels (FIX-129 Auto rank).
/// Belongs to: render-pass / graphics pipeline MCG.
/// Module path context: `gpu/MODUL0_VK_PIPELINE/mem/asm_disasm/vk_bfr/auto`.
pub trait RendererBfrAuto: Sized {
    fn auto_assemble() -> Self;
    fn slot_ref<'a, T>(slot: &'a Option<T>, name: &'static str) -> ModulResult<&'a T> {
        slot.as_ref()
            .ok_or_else(|| format!("renderer_bfr: slot `{name}` empty"))
    }
    fn slot_take<T>(slot: &mut Option<T>, name: &'static str) -> ModulResult<T> {
        slot.take()
            .ok_or_else(|| format!("renderer_bfr: slot `{name}` empty (take)"))
    }
    fn rp_stp(&self) -> ModulResult<&RenderPassTriangleStpPkg>;
    fn pl_stp(&self) -> ModulResult<&PipelineTriangleStpPkg>;
    fn shaders_tri(&self) -> ModulResult<&ShaderModulesDefaultRtPkg>;
    fn rp_rt(&self) -> ModulResult<&RenderPassDefaultRtPkg>;
    fn pl_tri(&self) -> ModulResult<&GraphicsPipelineDefaultRtPkg>;
    fn shaders_steel(&self) -> ModulResult<&ShaderModulesDefaultRtPkg>;
    fn pl_steel(&self) -> ModulResult<&GraphicsPipelineDefaultRtPkg>;
    fn shaders_line(&self) -> ModulResult<&ShaderModulesDefaultRtPkg>;
    fn pl_line(&self) -> ModulResult<&GraphicsPipelineDefaultRtPkg>;
    fn pl_line_tris(&self) -> ModulResult<&GraphicsPipelineDefaultRtPkg>;
    fn cargo(&self) -> ModulResult<&RendererDefaultRtCrg>;
}

impl RendererBfrAuto for RendererBfr {
    fn auto_assemble() -> Self {
        Self {
            render_pass_triangle_stp_pkg: None,
            pipeline_triangle_stp_pkg: None,
            shaders_triangle_rt_pkg: None,
            render_pass_triangle_rt_pkg: None,
            pipeline_triangle_rt_pkg: None,
            shaders_mesh_solid_rt_pkg: None,
            pipeline_mesh_solid_rt_pkg: None,
            shaders_line_rt_pkg: None,
            pipeline_line_rt_pkg: None,
            pipeline_line_tris_rt_pkg: None,
            cargo_rt: None,
        }
    }
    fn rp_stp(&self) -> ModulResult<&RenderPassTriangleStpPkg> {
        Self::slot_ref(
            &self.render_pass_triangle_stp_pkg,
            "render_pass_triangle_stp_pkg",
        )
    }
    fn pl_stp(&self) -> ModulResult<&PipelineTriangleStpPkg> {
        Self::slot_ref(
            &self.pipeline_triangle_stp_pkg,
            "pipeline_triangle_stp_pkg",
        )
    }
    fn shaders_tri(&self) -> ModulResult<&ShaderModulesDefaultRtPkg> {
        Self::slot_ref(&self.shaders_triangle_rt_pkg, "shaders_triangle_rt_pkg")
    }
    fn rp_rt(&self) -> ModulResult<&RenderPassDefaultRtPkg> {
        Self::slot_ref(
            &self.render_pass_triangle_rt_pkg,
            "render_pass_triangle_rt_pkg",
        )
    }
    fn pl_tri(&self) -> ModulResult<&GraphicsPipelineDefaultRtPkg> {
        Self::slot_ref(&self.pipeline_triangle_rt_pkg, "pipeline_triangle_rt_pkg")
    }
    fn shaders_steel(&self) -> ModulResult<&ShaderModulesDefaultRtPkg> {
        Self::slot_ref(&self.shaders_mesh_solid_rt_pkg, "shaders_mesh_solid_rt_pkg")
    }
    fn pl_steel(&self) -> ModulResult<&GraphicsPipelineDefaultRtPkg> {
        Self::slot_ref(&self.pipeline_mesh_solid_rt_pkg, "pipeline_mesh_solid_rt_pkg")
    }
    fn shaders_line(&self) -> ModulResult<&ShaderModulesDefaultRtPkg> {
        Self::slot_ref(&self.shaders_line_rt_pkg, "shaders_line_rt_pkg")
    }
    fn pl_line(&self) -> ModulResult<&GraphicsPipelineDefaultRtPkg> {
        Self::slot_ref(&self.pipeline_line_rt_pkg, "pipeline_line_rt_pkg")
    }
    fn pl_line_tris(&self) -> ModulResult<&GraphicsPipelineDefaultRtPkg> {
        Self::slot_ref(
            &self.pipeline_line_tris_rt_pkg,
            "pipeline_line_tris_rt_pkg",
        )
    }
    fn cargo(&self) -> ModulResult<&RendererDefaultRtCrg> {
        Self::slot_ref(&self.cargo_rt, "cargo_rt")
    }
}
