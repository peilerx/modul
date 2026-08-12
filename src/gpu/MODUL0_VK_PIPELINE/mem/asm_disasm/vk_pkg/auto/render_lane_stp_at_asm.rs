//! Auto-assemble **setup** bags from [`RenderLanePrt`] (PortMatch picture → *Stp).
//!
//! Not Vulkan create — pure closed-gestalt fill of knobs. Runtime *Rt still go through
//! Handled pipelines/pass assemblers that consume these *Stp peels.
//!
//! **Protocol:** `RenderLanePrt` *is* the intent that aggregates op-groups
//! (samples · depth · cull · polygon · MSAA layout). Auto = preset table.
//! Handled = would take an explicit ops bag / different Prt if custom knobs needed;
//! product presets stay on Auto.

use ash::vk;

use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::prt::render_lane_prt::RenderLanePrt;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::op::RenderPassAttachmentLayoutStpPkgOp;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::render_res_intsct_stp_pkgs::{
    PipelineTriangleStpPkg, RenderPassTriangleStpPkg,
};

/// Catalog — render-pass *Stp from lane picture + external surface format.
pub trait RenderPassTriangleStpAuto {
    fn auto_assemble(
        render_lane_prt: RenderLanePrt,
        surface_format_op: vk::Format,
    ) -> Self;
}

/// Catalog — pipeline *Stp from lane picture + external extent.
pub trait PipelineTriangleStpAuto {
    fn auto_assemble(
        render_lane_prt: RenderLanePrt,
        extent_width_stp: u32,
        extent_height_stp: u32,
    ) -> Self;
}

/// Co-create both *Stp from one picture (N.RES_INTSCT · same Prt).
pub trait RenderLaneStpResIntsctAuto {
    fn auto_assemble(
        render_lane_prt: RenderLanePrt,
        extent_width_stp: u32,
        extent_height_stp: u32,
        surface_format_op: vk::Format,
    ) -> (RenderPassTriangleStpPkg, PipelineTriangleStpPkg);
}

