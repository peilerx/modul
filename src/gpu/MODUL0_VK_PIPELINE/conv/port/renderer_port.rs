//! Renderer subject port · **import_for_asm9** (swapchain calque).

use ash::vk;

use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_bfr::auto::renderer_bfr_at_asm::RendererBfrAuto;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_crg::handled::renderer_default_rt_crg_hld_asm::RendererDefaultRtCrgHandled;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_pkg::handled::render_res_intsct_hld_asm::PipelineCadLineHandled;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_pkg::handled::render_res_intsct_hld_asm::PipelineCadLineTrisHandled;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_pkg::handled::render_res_intsct_hld_asm::PipelineCadSteelHandled;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_pkg::handled::render_res_intsct_hld_asm::PipelineTriangleHandled;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_pkg::handled::render_res_intsct_hld_asm::RenderPassTriangleHandled;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_pkg::handled::render_res_intsct_hld_asm::ShadersCadLineAuto;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_pkg::handled::render_res_intsct_hld_asm::ShadersCadSteelAuto;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_pkg::handled::render_res_intsct_hld_asm::ShadersTriangleAuto;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::embedded::buffer::RendererBfr;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::prt::render_lane_prt::RenderLanePrt;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::PipelineTriangleRtPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::RenderPassTriangleRtPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::RendererDefaultRtCrg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::ShadersTriangleRtPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::op::RenderPassAttachmentLayoutStpPkgOp;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::render_res_intsct_stp_pkgs::PipelineTriangleStpPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::render_res_intsct_stp_pkgs::RenderPassTriangleStpPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::DeviceDefaultRtPkg;
use crate::ModulResult;

/// Render lane factory-line · **9** = Intent→stp + 7 atom + pack.
pub const IMPORT_FOR_ASM_FACTORY_LINE_N: u8 = 9;

/// Vert/frag modules from an Auto shader pack (exactly two stages).
fn extract_shader_pair(modules: &[vk::ShaderModule]) -> ModulResult<(vk::ShaderModule, vk::ShaderModule)> {
    match modules {
        [vert, frag] => Ok((*vert, *frag)),
        _ => Err(format!(
            "renderer: expected 2 shader modules (vert+frag), got {}",
            modules.len()
        )),
    }
}

/// `RendererTransportable` — trait (renderer transportable).
///
/// Transportable surface: import/export peels for PTP slot-factory-line wiring.
/// Belongs to: render-pass / graphics pipeline MCG.
/// Module path context: `gpu/MODUL0_VK_PIPELINE/conv/port`.
pub trait RendererTransportable {
    fn import_for_asm9(
        bfr: &mut Self,
        render_lane_prt: RenderLanePrt,
        extent_width_stp: u32,
        extent_height_stp: u32,
        surface_format_op: vk::Format,
        device_default_rt_pkg: &DeviceDefaultRtPkg,
    ) -> ModulResult<()>;

    fn export_asmed1(bfr: &Self) -> Option<&RendererDefaultRtCrg>;
}

