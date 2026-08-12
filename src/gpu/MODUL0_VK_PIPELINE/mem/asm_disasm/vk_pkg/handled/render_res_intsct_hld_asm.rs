//! MCU **`render_res_intsct`** — shaders · `render_pass` · pipeline (`N.RES_INTSCT` · FIX-120).
//! Co-created triangle render resources · one file · local chains.

use ash::vk;
use ash::Device;

use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::{
    PipelineTriangleRtPkg, RenderPassTriangleRtPkg, ShadersTriangleRtPkg,
};
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::op::RenderPassAttachmentLayoutStpPkgOp;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk::auto::graphics_pipeline_at_asm::GraphicsPipelineAuto;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk::auto::pipeline_layout_res_intsct_at_asm::{
    PipelineLayoutEmptyAuto, PipelineLayoutAuto,
};
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk::auto::render_pass_at_asm::RenderPassAuto;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk::auto::shader_spv_at_asm::ShaderSpvAuto;
use crate::gpu::MODUL0_VK_MESH::proc::processor::line_gpu_rt_at_asm::LINE_PUSH_RT_SIZE;
use crate::gpu::MODUL0_VK_MESH::proc::processor::mesh_push_rt_at_asm::MESH_PUSH_RT_SIZE;
use crate::ModulResult;

// ========== SHADERS ==========

/// Catalog — triangle SPIR-V preset baked in generator (FIX-090/097).
pub trait ShadersTriangleAuto {
    fn auto_assemble(device_extrl: &Device) -> ModulResult<ShadersTriangleRtPkg>;
}

trait ShadersTriangleSpirvAuto {
    fn auto_assemble() -> (&'static [u8], &'static [u8]);
}

trait ShadersTriangleModulesAuto {
    fn auto_assemble(
        device_extrl: &Device,
        vert_code_extrl: &'static [u8],
        frag_code_extrl: &'static [u8],
    ) -> ModulResult<Vec<ash::vk::ShaderModule>>;
}

impl ShadersTriangleSpirvAuto for ShadersTriangleRtPkg {
    fn auto_assemble() -> (&'static [u8], &'static [u8]) {
        // Hard-coded NDC triangle · no vertex attrs · no push constants
        // (must match PipelineTriangleHandled empty input + empty layout).
        (
            include_bytes!("../../../../../../../shader/triangle.vert.spv").as_slice(),
            include_bytes!("../../../../../../../shader/triangle.frag.spv").as_slice(),
        )
    }
}

impl ShadersTriangleModulesAuto for ShadersTriangleRtPkg {
    fn auto_assemble(
        device_extrl: &Device,
        vert_code_extrl: &'static [u8],
        frag_code_extrl: &'static [u8],
    ) -> ModulResult<Vec<ash::vk::ShaderModule>> {
        let vert_spv_extrl = vk::ShaderModule::auto_assemble(device_extrl, vert_code_extrl)?;
        let frag_spv_extrl = vk::ShaderModule::auto_assemble(device_extrl, frag_code_extrl)?;
        Ok(vec![vert_spv_extrl, frag_spv_extrl])
    }
}

impl ShadersTriangleAuto for ShadersTriangleRtPkg {
    fn auto_assemble(device_extrl: &Device) -> ModulResult<ShadersTriangleRtPkg> {
        let (vert_code_extrl, frag_code_extrl) =
            <Self as ShadersTriangleSpirvAuto>::auto_assemble();
        let shader_modules_extrl =
            <Self as ShadersTriangleModulesAuto>::auto_assemble(
                device_extrl,
                vert_code_extrl,
                frag_code_extrl,
            )?;
        Ok(ShadersTriangleRtPkg {
            shader_modules_extrl,
            desc: "shaders_triangle",
        })
    }
}
// ========== RENDER_PASS ==========

/// Catalog — conv unpacks `RenderPassTriangleStpPkg` → `_op` / `_stp` knobs (FIX-090/091/093/094/097).
/// Orchestrator only: local `handled_assemble` chain (FIX-087/088, FIX-095).
pub trait RenderPassTriangleHandled {
    fn handled_assemble(
        device_extrl: &Device,
        surface_format_op: vk::Format,
        sample_count_op: vk::SampleCountFlags,
        attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp,
        depth_format_op: vk::Format,
        color_layout_op: vk::ImageLayout,
        depth_layout_op: vk::ImageLayout,
        present_layout_op: vk::ImageLayout,
        initial_layout_op: vk::ImageLayout,
        color_load_op: vk::AttachmentLoadOp,
        color_store_op: vk::AttachmentStoreOp,
        depth_load_op: vk::AttachmentLoadOp,
        depth_store_op: vk::AttachmentStoreOp,
    ) -> ModulResult<RenderPassTriangleRtPkg>;
}

trait RenderPassTriangleDepthAttachmentHandled {
    fn handled_assemble(
        sample_count_op: vk::SampleCountFlags,
        depth_format_op: vk::Format,
        depth_layout_op: vk::ImageLayout,
        initial_layout_op: vk::ImageLayout,
        depth_load_op: vk::AttachmentLoadOp,
        depth_store_op: vk::AttachmentStoreOp,
    ) -> vk::AttachmentDescription;
}

trait RenderPassTriangleColorAttachmentSimpleHandled {
    fn handled_assemble(
        surface_format_op: vk::Format,
        present_layout_op: vk::ImageLayout,
        initial_layout_op: vk::ImageLayout,
        color_load_op: vk::AttachmentLoadOp,
        color_store_op: vk::AttachmentStoreOp,
    ) -> vk::AttachmentDescription;
}

