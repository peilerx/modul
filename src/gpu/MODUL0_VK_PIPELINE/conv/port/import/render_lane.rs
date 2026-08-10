//! Intent in-port — PortMatch `RenderLanePrt` → *StpPkg only (closed gestalt).
//! Assemble execution ∈ `mem/asm_disasm/order/render_lane`.

use ash::vk;

use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::op::RenderPassAttachmentLayoutStpPkgOp;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::render_res_intsct_stp_pkgs::PipelineTriangleStpPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::render_res_intsct_stp_pkgs::RenderPassTriangleStpPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::RenderLanePrt;
/// PortMatch module picture → Internal setup bags.
///
/// `extent_*` + `surface_format_op` = **external** peer/window facts (not module intent).
/// All other levers = **closed** in each arm (full gestalt of that picture).
/// PortMatch → write setup bags · never returns (FIX-128 · v5.1).
pub fn import_render_lane_for_asm(
    render_lane_prt: RenderLanePrt,
    extent_width_stp: u32,
    extent_height_stp: u32,
    surface_format_op: vk::Format,
    render_pass_triangle_stp_pkg: &mut RenderPassTriangleStpPkg,
    pipeline_triangle_stp_pkg: &mut PipelineTriangleStpPkg,
) {
    let (rp, pl) = match render_lane_prt {
        RenderLanePrt::TriangleSolidDepth => (
            RenderPassTriangleStpPkg {
                surface_format_op,
                sample_count_op: vk::SampleCountFlags::TYPE_1,
                attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp::SIMPLE,
                depth_format_op: vk::Format::D32_SFLOAT,
                color_layout_op: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                depth_layout_op: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                present_layout_op: vk::ImageLayout::PRESENT_SRC_KHR,
                initial_layout_op: vk::ImageLayout::UNDEFINED,
                desc: "render_pass_triangle_solid_depth",
            },
            PipelineTriangleStpPkg {
                sample_count_op: vk::SampleCountFlags::TYPE_1,
                topology_op: vk::PrimitiveTopology::TRIANGLE_LIST,
                polygon_mode_op: vk::PolygonMode::FILL,
                cull_mode_op: vk::CullModeFlags::NONE,
                front_face_op: vk::FrontFace::COUNTER_CLOCKWISE,
                depth_compare_op: vk::CompareOp::LESS,
                color_write_mask_op: vk::ColorComponentFlags::RGBA,
                extent_width_stp,
                extent_height_stp,
                desc: "pipeline_triangle_solid_depth",
            },
        ),
        RenderLanePrt::TriangleSolidDepthCullBack => (
            RenderPassTriangleStpPkg {
                surface_format_op,
                sample_count_op: vk::SampleCountFlags::TYPE_1,
                attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp::SIMPLE,
                depth_format_op: vk::Format::D32_SFLOAT,
                color_layout_op: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                depth_layout_op: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                present_layout_op: vk::ImageLayout::PRESENT_SRC_KHR,
                initial_layout_op: vk::ImageLayout::UNDEFINED,
                desc: "render_pass_triangle_solid_depth_cull_back",
            },
            PipelineTriangleStpPkg {
                sample_count_op: vk::SampleCountFlags::TYPE_1,
                topology_op: vk::PrimitiveTopology::TRIANGLE_LIST,
                polygon_mode_op: vk::PolygonMode::FILL,
                cull_mode_op: vk::CullModeFlags::BACK,
                front_face_op: vk::FrontFace::COUNTER_CLOCKWISE,
                depth_compare_op: vk::CompareOp::LESS,
                color_write_mask_op: vk::ColorComponentFlags::RGBA,
                extent_width_stp,
                extent_height_stp,
                desc: "pipeline_triangle_solid_depth_cull_back",
            },
        ),
        RenderLanePrt::TriangleWireDepth => (
            RenderPassTriangleStpPkg {
                surface_format_op,
                sample_count_op: vk::SampleCountFlags::TYPE_1,
                attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp::SIMPLE,
                depth_format_op: vk::Format::D32_SFLOAT,
                color_layout_op: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                depth_layout_op: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                present_layout_op: vk::ImageLayout::PRESENT_SRC_KHR,
                initial_layout_op: vk::ImageLayout::UNDEFINED,
                desc: "render_pass_triangle_wire_depth",
            },
            PipelineTriangleStpPkg {
                sample_count_op: vk::SampleCountFlags::TYPE_1,
                topology_op: vk::PrimitiveTopology::TRIANGLE_LIST,
                polygon_mode_op: vk::PolygonMode::LINE,
                cull_mode_op: vk::CullModeFlags::NONE,
                front_face_op: vk::FrontFace::COUNTER_CLOCKWISE,
                depth_compare_op: vk::CompareOp::LESS,
                color_write_mask_op: vk::ColorComponentFlags::RGBA,
                extent_width_stp,
                extent_height_stp,
                desc: "pipeline_triangle_wire_depth",
            },
        ),
        RenderLanePrt::TriangleSolidDepthAa4 => (
            RenderPassTriangleStpPkg {
                surface_format_op,
                sample_count_op: vk::SampleCountFlags::TYPE_4,
                attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp::MSAA,
                depth_format_op: vk::Format::D32_SFLOAT,
                color_layout_op: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                depth_layout_op: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                present_layout_op: vk::ImageLayout::PRESENT_SRC_KHR,
                initial_layout_op: vk::ImageLayout::UNDEFINED,
                desc: "render_pass_triangle_solid_depth_aa4",
            },
            PipelineTriangleStpPkg {
                sample_count_op: vk::SampleCountFlags::TYPE_4,
                topology_op: vk::PrimitiveTopology::TRIANGLE_LIST,
                polygon_mode_op: vk::PolygonMode::FILL,
                cull_mode_op: vk::CullModeFlags::NONE,
                front_face_op: vk::FrontFace::COUNTER_CLOCKWISE,
                depth_compare_op: vk::CompareOp::LESS,
                color_write_mask_op: vk::ColorComponentFlags::RGBA,
                extent_width_stp,
                extent_height_stp,
                desc: "pipeline_triangle_solid_depth_aa4",
            },
        ),
        RenderLanePrt::TriangleSolidDepthAa8 => (
            RenderPassTriangleStpPkg {
                surface_format_op,
                sample_count_op: vk::SampleCountFlags::TYPE_8,
                attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp::MSAA,
                depth_format_op: vk::Format::D32_SFLOAT,
                color_layout_op: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                depth_layout_op: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                present_layout_op: vk::ImageLayout::PRESENT_SRC_KHR,
                initial_layout_op: vk::ImageLayout::UNDEFINED,
                desc: "render_pass_triangle_solid_depth_aa8",
            },
            PipelineTriangleStpPkg {
                sample_count_op: vk::SampleCountFlags::TYPE_8,
                topology_op: vk::PrimitiveTopology::TRIANGLE_LIST,
                polygon_mode_op: vk::PolygonMode::FILL,
                cull_mode_op: vk::CullModeFlags::NONE,
                front_face_op: vk::FrontFace::COUNTER_CLOCKWISE,
                depth_compare_op: vk::CompareOp::LESS,
                color_write_mask_op: vk::ColorComponentFlags::RGBA,
                extent_width_stp,
                extent_height_stp,
                desc: "pipeline_triangle_solid_depth_aa8",
            },
        ),
        RenderLanePrt::TriangleWireDepthAa4 => (
            RenderPassTriangleStpPkg {
                surface_format_op,
                sample_count_op: vk::SampleCountFlags::TYPE_4,
                attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp::MSAA,
                depth_format_op: vk::Format::D32_SFLOAT,
                color_layout_op: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                depth_layout_op: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                present_layout_op: vk::ImageLayout::PRESENT_SRC_KHR,
                initial_layout_op: vk::ImageLayout::UNDEFINED,
                desc: "render_pass_triangle_wire_depth_aa4",
            },
            PipelineTriangleStpPkg {
                sample_count_op: vk::SampleCountFlags::TYPE_4,
                topology_op: vk::PrimitiveTopology::TRIANGLE_LIST,
                polygon_mode_op: vk::PolygonMode::LINE,
                cull_mode_op: vk::CullModeFlags::NONE,
                front_face_op: vk::FrontFace::COUNTER_CLOCKWISE,
                depth_compare_op: vk::CompareOp::LESS,
                color_write_mask_op: vk::ColorComponentFlags::RGBA,
                extent_width_stp,
                extent_height_stp,
                desc: "pipeline_triangle_wire_depth_aa4",
            },
        ),
        RenderLanePrt::TriangleSolidDepthAlways => (
            RenderPassTriangleStpPkg {
                surface_format_op,
                sample_count_op: vk::SampleCountFlags::TYPE_1,
                attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp::SIMPLE,
                depth_format_op: vk::Format::D32_SFLOAT,
                color_layout_op: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                depth_layout_op: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                present_layout_op: vk::ImageLayout::PRESENT_SRC_KHR,
                initial_layout_op: vk::ImageLayout::UNDEFINED,
                desc: "render_pass_triangle_solid_depth_always",
            },
            PipelineTriangleStpPkg {
                sample_count_op: vk::SampleCountFlags::TYPE_1,
                topology_op: vk::PrimitiveTopology::TRIANGLE_LIST,
                polygon_mode_op: vk::PolygonMode::FILL,
                cull_mode_op: vk::CullModeFlags::NONE,
                front_face_op: vk::FrontFace::COUNTER_CLOCKWISE,
                depth_compare_op: vk::CompareOp::ALWAYS,
                color_write_mask_op: vk::ColorComponentFlags::RGBA,
                extent_width_stp,
                extent_height_stp,
                desc: "pipeline_triangle_solid_depth_always",
            },
        ),
    };
    *render_pass_triangle_stp_pkg = rp;
    *pipeline_triangle_stp_pkg = pl;
}
