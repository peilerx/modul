//! vk_crg — pack RendererDefaultRtCrg (FIX-120).
//! One catalog assemble per trait (FIX-083/129).

use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::{
    GraphicsPipelineDefaultRtPkg, RenderPassDefaultRtPkg, RendererDefaultRtCrg,
    ShaderModulesDefaultRtPkg,
};

/// Catalog — triangle-only renderer cargo.
pub trait RendererDefaultRtCrgAuto {
    fn auto_assemble(
        shaders_triangle_rt_pkg: ShaderModulesDefaultRtPkg,
        render_pass_triangle_rt_pkg: RenderPassDefaultRtPkg,
        pipeline_triangle_rt_pkg: GraphicsPipelineDefaultRtPkg,
    ) -> RendererDefaultRtCrg;
}

/// Catalog — triangle + steel renderer cargo.
pub trait RendererSteelRtCrgAuto {
    fn auto_assemble(
        shaders_triangle_rt_pkg: ShaderModulesDefaultRtPkg,
        render_pass_triangle_rt_pkg: RenderPassDefaultRtPkg,
        pipeline_triangle_rt_pkg: GraphicsPipelineDefaultRtPkg,
        shaders_steel_rt_pkg: ShaderModulesDefaultRtPkg,
        pipeline_steel_rt_pkg: GraphicsPipelineDefaultRtPkg,
    ) -> RendererDefaultRtCrg;
}

/// Catalog — full product renderer cargo (triangle + steel + line).
pub trait RendererProductRtCrgAuto {
    fn auto_assemble(
        shaders_triangle_rt_pkg: ShaderModulesDefaultRtPkg,
        render_pass_triangle_rt_pkg: RenderPassDefaultRtPkg,
        pipeline_triangle_rt_pkg: GraphicsPipelineDefaultRtPkg,
        shaders_steel_rt_pkg: ShaderModulesDefaultRtPkg,
        pipeline_steel_rt_pkg: GraphicsPipelineDefaultRtPkg,
        shaders_line_rt_pkg: ShaderModulesDefaultRtPkg,
        pipeline_line_rt_pkg: GraphicsPipelineDefaultRtPkg,
        pipeline_line_tris_rt_pkg: GraphicsPipelineDefaultRtPkg,
    ) -> RendererDefaultRtCrg;
}

impl RendererDefaultRtCrgAuto for RendererDefaultRtCrg {
    fn auto_assemble(
        shaders_triangle_rt_pkg: ShaderModulesDefaultRtPkg,
        render_pass_triangle_rt_pkg: RenderPassDefaultRtPkg,
        pipeline_triangle_rt_pkg: GraphicsPipelineDefaultRtPkg,
    ) -> RendererDefaultRtCrg {
        RendererDefaultRtCrg {
            shaders_triangle_rt_pkg,
            render_pass_triangle_rt_pkg,
            pipeline_triangle_rt_pkg,
            shaders_steel_rt_pkg: None,
            pipeline_steel_rt_pkg: None,
            shaders_line_rt_pkg: None,
            pipeline_line_rt_pkg: None,
            pipeline_line_tris_rt_pkg: None,
            descriptor_set_layout_default_rt_pkg: None,
            descriptor_pool_default_rt_pkg: None,
            descriptor_sets_default_rt_pkg: None,
            sampler_default_rt_pkg: None,
            pipeline_cache_default_rt_pkg: None,
            desc: "renderer_triangle_lane",
        }
    }
}

impl RendererSteelRtCrgAuto for RendererDefaultRtCrg {
    fn auto_assemble(
        shaders_triangle_rt_pkg: ShaderModulesDefaultRtPkg,
        render_pass_triangle_rt_pkg: RenderPassDefaultRtPkg,
        pipeline_triangle_rt_pkg: GraphicsPipelineDefaultRtPkg,
        shaders_steel_rt_pkg: ShaderModulesDefaultRtPkg,
        pipeline_steel_rt_pkg: GraphicsPipelineDefaultRtPkg,
    ) -> RendererDefaultRtCrg {
        RendererDefaultRtCrg {
            shaders_triangle_rt_pkg,
            render_pass_triangle_rt_pkg,
            pipeline_triangle_rt_pkg,
            shaders_steel_rt_pkg: Some(shaders_steel_rt_pkg),
            pipeline_steel_rt_pkg: Some(pipeline_steel_rt_pkg),
            shaders_line_rt_pkg: None,
            pipeline_line_rt_pkg: None,
            pipeline_line_tris_rt_pkg: None,
            descriptor_set_layout_default_rt_pkg: None,
            descriptor_pool_default_rt_pkg: None,
            descriptor_sets_default_rt_pkg: None,
            sampler_default_rt_pkg: None,
            pipeline_cache_default_rt_pkg: None,
            desc: "renderer_cubes_lane",
        }
    }
}

impl RendererProductRtCrgAuto for RendererDefaultRtCrg {
    fn auto_assemble(
        shaders_triangle_rt_pkg: ShaderModulesDefaultRtPkg,
        render_pass_triangle_rt_pkg: RenderPassDefaultRtPkg,
        pipeline_triangle_rt_pkg: GraphicsPipelineDefaultRtPkg,
        shaders_steel_rt_pkg: ShaderModulesDefaultRtPkg,
        pipeline_steel_rt_pkg: GraphicsPipelineDefaultRtPkg,
        shaders_line_rt_pkg: ShaderModulesDefaultRtPkg,
        pipeline_line_rt_pkg: GraphicsPipelineDefaultRtPkg,
        pipeline_line_tris_rt_pkg: GraphicsPipelineDefaultRtPkg,
    ) -> RendererDefaultRtCrg {
        RendererDefaultRtCrg {
            shaders_triangle_rt_pkg,
            render_pass_triangle_rt_pkg,
            pipeline_triangle_rt_pkg,
            shaders_steel_rt_pkg: Some(shaders_steel_rt_pkg),
            pipeline_steel_rt_pkg: Some(pipeline_steel_rt_pkg),
            shaders_line_rt_pkg: Some(shaders_line_rt_pkg),
            pipeline_line_rt_pkg: Some(pipeline_line_rt_pkg),
            pipeline_line_tris_rt_pkg: Some(pipeline_line_tris_rt_pkg),
            descriptor_set_layout_default_rt_pkg: None,
            descriptor_pool_default_rt_pkg: None,
            descriptor_sets_default_rt_pkg: None,
            sampler_default_rt_pkg: None,
            pipeline_cache_default_rt_pkg: None,
            desc: "renderer_product_steel_line",
        }
    }
}