trait RenderPassTriangleColorAttachmentMsaaHandled {
    fn handled_assemble(
        surface_format_op: vk::Format,
        sample_count_op: vk::SampleCountFlags,
        color_layout_op: vk::ImageLayout,
        initial_layout_op: vk::ImageLayout,
        color_load_op: vk::AttachmentLoadOp,
        color_store_op: vk::AttachmentStoreOp,
    ) -> vk::AttachmentDescription;
}

trait RenderPassTriangleResolveAttachmentHandled {
    fn handled_assemble(
        surface_format_op: vk::Format,
        present_layout_op: vk::ImageLayout,
        initial_layout_op: vk::ImageLayout,
        color_store_op: vk::AttachmentStoreOp,
    ) -> vk::AttachmentDescription;
}

trait RenderPassTriangleMsaaPathHandled {
    fn handled_assemble(
        surface_format_op: vk::Format,
        sample_count_op: vk::SampleCountFlags,
        depth_format_op: vk::Format,
        color_layout_op: vk::ImageLayout,
        depth_layout_op: vk::ImageLayout,
        present_layout_op: vk::ImageLayout,
        initial_layout_op: vk::ImageLayout,
        attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp,
        color_load_op: vk::AttachmentLoadOp,
        color_store_op: vk::AttachmentStoreOp,
        depth_load_op: vk::AttachmentLoadOp,
        depth_store_op: vk::AttachmentStoreOp,
    ) -> (Vec<vk::AttachmentDescription>, &'static str);
}

trait RenderPassTriangleSimplePathHandled {
    fn handled_assemble(
        surface_format_op: vk::Format,
        depth_format_op: vk::Format,
        depth_layout_op: vk::ImageLayout,
        present_layout_op: vk::ImageLayout,
        initial_layout_op: vk::ImageLayout,
        attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp,
        color_load_op: vk::AttachmentLoadOp,
        color_store_op: vk::AttachmentStoreOp,
        depth_load_op: vk::AttachmentLoadOp,
        depth_store_op: vk::AttachmentStoreOp,
    ) -> (Vec<vk::AttachmentDescription>, &'static str);
}

impl RenderPassTriangleDepthAttachmentHandled for RenderPassTriangleRtPkg {
    fn handled_assemble(
        sample_count_op: vk::SampleCountFlags,
        depth_format_op: vk::Format,
        depth_layout_op: vk::ImageLayout,
        initial_layout_op: vk::ImageLayout,
        depth_load_op: vk::AttachmentLoadOp,
        depth_store_op: vk::AttachmentStoreOp,
    ) -> vk::AttachmentDescription {
        vk::AttachmentDescription::default()
            .format(depth_format_op)
            .samples(sample_count_op)
            .load_op(depth_load_op)
            .store_op(depth_store_op)
            .initial_layout(initial_layout_op)
            .final_layout(depth_layout_op)
    }
}

impl RenderPassTriangleColorAttachmentSimpleHandled for RenderPassTriangleRtPkg {
    fn handled_assemble(
        surface_format_op: vk::Format,
        present_layout_op: vk::ImageLayout,
        initial_layout_op: vk::ImageLayout,
        color_load_op: vk::AttachmentLoadOp,
        color_store_op: vk::AttachmentStoreOp,
    ) -> vk::AttachmentDescription {
        vk::AttachmentDescription::default()
            .format(surface_format_op)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(color_load_op)
            .store_op(color_store_op)
            .initial_layout(initial_layout_op)
            .final_layout(present_layout_op)
    }
}

impl RenderPassTriangleColorAttachmentMsaaHandled for RenderPassTriangleRtPkg {
    fn handled_assemble(
        surface_format_op: vk::Format,
        sample_count_op: vk::SampleCountFlags,
        color_layout_op: vk::ImageLayout,
        initial_layout_op: vk::ImageLayout,
        color_load_op: vk::AttachmentLoadOp,
        color_store_op: vk::AttachmentStoreOp,
    ) -> vk::AttachmentDescription {
        // MSAA color is transient: store often DONT_CARE; knobs still from *Stp.
        let _ = color_store_op;
        vk::AttachmentDescription::default()
            .format(surface_format_op)
            .samples(sample_count_op)
            .load_op(color_load_op)
            .store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(initial_layout_op)
            .final_layout(color_layout_op)
    }
}

impl RenderPassTriangleResolveAttachmentHandled for RenderPassTriangleRtPkg {
    fn handled_assemble(
        surface_format_op: vk::Format,
        present_layout_op: vk::ImageLayout,
        initial_layout_op: vk::ImageLayout,
        color_store_op: vk::AttachmentStoreOp,
    ) -> vk::AttachmentDescription {
        vk::AttachmentDescription::default()
            .format(surface_format_op)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::DONT_CARE)
            .store_op(color_store_op)
            .initial_layout(initial_layout_op)
            .final_layout(present_layout_op)
    }
}

impl RenderPassTriangleMsaaPathHandled for RenderPassTriangleRtPkg {
    fn handled_assemble(
        surface_format_op: vk::Format,
        sample_count_op: vk::SampleCountFlags,
        depth_format_op: vk::Format,
        color_layout_op: vk::ImageLayout,
        depth_layout_op: vk::ImageLayout,
        present_layout_op: vk::ImageLayout,
        initial_layout_op: vk::ImageLayout,
        attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp,
        color_load_op: vk::AttachmentLoadOp,
        color_store_op: vk::AttachmentStoreOp,
        depth_load_op: vk::AttachmentLoadOp,
        depth_store_op: vk::AttachmentStoreOp,
    ) -> (Vec<vk::AttachmentDescription>, &'static str) {
        let attachments = vec![
            <Self as RenderPassTriangleColorAttachmentMsaaHandled>::handled_assemble(
                surface_format_op,
                sample_count_op,
                color_layout_op,
                initial_layout_op,
                color_load_op,
                color_store_op,
            ),
            <Self as RenderPassTriangleDepthAttachmentHandled>::handled_assemble(
                sample_count_op,
                depth_format_op,
                depth_layout_op,
                initial_layout_op,
                depth_load_op,
                depth_store_op,
            ),
            <Self as RenderPassTriangleResolveAttachmentHandled>::handled_assemble(
                surface_format_op,
                present_layout_op,
                initial_layout_op,
                color_store_op,
            ),
        ];
        let desc = match attachment_layout_op {
            RenderPassAttachmentLayoutStpPkgOp::MSAA => "render_pass_triangle_msaa",
            RenderPassAttachmentLayoutStpPkgOp::SIMPLE => "render_pass_triangle",
        };
        (attachments, desc)
    }
}

impl RenderPassTriangleSimplePathHandled for RenderPassTriangleRtPkg {
    fn handled_assemble(
        surface_format_op: vk::Format,
        depth_format_op: vk::Format,
        depth_layout_op: vk::ImageLayout,
        present_layout_op: vk::ImageLayout,
        initial_layout_op: vk::ImageLayout,
        attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp,
        color_load_op: vk::AttachmentLoadOp,
        color_store_op: vk::AttachmentStoreOp,
        depth_load_op: vk::AttachmentLoadOp,
        depth_store_op: vk::AttachmentStoreOp,
    ) -> (Vec<vk::AttachmentDescription>, &'static str) {
        let attachments = vec![
            <Self as RenderPassTriangleColorAttachmentSimpleHandled>::handled_assemble(
                surface_format_op,
                present_layout_op,
                initial_layout_op,
                color_load_op,
                color_store_op,
            ),
            <Self as RenderPassTriangleDepthAttachmentHandled>::handled_assemble(
                vk::SampleCountFlags::TYPE_1,
                depth_format_op,
                depth_layout_op,
                initial_layout_op,
                depth_load_op,
                depth_store_op,
            ),
        ];
        let desc = match attachment_layout_op {
            RenderPassAttachmentLayoutStpPkgOp::MSAA => "render_pass_triangle_msaa",
            RenderPassAttachmentLayoutStpPkgOp::SIMPLE => "render_pass_triangle",
        };
        (attachments, desc)
    }
}

impl RenderPassTriangleHandled for RenderPassTriangleRtPkg {
    fn handled_assemble(
        device_extrl: &Device,
        surface_format_op: vk::Format,
        sample_count_op: vk::SampleCountFlags,
        attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp,
        depth_format_op: vk::Format,
        color_layout_op: vk::ImageLayout,
        depth_layout_op: vk::ImageLayout,
        present_layout_op: vk::ImageLayout,
        initial_layout_op: vk::ImageLayout,
        color_load_op: vk::AttachmentLoadOp,
        color_store_op: vk::AttachmentStoreOp,
        depth_load_op: vk::AttachmentLoadOp,
        depth_store_op: vk::AttachmentStoreOp,
    ) -> ModulResult<RenderPassTriangleRtPkg> {
        let (attachments, desc) = match attachment_layout_op {
            RenderPassAttachmentLayoutStpPkgOp::MSAA => {
                <Self as RenderPassTriangleMsaaPathHandled>::handled_assemble(
                    surface_format_op,
                    sample_count_op,
                    depth_format_op,
                    color_layout_op,
                    depth_layout_op,
                    present_layout_op,
                    initial_layout_op,
                    attachment_layout_op,
                    color_load_op,
                    color_store_op,
                    depth_load_op,
                    depth_store_op,
                )
            }
            RenderPassAttachmentLayoutStpPkgOp::SIMPLE => {
                <Self as RenderPassTriangleSimplePathHandled>::handled_assemble(
                    surface_format_op,
                    depth_format_op,
                    depth_layout_op,
                    present_layout_op,
                    initial_layout_op,
                    attachment_layout_op,
                    color_load_op,
                    color_store_op,
                    depth_load_op,
                    depth_store_op,
                )
            }
        };
        let render_pass_extrl = handled_render_pass_triangle_build(
            device_extrl,
            attachment_layout_op,
            color_layout_op,
            depth_layout_op,
            &attachments,
        )?;
        Ok(RenderPassTriangleRtPkg {
            render_pass_extrl,
            desc,
        })
    }
}

fn handled_render_pass_triangle_build(
    device_extrl: &Device,
    attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp,
    color_layout_op: vk::ImageLayout,
    depth_layout_op: vk::ImageLayout,
    attachments: &[vk::AttachmentDescription],
) -> ModulResult<vk::RenderPass> {
    match attachment_layout_op {
        RenderPassAttachmentLayoutStpPkgOp::MSAA => {
            let color_ref = vk::AttachmentReference::default()
                .attachment(0)
                .layout(color_layout_op);
            let depth_ref = vk::AttachmentReference::default()
                .attachment(1)
                .layout(depth_layout_op);
            let resolve_ref = vk::AttachmentReference::default()
                .attachment(2)
                .layout(color_layout_op);
            let subpass = vk::SubpassDescription::default()
                .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                .color_attachments(std::slice::from_ref(&color_ref))
                .depth_stencil_attachment(&depth_ref)
                .resolve_attachments(std::slice::from_ref(&resolve_ref));
            vk::RenderPass::auto_assemble(
                device_extrl,
                attachments,
                std::slice::from_ref(&subpass),
                &[],
            )
        }
        RenderPassAttachmentLayoutStpPkgOp::SIMPLE => {
            let color_ref = vk::AttachmentReference::default()
                .attachment(0)
                .layout(color_layout_op);
            let depth_ref = vk::AttachmentReference::default()
                .attachment(1)
                .layout(depth_layout_op);
            let subpass = vk::SubpassDescription::default()
                .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                .color_attachments(std::slice::from_ref(&color_ref))
                .depth_stencil_attachment(&depth_ref);
            vk::RenderPass::auto_assemble(
                device_extrl,
                attachments,
                std::slice::from_ref(&subpass),
                &[],
            )
        }
    }
}
// ========== PIPELINE ==========

/// Catalog — conv unpacks `PipelineTriangleStpPkg` → `_op` / `_stp` knobs (FIX-090/091/094/095/096/097).
pub trait PipelineTriangleHandled {
    /// Full *Stp knobs (P0 depth/blend/primitive) · no silent create-info defaults.
    fn handled_assemble(
        device_extrl: &Device,
        pl_stp: crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::PipelineTriangleStpPkg,
        render_pass_extrl: vk::RenderPass,
        vert_module_extrl: vk::ShaderModule,
        frag_module_extrl: vk::ShaderModule,
    ) -> ModulResult<PipelineTriangleRtPkg>;
}

trait PipelineTriangleInputAssemblyHandled {
    fn handled_assemble(
        topology_op: vk::PrimitiveTopology,
        primitive_restart_enable_stp: bool,
    ) -> vk::PipelineInputAssemblyStateCreateInfo<'static>;
}

trait PipelineTriangleRasterStateHandled {
    fn handled_assemble(
        polygon_mode_op: vk::PolygonMode,
        cull_mode_op: vk::CullModeFlags,
        front_face_op: vk::FrontFace,
        line_width_stp: f32,
    ) -> vk::PipelineRasterizationStateCreateInfo<'static>;
}

trait PipelineTriangleMultisampleStateHandled {
    fn handled_assemble(
        sample_count_op: vk::SampleCountFlags,
    ) -> vk::PipelineMultisampleStateCreateInfo<'static>;
}

trait PipelineTriangleColorBlendAttachmentHandled {
    fn handled_assemble(
        color_write_mask_op: vk::ColorComponentFlags,
        blend_enable_stp: bool,
        src_color_blend_factor_op: vk::BlendFactor,
        dst_color_blend_factor_op: vk::BlendFactor,
        color_blend_op: vk::BlendOp,
    ) -> vk::PipelineColorBlendAttachmentState;
}

trait PipelineTriangleDepthStencilStateHandled {
    fn handled_assemble(
        depth_test_enable_stp: bool,
        depth_write_enable_stp: bool,
        depth_compare_op: vk::CompareOp,
    ) -> vk::PipelineDepthStencilStateCreateInfo<'static>;
}

trait PipelineTriangleShaderStagesAuto {
    fn auto_assemble(
        vert_module_extrl: vk::ShaderModule,
        frag_module_extrl: vk::ShaderModule,
    ) -> [vk::PipelineShaderStageCreateInfo<'static>; 2];
}

trait PipelineTriangleLayoutAuto {
    fn auto_assemble(device_extrl: &Device) -> ModulResult<vk::PipelineLayout>;
}

impl PipelineTriangleInputAssemblyHandled for PipelineTriangleRtPkg {
    fn handled_assemble(
        topology_op: vk::PrimitiveTopology,
        primitive_restart_enable_stp: bool,
    ) -> vk::PipelineInputAssemblyStateCreateInfo<'static> {
        vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(topology_op)
            .primitive_restart_enable(primitive_restart_enable_stp)
    }
}

impl PipelineTriangleRasterStateHandled for PipelineTriangleRtPkg {
    fn handled_assemble(
        polygon_mode_op: vk::PolygonMode,
        cull_mode_op: vk::CullModeFlags,
        front_face_op: vk::FrontFace,
        line_width_stp: f32,
    ) -> vk::PipelineRasterizationStateCreateInfo<'static> {
        vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(polygon_mode_op)
            .cull_mode(cull_mode_op)
            .front_face(front_face_op)
            .line_width(line_width_stp)
    }
}