impl RenderPassTriangleStpAuto for RenderPassTriangleStpPkg {
    fn auto_assemble(
        render_lane_prt: RenderLanePrt,
        surface_format_op: vk::Format,
    ) -> Self {
        match render_lane_prt {
            RenderLanePrt::TRIANGLE_SOLID_DEPTH => Self {
                surface_format_op,
                sample_count_op: vk::SampleCountFlags::TYPE_1,
                attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp::SIMPLE,
                depth_format_op: vk::Format::D32_SFLOAT,
                color_layout_op: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                depth_layout_op: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                present_layout_op: vk::ImageLayout::PRESENT_SRC_KHR,
                initial_layout_op: vk::ImageLayout::UNDEFINED,
                color_load_op: vk::AttachmentLoadOp::CLEAR,
                color_store_op: vk::AttachmentStoreOp::STORE,
                depth_load_op: vk::AttachmentLoadOp::CLEAR,
                depth_store_op: vk::AttachmentStoreOp::DONT_CARE,
                desc: "render_pass_triangle_solid_depth",
            },
            RenderLanePrt::TRIANGLE_SOLID_DEPTH_CULL_BACK => Self {
                surface_format_op,
                sample_count_op: vk::SampleCountFlags::TYPE_1,
                attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp::SIMPLE,
                depth_format_op: vk::Format::D32_SFLOAT,
                color_layout_op: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                depth_layout_op: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                present_layout_op: vk::ImageLayout::PRESENT_SRC_KHR,
                initial_layout_op: vk::ImageLayout::UNDEFINED,
                color_load_op: vk::AttachmentLoadOp::CLEAR,
                color_store_op: vk::AttachmentStoreOp::STORE,
                depth_load_op: vk::AttachmentLoadOp::CLEAR,
                depth_store_op: vk::AttachmentStoreOp::DONT_CARE,
                desc: "render_pass_triangle_solid_depth_cull_back",
            },
            RenderLanePrt::TRIANGLE_WIRE_DEPTH => Self {
                surface_format_op,
                sample_count_op: vk::SampleCountFlags::TYPE_1,
                attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp::SIMPLE,
                depth_format_op: vk::Format::D32_SFLOAT,
                color_layout_op: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                depth_layout_op: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                present_layout_op: vk::ImageLayout::PRESENT_SRC_KHR,
                initial_layout_op: vk::ImageLayout::UNDEFINED,
                color_load_op: vk::AttachmentLoadOp::CLEAR,
                color_store_op: vk::AttachmentStoreOp::STORE,
                depth_load_op: vk::AttachmentLoadOp::CLEAR,
                depth_store_op: vk::AttachmentStoreOp::DONT_CARE,
                desc: "render_pass_triangle_wire_depth",
            },
            RenderLanePrt::TRIANGLE_SOLID_DEPTH_AA4 => Self {
                surface_format_op,
                sample_count_op: vk::SampleCountFlags::TYPE_4,
                attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp::MSAA,
                depth_format_op: vk::Format::D32_SFLOAT,
                color_layout_op: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                depth_layout_op: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                present_layout_op: vk::ImageLayout::PRESENT_SRC_KHR,
                initial_layout_op: vk::ImageLayout::UNDEFINED,
                color_load_op: vk::AttachmentLoadOp::CLEAR,
                color_store_op: vk::AttachmentStoreOp::STORE,
                depth_load_op: vk::AttachmentLoadOp::CLEAR,
                depth_store_op: vk::AttachmentStoreOp::DONT_CARE,
                desc: "render_pass_triangle_solid_depth_aa4",
            },
            RenderLanePrt::TRIANGLE_SOLID_DEPTH_AA8 => Self {
                surface_format_op,
                sample_count_op: vk::SampleCountFlags::TYPE_8,
                attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp::MSAA,
                depth_format_op: vk::Format::D32_SFLOAT,
                color_layout_op: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                depth_layout_op: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                present_layout_op: vk::ImageLayout::PRESENT_SRC_KHR,
                initial_layout_op: vk::ImageLayout::UNDEFINED,
                color_load_op: vk::AttachmentLoadOp::CLEAR,
                color_store_op: vk::AttachmentStoreOp::STORE,
                depth_load_op: vk::AttachmentLoadOp::CLEAR,
                depth_store_op: vk::AttachmentStoreOp::DONT_CARE,
                desc: "render_pass_triangle_solid_depth_aa8",
            },
            RenderLanePrt::TRIANGLE_WIRE_DEPTH_AA4 => Self {
                surface_format_op,
                sample_count_op: vk::SampleCountFlags::TYPE_4,
                attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp::MSAA,
                depth_format_op: vk::Format::D32_SFLOAT,
                color_layout_op: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                depth_layout_op: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                present_layout_op: vk::ImageLayout::PRESENT_SRC_KHR,
                initial_layout_op: vk::ImageLayout::UNDEFINED,
                color_load_op: vk::AttachmentLoadOp::CLEAR,
                color_store_op: vk::AttachmentStoreOp::STORE,
                depth_load_op: vk::AttachmentLoadOp::CLEAR,
                depth_store_op: vk::AttachmentStoreOp::DONT_CARE,
                desc: "render_pass_triangle_wire_depth_aa4",
            },
            RenderLanePrt::TRIANGLE_SOLID_DEPTH_ALWAYS => Self {
                surface_format_op,
                sample_count_op: vk::SampleCountFlags::TYPE_1,
                attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp::SIMPLE,
                depth_format_op: vk::Format::D32_SFLOAT,
                color_layout_op: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                depth_layout_op: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                present_layout_op: vk::ImageLayout::PRESENT_SRC_KHR,
                initial_layout_op: vk::ImageLayout::UNDEFINED,
                color_load_op: vk::AttachmentLoadOp::CLEAR,
                color_store_op: vk::AttachmentStoreOp::STORE,
                depth_load_op: vk::AttachmentLoadOp::CLEAR,
                depth_store_op: vk::AttachmentStoreOp::DONT_CARE,
                desc: "render_pass_triangle_solid_depth_always",
            },
        }
    }
}

