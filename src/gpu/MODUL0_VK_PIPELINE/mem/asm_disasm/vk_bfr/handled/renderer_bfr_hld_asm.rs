//! Catalog — Handled *Bfr seed from *Stp knobs (FIX-129 · FIX-131).
//!
//! Handled ≔ ≥1 intention *StpPkg. Empty warehouse without knobs is Auto.

use crate::gpu::MODUL0_VK_PIPELINE::mem::base::embedded::buffer::RendererBfr;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::render_res_intsct_stp_pkgs::PipelineTriangleStpPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::render_res_intsct_stp_pkgs::RenderPassTriangleStpPkg;

/// Handled warehouse seed · *Stp knobs already on Bfr (no Auto empty catalog).
pub trait RendererBfrHandled: Sized {
    fn handled_assemble(
        render_pass_triangle_stp_pkg: RenderPassTriangleStpPkg,
        pipeline_triangle_stp_pkg: PipelineTriangleStpPkg,
    ) -> Self;
}

impl RendererBfrHandled for RendererBfr {
    fn handled_assemble(
        render_pass_triangle_stp_pkg: RenderPassTriangleStpPkg,
        pipeline_triangle_stp_pkg: PipelineTriangleStpPkg,
    ) -> Self {
        Self {
            render_pass_triangle_stp_pkg: Some(render_pass_triangle_stp_pkg),
            pipeline_triangle_stp_pkg: Some(pipeline_triangle_stp_pkg),
            shaders_triangle_rt_pkg: None,
            render_pass_triangle_rt_pkg: None,
            pipeline_triangle_rt_pkg: None,
            shaders_mesh_solid_rt_pkg: None,
            pipeline_mesh_solid_rt_pkg: None,
            shaders_line_rt_pkg: None,
            pipeline_line_rt_pkg: None,
            pipeline_line_tris_rt_pkg: None,
            pipeline_mesh_soa_comp_rt_pkg: None,
            pipeline_soa_heat_comp_rt_pkg: None,
            descriptor_set_layout_default_rt_pkg: None,
            descriptor_pool_default_rt_pkg: None,
            descriptor_sets_default_rt_pkg: None,
            cargo_rt: None,
        }
    }
}