impl PipelineTriangleMultisampleStateHandled for PipelineTriangleRtPkg {
    fn handled_assemble(
        sample_count_op: vk::SampleCountFlags,
    ) -> vk::PipelineMultisampleStateCreateInfo<'static> {
        vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(sample_count_op)
    }
}

impl PipelineTriangleColorBlendAttachmentHandled for PipelineTriangleRtPkg {
    fn handled_assemble(
        color_write_mask_op: vk::ColorComponentFlags,
        blend_enable_stp: bool,
        src_color_blend_factor_op: vk::BlendFactor,
        dst_color_blend_factor_op: vk::BlendFactor,
        color_blend_op: vk::BlendOp,
    ) -> vk::PipelineColorBlendAttachmentState {
        vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(color_write_mask_op)
            .blend_enable(blend_enable_stp)
            .src_color_blend_factor(src_color_blend_factor_op)
            .dst_color_blend_factor(dst_color_blend_factor_op)
            .color_blend_op(color_blend_op)
            .src_alpha_blend_factor(src_color_blend_factor_op)
            .dst_alpha_blend_factor(dst_color_blend_factor_op)
            .alpha_blend_op(color_blend_op)
    }
}

impl PipelineTriangleDepthStencilStateHandled for PipelineTriangleRtPkg {
    fn handled_assemble(
        depth_test_enable_stp: bool,
        depth_write_enable_stp: bool,
        depth_compare_op: vk::CompareOp,
    ) -> vk::PipelineDepthStencilStateCreateInfo<'static> {
        vk::PipelineDepthStencilStateCreateInfo::default()
            .depth_test_enable(depth_test_enable_stp)
            .depth_write_enable(depth_write_enable_stp)
            .depth_compare_op(depth_compare_op)
    }
}