impl RendererTransportable for RendererBfr {
    fn import_for_asm9(
        bfr: &mut Self,
        render_lane_prt: RenderLanePrt,
        extent_width_stp: u32,
        extent_height_stp: u32,
        surface_format_op: vk::Format,
        device_default_rt_pkg: &DeviceDefaultRtPkg,
    ) -> ModulResult<()> {
        debug_assert_eq!(IMPORT_FOR_ASM_FACTORY_LINE_N, 9);
        let device_extrl = &device_default_rt_pkg.device_extrl;

        // Intent → setup slots (closed gestalt · legacy PortMatch helper)
        let mut rp = RenderPassTriangleStpPkg {
            surface_format_op: vk::Format::UNDEFINED,
            sample_count_op: vk::SampleCountFlags::TYPE_1,
            attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp::SIMPLE,
            depth_format_op: vk::Format::UNDEFINED,
            color_layout_op: vk::ImageLayout::UNDEFINED,
            depth_layout_op: vk::ImageLayout::UNDEFINED,
            present_layout_op: vk::ImageLayout::UNDEFINED,
            initial_layout_op: vk::ImageLayout::UNDEFINED,
            desc: "",
        };
        let mut pl = PipelineTriangleStpPkg {
            sample_count_op: vk::SampleCountFlags::TYPE_1,
            topology_op: vk::PrimitiveTopology::TRIANGLE_LIST,
            polygon_mode_op: vk::PolygonMode::FILL,
            cull_mode_op: vk::CullModeFlags::NONE,
            front_face_op: vk::FrontFace::COUNTER_CLOCKWISE,
            depth_compare_op: vk::CompareOp::LESS,
            color_write_mask_op: vk::ColorComponentFlags::RGBA,
            extent_width_stp: 0,
            extent_height_stp: 0,
            desc: "",
        };
        crate::gpu::MODUL0_VK_PIPELINE::conv::port::import::render_lane::import_render_lane_for_asm(
            render_lane_prt,
            extent_width_stp,
            extent_height_stp,
            surface_format_op,
            &mut rp,
            &mut pl,
        );
        bfr.render_pass_triangle_stp_pkg = Some(rp);
        bfr.pipeline_triangle_stp_pkg = Some(pl);

        // asm 1/9 · triangle shaders
        bfr.shaders_triangle_rt_pkg = Some(
            <ShadersTriangleRtPkg as ShadersTriangleAuto>::auto_assemble(device_extrl)?,
        );

        // asm 2/9 · render pass (stp intents are fixed for this import; cache peels once)
        let rp_stp = bfr.rp_stp()?;
        bfr.render_pass_triangle_rt_pkg = Some(
            <RenderPassTriangleRtPkg as RenderPassTriangleHandled>::handled_assemble(
                device_extrl,
                rp_stp.surface_format_op,
                rp_stp.sample_count_op,
                rp_stp.attachment_layout_op,
                rp_stp.depth_format_op,
                rp_stp.color_layout_op,
                rp_stp.depth_layout_op,
                rp_stp.present_layout_op,
                rp_stp.initial_layout_op,
            )?,
        );

        // Shared pipeline / pass peels for asm 3–8 (avoid repeated bag lookups)
        let pl_stp = *bfr.pl_stp()?;
        let rp_rt_pass = bfr.rp_rt()?.render_pass_extrl;

        // asm 3/9 · triangle pipeline
        let shaders = bfr.shaders_tri()?;
        let (vert, frag) = extract_shader_pair(&shaders.shader_modules_extrl)?;
        bfr.pipeline_triangle_rt_pkg = Some(
            <PipelineTriangleRtPkg as PipelineTriangleHandled>::handled_assemble(
                device_extrl,
                pl_stp.sample_count_op,
                pl_stp.topology_op,
                pl_stp.polygon_mode_op,
                pl_stp.cull_mode_op,
                pl_stp.front_face_op,
                pl_stp.depth_compare_op,
                pl_stp.color_write_mask_op,
                pl_stp.extent_width_stp,
                pl_stp.extent_height_stp,
                rp_rt_pass,
                vert,
                frag,
            )?,
        );

        // asm 4/9 · steel shaders
        bfr.shaders_steel_rt_pkg = Some(
            <ShadersTriangleRtPkg as ShadersCadSteelAuto>::auto_assemble(device_extrl)?,
        );

        // asm 5/9 · steel pipeline
        let steel = bfr.shaders_steel()?;
        let (steel_vert, steel_frag) = extract_shader_pair(&steel.shader_modules_extrl)?;
        bfr.pipeline_steel_rt_pkg = Some(
            <PipelineTriangleRtPkg as PipelineCadSteelHandled>::handled_assemble(
                device_extrl,
                pl_stp.sample_count_op,
                pl_stp.topology_op,
                pl_stp.polygon_mode_op,
                // NONE · exterior planes of solid pulse both sides (no missing faces)
                vk::CullModeFlags::NONE,
                pl_stp.front_face_op,
                pl_stp.depth_compare_op,
                pl_stp.color_write_mask_op,
                rp_rt_pass,
                steel_vert,
                steel_frag,
            )?,
        );

        // asm 6/9 · line shaders
        bfr.shaders_line_rt_pkg = Some(
            <ShadersTriangleRtPkg as ShadersCadLineAuto>::auto_assemble(device_extrl)?,
        );

        // asm 7/9 · line pipeline
        let line = bfr.shaders_line()?;
        let (line_vert, line_frag) = extract_shader_pair(&line.shader_modules_extrl)?;
        bfr.pipeline_line_rt_pkg = Some(
            <PipelineTriangleRtPkg as PipelineCadLineHandled>::handled_assemble(
                device_extrl,
                pl_stp.sample_count_op,
                pl_stp.depth_compare_op,
                pl_stp.color_write_mask_op,
                rp_rt_pass,
                line_vert,
                line_frag,
            )?,
        );

        // asm 8/9 · line tris pipeline (reuses line shader modules)
        bfr.pipeline_line_tris_rt_pkg = Some(
            <PipelineTriangleRtPkg as PipelineCadLineTrisHandled>::handled_assemble(
                device_extrl,
                pl_stp.sample_count_op,
                pl_stp.depth_compare_op,
                pl_stp.color_write_mask_op,
                rp_rt_pass,
                line_vert,
                line_frag,
            )?,
        );

        // asm 9/9 · pack
        let cargo_rt = RendererDefaultRtCrg::handled_assemble(bfr)?;
        bfr.cargo_rt = Some(cargo_rt);
        Ok(())
    }

    fn export_asmed1(bfr: &Self) -> Option<&RendererDefaultRtCrg> {
        bfr.cargo_rt.as_ref()
    }
}