impl PipelineTriangleStpAuto for PipelineTriangleStpPkg {
    fn auto_assemble(
        render_lane_prt: RenderLanePrt,
        extent_width_stp: u32,
        extent_height_stp: u32,
    ) -> Self {
        match render_lane_prt {
            RenderLanePrt::TRIANGLE_SOLID_DEPTH => Self {
                sample_count_op: vk::SampleCountFlags::TYPE_1,
                topology_op: vk::PrimitiveTopology::TRIANGLE_LIST,
                polygon_mode_op: vk::PolygonMode::FILL,
                cull_mode_op: vk::CullModeFlags::NONE,
                front_face_op: vk::FrontFace::COUNTER_CLOCKWISE,
                depth_test_enable_stp: true,
                depth_write_enable_stp: true,
                depth_compare_op: vk::CompareOp::LESS,
                color_write_mask_op: vk::ColorComponentFlags::RGBA,
                blend_enable_stp: false,
                src_color_blend_factor_op: vk::BlendFactor::ONE,
                dst_color_blend_factor_op: vk::BlendFactor::ZERO,
                color_blend_op: vk::BlendOp::ADD,
                primitive_restart_enable_stp: false,
                line_width_stp: 1.0,
                extent_width_stp,
                extent_height_stp,
                desc: "pipeline_triangle_solid_depth",
            },
            RenderLanePrt::TRIANGLE_SOLID_DEPTH_CULL_BACK => Self {
                sample_count_op: vk::SampleCountFlags::TYPE_1,
                topology_op: vk::PrimitiveTopology::TRIANGLE_LIST,
                polygon_mode_op: vk::PolygonMode::FILL,
                cull_mode_op: vk::CullModeFlags::BACK,
                front_face_op: vk::FrontFace::COUNTER_CLOCKWISE,
                depth_test_enable_stp: true,
                depth_write_enable_stp: true,
                depth_compare_op: vk::CompareOp::LESS,
                color_write_mask_op: vk::ColorComponentFlags::RGBA,
                blend_enable_stp: false,
                src_color_blend_factor_op: vk::BlendFactor::ONE,
                dst_color_blend_factor_op: vk::BlendFactor::ZERO,
                color_blend_op: vk::BlendOp::ADD,
                primitive_restart_enable_stp: false,
                line_width_stp: 1.0,
                extent_width_stp,
                extent_height_stp,
                desc: "pipeline_triangle_solid_depth_cull_back",
            },
            RenderLanePrt::TRIANGLE_WIRE_DEPTH => Self {
                sample_count_op: vk::SampleCountFlags::TYPE_1,
                topology_op: vk::PrimitiveTopology::TRIANGLE_LIST,
                polygon_mode_op: vk::PolygonMode::LINE,
                cull_mode_op: vk::CullModeFlags::NONE,
                front_face_op: vk::FrontFace::COUNTER_CLOCKWISE,
                depth_test_enable_stp: true,
                depth_write_enable_stp: true,
                depth_compare_op: vk::CompareOp::LESS,
                color_write_mask_op: vk::ColorComponentFlags::RGBA,
                blend_enable_stp: false,
                src_color_blend_factor_op: vk::BlendFactor::ONE,
                dst_color_blend_factor_op: vk::BlendFactor::ZERO,
                color_blend_op: vk::BlendOp::ADD,
                primitive_restart_enable_stp: false,
                line_width_stp: 1.0,
                extent_width_stp,
                extent_height_stp,
                desc: "pipeline_triangle_wire_depth",
            },
            RenderLanePrt::TRIANGLE_SOLID_DEPTH_AA4 => Self {
                sample_count_op: vk::SampleCountFlags::TYPE_4,
                topology_op: vk::PrimitiveTopology::TRIANGLE_LIST,
                polygon_mode_op: vk::PolygonMode::FILL,
                cull_mode_op: vk::CullModeFlags::NONE,
                front_face_op: vk::FrontFace::COUNTER_CLOCKWISE,
                depth_test_enable_stp: true,
                depth_write_enable_stp: true,
                depth_compare_op: vk::CompareOp::LESS,
                color_write_mask_op: vk::ColorComponentFlags::RGBA,
                blend_enable_stp: false,
                src_color_blend_factor_op: vk::BlendFactor::ONE,
                dst_color_blend_factor_op: vk::BlendFactor::ZERO,
                color_blend_op: vk::BlendOp::ADD,
                primitive_restart_enable_stp: false,
                line_width_stp: 1.0,
                extent_width_stp,
                extent_height_stp,
                desc: "pipeline_triangle_solid_depth_aa4",
            },
            RenderLanePrt::TRIANGLE_SOLID_DEPTH_AA8 => Self {
                sample_count_op: vk::SampleCountFlags::TYPE_8,
                topology_op: vk::PrimitiveTopology::TRIANGLE_LIST,
                polygon_mode_op: vk::PolygonMode::FILL,
                cull_mode_op: vk::CullModeFlags::NONE,
                front_face_op: vk::FrontFace::COUNTER_CLOCKWISE,
                depth_test_enable_stp: true,
                depth_write_enable_stp: true,
                depth_compare_op: vk::CompareOp::LESS,
                color_write_mask_op: vk::ColorComponentFlags::RGBA,
                blend_enable_stp: false,
                src_color_blend_factor_op: vk::BlendFactor::ONE,
                dst_color_blend_factor_op: vk::BlendFactor::ZERO,
                color_blend_op: vk::BlendOp::ADD,
                primitive_restart_enable_stp: false,
                line_width_stp: 1.0,
                extent_width_stp,
                extent_height_stp,
                desc: "pipeline_triangle_solid_depth_aa8",
            },
            RenderLanePrt::TRIANGLE_WIRE_DEPTH_AA4 => Self {
                sample_count_op: vk::SampleCountFlags::TYPE_4,
                topology_op: vk::PrimitiveTopology::TRIANGLE_LIST,
                polygon_mode_op: vk::PolygonMode::LINE,
                cull_mode_op: vk::CullModeFlags::NONE,
                front_face_op: vk::FrontFace::COUNTER_CLOCKWISE,
                depth_test_enable_stp: true,
                depth_write_enable_stp: true,
                depth_compare_op: vk::CompareOp::LESS,
                color_write_mask_op: vk::ColorComponentFlags::RGBA,
                blend_enable_stp: false,
                src_color_blend_factor_op: vk::BlendFactor::ONE,
                dst_color_blend_factor_op: vk::BlendFactor::ZERO,
                color_blend_op: vk::BlendOp::ADD,
                primitive_restart_enable_stp: false,
                line_width_stp: 1.0,
                extent_width_stp,
                extent_height_stp,
                desc: "pipeline_triangle_wire_depth_aa4",
            },
            RenderLanePrt::TRIANGLE_SOLID_DEPTH_ALWAYS => Self {
                sample_count_op: vk::SampleCountFlags::TYPE_1,
                topology_op: vk::PrimitiveTopology::TRIANGLE_LIST,
                polygon_mode_op: vk::PolygonMode::FILL,
                cull_mode_op: vk::CullModeFlags::NONE,
                front_face_op: vk::FrontFace::COUNTER_CLOCKWISE,
                depth_test_enable_stp: true,
                depth_write_enable_stp: true,
                depth_compare_op: vk::CompareOp::ALWAYS,
                color_write_mask_op: vk::ColorComponentFlags::RGBA,
                blend_enable_stp: false,
                src_color_blend_factor_op: vk::BlendFactor::ONE,
                dst_color_blend_factor_op: vk::BlendFactor::ZERO,
                color_blend_op: vk::BlendOp::ADD,
                primitive_restart_enable_stp: false,
                line_width_stp: 1.0,
                extent_width_stp,
                extent_height_stp,
                desc: "pipeline_triangle_solid_depth_always",
            },
        }
    }
}