impl PipelineTriangleShaderStagesAuto for PipelineTriangleRtPkg {
    fn auto_assemble(
        vert_module_extrl: vk::ShaderModule,
        frag_module_extrl: vk::ShaderModule,
    ) -> [vk::PipelineShaderStageCreateInfo<'static>; 2] {
        let main_name = c"main";
        [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(vert_module_extrl)
                .name(main_name),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(frag_module_extrl)
                .name(main_name),
        ]
    }
}

impl PipelineTriangleLayoutAuto for PipelineTriangleRtPkg {
    fn auto_assemble(device_extrl: &Device) -> ModulResult<vk::PipelineLayout> {
        <vk::PipelineLayout as PipelineLayoutEmptyAuto>::auto_assemble(device_extrl)
    }
}

impl PipelineTriangleHandled for PipelineTriangleRtPkg {
    fn handled_assemble(
        device_extrl: &Device,
        pl_stp: crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::PipelineTriangleStpPkg,
        render_pass_extrl: vk::RenderPass,
        vert_module_extrl: vk::ShaderModule,
        frag_module_extrl: vk::ShaderModule,
    ) -> ModulResult<PipelineTriangleRtPkg> {
        let _ = (pl_stp.extent_width_stp, pl_stp.extent_height_stp);
        let input_assembly = <Self as PipelineTriangleInputAssemblyHandled>::handled_assemble(
            pl_stp.topology_op,
            pl_stp.primitive_restart_enable_stp,
        );
        let raster = <Self as PipelineTriangleRasterStateHandled>::handled_assemble(
            pl_stp.polygon_mode_op,
            pl_stp.cull_mode_op,
            pl_stp.front_face_op,
            pl_stp.line_width_stp,
        );
        let multisample = <Self as PipelineTriangleMultisampleStateHandled>::handled_assemble(
            pl_stp.sample_count_op,
        );
        let color_blend_attachment =
            <Self as PipelineTriangleColorBlendAttachmentHandled>::handled_assemble(
                pl_stp.color_write_mask_op,
                pl_stp.blend_enable_stp,
                pl_stp.src_color_blend_factor_op,
                pl_stp.dst_color_blend_factor_op,
                pl_stp.color_blend_op,
            );
        let color_blend = vk::PipelineColorBlendStateCreateInfo::default()
            .attachments(std::slice::from_ref(&color_blend_attachment));
        let depth_stencil = <Self as PipelineTriangleDepthStencilStateHandled>::handled_assemble(
            pl_stp.depth_test_enable_stp,
            pl_stp.depth_write_enable_stp,
            pl_stp.depth_compare_op,
        );
        let stages = <Self as PipelineTriangleShaderStagesAuto>::auto_assemble(
            vert_module_extrl,
            frag_module_extrl,
        );
        let pipeline_layout_extrl =
            <Self as PipelineTriangleLayoutAuto>::auto_assemble(device_extrl)?;
        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default();
        let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic_state =
            vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_states);
        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewport_count(1)
            .scissor_count(1);
        let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&raster)
            .multisample_state(&multisample)
            .color_blend_state(&color_blend)
            .depth_stencil_state(&depth_stencil)
            .dynamic_state(&dynamic_state)
            .layout(pipeline_layout_extrl)
            .render_pass(render_pass_extrl)
            .subpass(0);
        let pipeline_extrl = vk::Pipeline::auto_assemble(
            device_extrl,
            vk::PipelineCache::null(),
            &pipeline_info,
        )?;
        Ok(PipelineTriangleRtPkg {
            pipeline_extrl,
            pipeline_layout_extrl,
            desc: pl_stp.desc,
        })
    }
}

