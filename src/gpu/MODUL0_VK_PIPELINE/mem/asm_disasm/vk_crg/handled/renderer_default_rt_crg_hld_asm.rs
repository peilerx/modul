//! vk_crg **handled** — pack renderer cargo from bfr slots (takes inside).

use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_bfr::auto::renderer_bfr_at_asm::RendererBfrAuto;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::embedded::buffer::RendererBfr;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::RendererDefaultRtCrg;
use crate::ModulResult;

/// `RendererDefaultRtCrgHandled` — trait (renderer default rt crg handled).
/// Handled-assemble catalog trait: construction from imported knobs / peels (FIX-129 Handled rank).
/// Belongs to: render-pass / graphics pipeline MCG.
/// Module path context: `gpu/MODUL0_VK_PIPELINE/mem/asm_disasm/vk_crg/handled`.
pub trait RendererDefaultRtCrgHandled {
    fn handled_assemble(bfr: &mut RendererBfr) -> ModulResult<RendererDefaultRtCrg>;
}

impl RendererDefaultRtCrgHandled for RendererDefaultRtCrg {
    fn handled_assemble(bfr: &mut RendererBfr) -> ModulResult<RendererDefaultRtCrg> {
        Ok(RendererDefaultRtCrg {
            shaders_triangle_rt_pkg: <RendererBfr as RendererBfrAuto>::slot_take(
                &mut bfr.shaders_triangle_rt_pkg,
                "shaders_triangle_rt_pkg",
            )?,
            render_pass_triangle_rt_pkg: <RendererBfr as RendererBfrAuto>::slot_take(
                &mut bfr.render_pass_triangle_rt_pkg,
                "render_pass_triangle_rt_pkg",
            )?,
            pipeline_triangle_rt_pkg: <RendererBfr as RendererBfrAuto>::slot_take(
                &mut bfr.pipeline_triangle_rt_pkg,
                "pipeline_triangle_rt_pkg",
            )?,
            shaders_steel_rt_pkg: bfr.shaders_steel_rt_pkg.take(),
            pipeline_steel_rt_pkg: bfr.pipeline_steel_rt_pkg.take(),
            shaders_line_rt_pkg: bfr.shaders_line_rt_pkg.take(),
            pipeline_line_rt_pkg: bfr.pipeline_line_rt_pkg.take(),
            pipeline_line_tris_rt_pkg: bfr.pipeline_line_tris_rt_pkg.take(),
            descriptor_set_layout_default_rt_pkg: None,
            descriptor_pool_default_rt_pkg: None,
            descriptor_sets_default_rt_pkg: None,
            sampler_default_rt_pkg: None,
            pipeline_cache_default_rt_pkg: None,
            desc: "renderer_product_steel_line",
        })
    }
}