/// Marker for co-assembly of both *Stp from one Prt.
pub struct RenderLaneStpResIntsct;

impl RenderLaneStpResIntsctAuto for RenderLaneStpResIntsct {
    fn auto_assemble(
        render_lane_prt: RenderLanePrt,
        extent_width_stp: u32,
        extent_height_stp: u32,
        surface_format_op: vk::Format,
    ) -> (RenderPassTriangleStpPkg, PipelineTriangleStpPkg) {
        (
            RenderPassTriangleStpPkg::auto_assemble(render_lane_prt, surface_format_op),
            PipelineTriangleStpPkg::auto_assemble(
                render_lane_prt,
                extent_width_stp,
                extent_height_stp,
            ),
        )
    }
}


/// Ship/default solid lane for a sample-count pick (4× → Aa4, else 1× solid depth).
#[must_use]
pub fn render_lane_prt_for_sample_count(samples: vk::SampleCountFlags) -> RenderLanePrt {
    if samples == vk::SampleCountFlags::TYPE_8 {
        RenderLanePrt::TRIANGLE_SOLID_DEPTH_AA8
    } else if samples == vk::SampleCountFlags::TYPE_4 {
        RenderLanePrt::TRIANGLE_SOLID_DEPTH_AA4
    } else {
        RenderLanePrt::TRIANGLE_SOLID_DEPTH
    }
}