// ========== MESH SOLID (cubes product · pos+nrm+instance) ==========

/// Catalog — `mesh_solid` SPIR-V (pos+nrm solid).
pub trait ShadersMeshSolidAuto {
    fn auto_assemble(device_extrl: &Device) -> ModulResult<ShadersTriangleRtPkg>;
}

impl ShadersMeshSolidAuto for ShadersTriangleRtPkg {
    fn auto_assemble(device_extrl: &Device) -> ModulResult<ShadersTriangleRtPkg> {
        let vert_code =
            include_bytes!("../../../../../../../shader/cubes.vert.spv").as_slice();
        let frag_code =
            include_bytes!("../../../../../../../shader/cubes.frag.spv").as_slice();
        let shader_modules_extrl =
            <Self as ShadersTriangleModulesAuto>::auto_assemble(
                device_extrl,
                vert_code,
                frag_code,
            )?;
        Ok(ShadersTriangleRtPkg {
            shader_modules_extrl,
            desc: "shaders_mesh_solid",
        })
    }
}

/// Catalog — graphics pipeline for `mesh_solid` (vertex input + push constants).
pub trait PipelineMeshSolidHandled {
    fn handled_assemble(
        device_extrl: &Device,
        sample_count_op: vk::SampleCountFlags,
        topology_op: vk::PrimitiveTopology,
        polygon_mode_op: vk::PolygonMode,
        cull_mode_op: vk::CullModeFlags,
        front_face_op: vk::FrontFace,
        depth_compare_op: vk::CompareOp,
        color_write_mask_op: vk::ColorComponentFlags,
        render_pass_extrl: vk::RenderPass,
        vert_module_extrl: vk::ShaderModule,
        frag_module_extrl: vk::ShaderModule,
    ) -> ModulResult<PipelineTriangleRtPkg>;
}

impl PipelineMeshSolidHandled for PipelineTriangleRtPkg {
    fn handled_assemble(
        device_extrl: &Device,
        sample_count_op: vk::SampleCountFlags,
        topology_op: vk::PrimitiveTopology,
        polygon_mode_op: vk::PolygonMode,
        cull_mode_op: vk::CullModeFlags,
        front_face_op: vk::FrontFace,
        depth_compare_op: vk::CompareOp,
        color_write_mask_op: vk::ColorComponentFlags,
        render_pass_extrl: vk::RenderPass,
        vert_module_extrl: vk::ShaderModule,
        frag_module_extrl: vk::ShaderModule,
    ) -> ModulResult<PipelineTriangleRtPkg> {
        {
            let input_assembly =
                <Self as PipelineTriangleInputAssemblyHandled>::handled_assemble(
                    topology_op,
                    false,
                );
            let raster =
                <Self as PipelineTriangleRasterStateHandled>::handled_assemble(
                    polygon_mode_op,
                    cull_mode_op,
                    front_face_op,
                    1.0,
                );
            let multisample =
                <Self as PipelineTriangleMultisampleStateHandled>::handled_assemble(
                    sample_count_op,
                );
            let color_blend_attachment =
                <Self as PipelineTriangleColorBlendAttachmentHandled>::handled_assemble(
                    color_write_mask_op,
                    false,
                    vk::BlendFactor::ONE,
                    vk::BlendFactor::ZERO,
                    vk::BlendOp::ADD,
                );
            let color_blend = vk::PipelineColorBlendStateCreateInfo::default()
                .attachments(std::slice::from_ref(&color_blend_attachment));
            let depth_stencil =
                <Self as PipelineTriangleDepthStencilStateHandled>::handled_assemble(
                    true,
                    true,
                    depth_compare_op,
                );
            let stages =
                <Self as PipelineTriangleShaderStagesAuto>::auto_assemble(
                    vert_module_extrl,
                    frag_module_extrl,
                );
            // Push: mat4 + 6×vec4 = 160 bytes (mesh_solid · 3D View look knobs).
            let push_range = vk::PushConstantRange::default()
                .stage_flags(vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT)
                .offset(0)
                .size(MESH_PUSH_RT_SIZE);
            let pipeline_layout_extrl =
                <vk::PipelineLayout as PipelineLayoutAuto>::auto_assemble(
                    device_extrl,
                    &[],
                    std::slice::from_ref(&push_range),
                )?;
            // binding 0 VERTEX: stride 24 · loc0 pos · loc1 nrm
            // binding 1 INSTANCE: stride 16 · loc2 instance xyzw
            let bindings = [
                vk::VertexInputBindingDescription::default()
                    .binding(0)
                    .stride(24)
                    .input_rate(vk::VertexInputRate::VERTEX),
                vk::VertexInputBindingDescription::default()
                    .binding(1)
                    .stride(16)
                    .input_rate(vk::VertexInputRate::INSTANCE),
            ];
            let attrs = [
                vk::VertexInputAttributeDescription::default()
                    .location(0)
                    .binding(0)
                    .format(vk::Format::R32G32B32_SFLOAT)
                    .offset(0),
                vk::VertexInputAttributeDescription::default()
                    .location(1)
                    .binding(0)
                    .format(vk::Format::R32G32B32_SFLOAT)
                    .offset(12),
                vk::VertexInputAttributeDescription::default()
                    .location(2)
                    .binding(1)
                    .format(vk::Format::R32G32B32A32_SFLOAT)
                    .offset(0),
            ];
            let vertex_input = vk::PipelineVertexInputStateCreateInfo::default()
                .vertex_binding_descriptions(&bindings)
                .vertex_attribute_descriptions(&attrs);
            let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
            let dynamic_state =
                vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_states);
            let viewport_state = vk::PipelineViewportStateCreateInfo::default()
                .viewport_count(1)
                .scissor_count(1);
            let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
                .stages(&stages)
                .vertex_input_state(&vertex_input)
                .input_assembly_state(&input_assembly)
                .viewport_state(&viewport_state)
                .rasterization_state(&raster)
                .multisample_state(&multisample)
                .color_blend_state(&color_blend)
                .depth_stencil_state(&depth_stencil)
                .dynamic_state(&dynamic_state)
                .layout(pipeline_layout_extrl)
                .render_pass(render_pass_extrl)
                .subpass(0);
            let pipeline_extrl = vk::Pipeline::auto_assemble(
                device_extrl,
                vk::PipelineCache::null(),
                &pipeline_info,
            )?;
            Ok(PipelineTriangleRtPkg {
                pipeline_extrl,
                pipeline_layout_extrl,
                desc: "pipeline_mesh_solid",
            })
        }
    }
}

// ========== LINE (optional grid/sketch/outline peels) ==========

/// `ShadersLineAuto` — trait (shaders cad line auto).
///
/// Auto-assemble catalog trait: pure construction without external peels (FIX-129 Auto rank).
/// Belongs to: render-pass / graphics pipeline MCG.
/// Module path context: `gpu/MODUL0_VK_PIPELINE/mem/asm_disasm/vk_pkg/handled`.
pub trait ShadersLineAuto {
    fn auto_assemble(device_extrl: &Device) -> ModulResult<ShadersTriangleRtPkg>;
}

impl ShadersLineAuto for ShadersTriangleRtPkg {
    fn auto_assemble(device_extrl: &Device) -> ModulResult<ShadersTriangleRtPkg> {
        // LinePushRt · vertex loc0 xyz only (push size = LINE_PUSH_RT_SIZE).
        let vert_code = include_bytes!("../../../../../../../shader/line.vert.spv").as_slice();
        let frag_code = include_bytes!("../../../../../../../shader/line.frag.spv").as_slice();
        let shader_modules_extrl =
            <Self as ShadersTriangleModulesAuto>::auto_assemble(
                device_extrl,
                vert_code,
                frag_code,
            )?;
        Ok(ShadersTriangleRtPkg {
            shader_modules_extrl,
            desc: "shaders_line",
        })
    }
}

/// `PipelineLineHandled` — LINE_LIST pipeline (pos VBO · LinePushRt).
///
/// Handled-assemble catalog trait: construction from imported knobs / peels (FIX-129 Handled rank).
/// Belongs to: render-pass / graphics pipeline MCG.
/// Module path context: `gpu/MODUL0_VK_PIPELINE/mem/asm_disasm/vk_pkg/handled`.
pub trait PipelineLineHandled {
    fn handled_assemble(
        device_extrl: &Device,
        sample_count_op: vk::SampleCountFlags,
        depth_compare_op: vk::CompareOp,
        color_write_mask_op: vk::ColorComponentFlags,
        render_pass_extrl: vk::RenderPass,
        vert_module_extrl: vk::ShaderModule,
        frag_module_extrl: vk::ShaderModule,
    ) -> ModulResult<PipelineTriangleRtPkg>;
}

/// Catalog — line ribbons as `TRIANGLE_LIST`.
pub trait PipelineLineTrisHandled {
    fn handled_assemble(
        device_extrl: &Device,
        sample_count_op: vk::SampleCountFlags,
        depth_compare_op: vk::CompareOp,
        color_write_mask_op: vk::ColorComponentFlags,
        render_pass_extrl: vk::RenderPass,
        vert_module_extrl: vk::ShaderModule,
        frag_module_extrl: vk::ShaderModule,
    ) -> ModulResult<PipelineTriangleRtPkg>;
}

impl PipelineLineHandled for PipelineTriangleRtPkg {
    fn handled_assemble(
        device_extrl: &Device,
        sample_count_op: vk::SampleCountFlags,
        depth_compare_op: vk::CompareOp,
        color_write_mask_op: vk::ColorComponentFlags,
        render_pass_extrl: vk::RenderPass,
        vert_module_extrl: vk::ShaderModule,
        frag_module_extrl: vk::ShaderModule,
    ) -> ModulResult<PipelineTriangleRtPkg> {
        {
            let topology_op = vk::PrimitiveTopology::LINE_LIST;
            let input_assembly =
                <Self as PipelineTriangleInputAssemblyHandled>::handled_assemble(
                    topology_op,
                    false,
                );
            let raster =
                <Self as PipelineTriangleRasterStateHandled>::handled_assemble(
                    vk::PolygonMode::FILL,
                    vk::CullModeFlags::NONE,
                    vk::FrontFace::COUNTER_CLOCKWISE,
                    1.0,
                );
            let multisample =
                <Self as PipelineTriangleMultisampleStateHandled>::handled_assemble(
                    sample_count_op,
                );
            let color_blend_attachment =
                <Self as PipelineTriangleColorBlendAttachmentHandled>::handled_assemble(
                    color_write_mask_op,
                    false,
                    vk::BlendFactor::ONE,
                    vk::BlendFactor::ZERO,
                    vk::BlendOp::ADD,
                );
            let color_blend = vk::PipelineColorBlendStateCreateInfo::default()
                .attachments(std::slice::from_ref(&color_blend_attachment));
            let depth_stencil =
                <Self as PipelineTriangleDepthStencilStateHandled>::handled_assemble(
                    true,
                    false,
                    depth_compare_op,
                );
            let stages =
                <Self as PipelineTriangleShaderStagesAuto>::auto_assemble(
                    vert_module_extrl,
                    frag_module_extrl,
                );
            let push_range = vk::PushConstantRange::default()
                .stage_flags(vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT)
                .offset(0)
                .size(LINE_PUSH_RT_SIZE);
            let pipeline_layout_extrl =
                <vk::PipelineLayout as PipelineLayoutAuto>::auto_assemble(
                    device_extrl,
                    &[],
                    std::slice::from_ref(&push_range),
                )?;
            let binding = vk::VertexInputBindingDescription::default()
                .binding(0)
                .stride(12)
                .input_rate(vk::VertexInputRate::VERTEX);
            let attrs = [vk::VertexInputAttributeDescription::default()
                .location(0)
                .binding(0)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(0)];
            let vertex_input = vk::PipelineVertexInputStateCreateInfo::default()
                .vertex_binding_descriptions(std::slice::from_ref(&binding))
                .vertex_attribute_descriptions(&attrs);
            let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
            let dynamic_state =
                vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_states);
            let viewport_state = vk::PipelineViewportStateCreateInfo::default()
                .viewport_count(1)
                .scissor_count(1);
            let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
                .stages(&stages)
                .vertex_input_state(&vertex_input)
                .input_assembly_state(&input_assembly)
                .viewport_state(&viewport_state)
                .rasterization_state(&raster)
                .multisample_state(&multisample)
                .color_blend_state(&color_blend)
                .depth_stencil_state(&depth_stencil)
                .dynamic_state(&dynamic_state)
                .layout(pipeline_layout_extrl)
                .render_pass(render_pass_extrl)
                .subpass(0);
            let pipeline_extrl = vk::Pipeline::auto_assemble(
                device_extrl,
                vk::PipelineCache::null(),
                &pipeline_info,
            )?;
            Ok(PipelineTriangleRtPkg {
                pipeline_extrl,
                pipeline_layout_extrl,
                desc: "pipeline_line",
            })
        }
    }
}

impl PipelineLineTrisHandled for PipelineTriangleRtPkg {
    fn handled_assemble(
        device_extrl: &Device,
        sample_count_op: vk::SampleCountFlags,
        depth_compare_op: vk::CompareOp,
        color_write_mask_op: vk::ColorComponentFlags,
        render_pass_extrl: vk::RenderPass,
        vert_module_extrl: vk::ShaderModule,
        frag_module_extrl: vk::ShaderModule,
    ) -> ModulResult<PipelineTriangleRtPkg> {
        {
            let topology_op = vk::PrimitiveTopology::TRIANGLE_LIST;
            let input_assembly =
                <Self as PipelineTriangleInputAssemblyHandled>::handled_assemble(
                    topology_op,
                    false,
                );
            let raster =
                <Self as PipelineTriangleRasterStateHandled>::handled_assemble(
                    vk::PolygonMode::FILL,
                    vk::CullModeFlags::NONE,
                    vk::FrontFace::COUNTER_CLOCKWISE,
                    1.0,
                );
            let multisample =
                <Self as PipelineTriangleMultisampleStateHandled>::handled_assemble(
                    sample_count_op,
                );
            let color_blend_attachment =
                <Self as PipelineTriangleColorBlendAttachmentHandled>::handled_assemble(
                    color_write_mask_op,
                    false,
                    vk::BlendFactor::ONE,
                    vk::BlendFactor::ZERO,
                    vk::BlendOp::ADD,
                );
            let color_blend = vk::PipelineColorBlendStateCreateInfo::default()
                .attachments(std::slice::from_ref(&color_blend_attachment));
            let depth_stencil =
                <Self as PipelineTriangleDepthStencilStateHandled>::handled_assemble(
                    true,
                    false,
                    depth_compare_op,
                );
            let stages =
                <Self as PipelineTriangleShaderStagesAuto>::auto_assemble(
                    vert_module_extrl,
                    frag_module_extrl,
                );
            let push_range = vk::PushConstantRange::default()
                .stage_flags(vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT)
                .offset(0)
                .size(LINE_PUSH_RT_SIZE);
            let pipeline_layout_extrl =
                <vk::PipelineLayout as PipelineLayoutAuto>::auto_assemble(
                    device_extrl,
                    &[],
                    std::slice::from_ref(&push_range),
                )?;
            let binding = vk::VertexInputBindingDescription::default()
                .binding(0)
                .stride(12)
                .input_rate(vk::VertexInputRate::VERTEX);
            let attrs = [vk::VertexInputAttributeDescription::default()
                .location(0)
                .binding(0)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(0)];
            let vertex_input = vk::PipelineVertexInputStateCreateInfo::default()
                .vertex_binding_descriptions(std::slice::from_ref(&binding))
                .vertex_attribute_descriptions(&attrs);
            let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
            let dynamic_state =
                vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_states);
            let viewport_state = vk::PipelineViewportStateCreateInfo::default()
                .viewport_count(1)
                .scissor_count(1);
            let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
                .stages(&stages)
                .vertex_input_state(&vertex_input)
                .input_assembly_state(&input_assembly)
                .viewport_state(&viewport_state)
                .rasterization_state(&raster)
                .multisample_state(&multisample)
                .color_blend_state(&color_blend)
                .depth_stencil_state(&depth_stencil)
                .dynamic_state(&dynamic_state)
                .layout(pipeline_layout_extrl)
                .render_pass(render_pass_extrl)
                .subpass(0);
            let pipeline_extrl = vk::Pipeline::auto_assemble(
                device_extrl,
                vk::PipelineCache::null(),
                &pipeline_info,
            )?;
            Ok(PipelineTriangleRtPkg {
                pipeline_extrl,
                pipeline_layout_extrl,
                desc: "pipeline_line_tris",
            })
        }
    }
}
