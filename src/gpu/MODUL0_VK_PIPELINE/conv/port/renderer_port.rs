//! Renderer subject port · **import_for_asm9** (swapchain calque).

use ash::vk;

use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_bfr::auto::renderer_bfr_at_asm::RendererBfrAuto;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_crg::handled::renderer_default_rt_crg_hld_asm::RendererDefaultRtCrgHandled;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_pkg::handled::render_res_intsct_hld_asm::PipelineLineHandled;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_pkg::handled::render_res_intsct_hld_asm::PipelineLineTrisHandled;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_pkg::handled::render_res_intsct_hld_asm::PipelineMeshSolidHandled;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_pkg::handled::render_res_intsct_hld_asm::PipelineTriangleHandled;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_pkg::handled::render_res_intsct_hld_asm::RenderPassTriangleHandled;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_pkg::handled::render_res_intsct_hld_asm::ShadersLineAuto;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_pkg::handled::render_res_intsct_hld_asm::ShadersMeshSolidAuto;
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_pkg::auto::render_lane_stp_at_asm::{
    PipelineTriangleStpAuto, RenderPassTriangleStpAuto,
};
use crate::gpu::MODUL0_VK_PIPELINE::mem::asm_disasm::vk_pkg::handled::render_res_intsct_hld_asm::ShadersTriangleAuto;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::embedded::buffer::RendererBfr;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::prt::render_lane_prt::RenderLanePrt;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::PipelineTriangleRtPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::RenderPassTriangleRtPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::RendererDefaultRtCrg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::ShadersTriangleRtPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::render_res_intsct_stp_pkgs::PipelineTriangleStpPkg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::setup::render_res_intsct_stp_pkgs::RenderPassTriangleStpPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::DeviceDefaultRtPkg;
use crate::ModulResult;

/// Intent line · **1** = PortMatch `RenderLanePrt` → *Stp slots on Bfr only.
pub const IMPORT_RENDER_LANE_FOR_ASM_FACTORY_LINE_N: u8 = 1;

/// Full factory-line · **9** = lane *Stp already on Bfr · 7 atom · pack (or lane+atoms if called as one shot).
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
/// Same shape as [`SwapchainTransportable`](crate::gpu::MODUL0_VK_SWAPCHAIN::conv::port::SwapchainTransportable):
/// several **import_*** methods on the trait, all write **`&mut Bfr`**, never return bags.
///
/// | Method | Role |
/// |--------|------|
/// | [`import_render_lane_for_asm1`](Self::import_render_lane_for_asm1) | Intent → *Stp slots (PortMatch) |
/// | [`import_for_asm9`](Self::import_for_asm9) | Atom assemblies + pack cargo from Bfr slots |
/// | [`export_asmed1`](Self::export_asmed1) | Peel asmed cargo |
///
/// *Stp packages are **setup bags** (closed gestalt), not Vulkan creates — so they are
/// filled by Auto *Stp assemblers (`render_lane_stp_at_asm`) onto Bfr.
/// Runtime *RtPkg still go through Handled (`RenderPassTriangleHandled`, …).
pub trait RendererTransportable {
    /// Intent import · **1** · `RenderLanePrt` + external extent/format → *Stp on Bfr.
    /// Write-only · `Result<()>` only for trait uniformity · never returns a bag.
    fn import_render_lane_for_asm1(
        bfr: &mut Self,
        render_lane_prt: RenderLanePrt,
        extent_width_stp: u32,
        extent_height_stp: u32,
        surface_format_op: vk::Format,
    );

    /// Full factory-line · **9** · runs lane intent then atom asm 1..8 + pack.
    /// Requires (or performs) *Stp fill · barter: device from swapchain boot.
    fn import_for_asm9(
        bfr: &mut Self,
        render_lane_prt: RenderLanePrt,
        extent_width_stp: u32,
        extent_height_stp: u32,
        surface_format_op: vk::Format,
        device_default_rt_pkg: &DeviceDefaultRtPkg,
    ) -> ModulResult<()>;

    /// Handled factory-line · **8 atom + pack** · *Stp already on Bfr (no Auto Prt table).
    /// Caller writes full `RenderPassTriangleStpPkg` + `PipelineTriangleStpPkg` with every op.
    fn import_for_asm8_from_stp(
        bfr: &mut Self,
        device_default_rt_pkg: &DeviceDefaultRtPkg,
    ) -> ModulResult<()>;

    fn export_asmed1(bfr: &Self) -> Option<&RendererDefaultRtCrg>;
}

impl RendererTransportable for RendererBfr {
    fn import_render_lane_for_asm1(
        bfr: &mut Self,
        render_lane_prt: RenderLanePrt,
        extent_width_stp: u32,
        extent_height_stp: u32,
        surface_format_op: vk::Format,
    ) {
        debug_assert_eq!(IMPORT_RENDER_LANE_FOR_ASM_FACTORY_LINE_N, 1);
        // Auto *Stp from RenderLanePrt (asm_disasm presets) → Bfr slots only.
        bfr.render_pass_triangle_stp_pkg = Some(RenderPassTriangleStpPkg::auto_assemble(
            render_lane_prt,
            surface_format_op,
        ));
        bfr.pipeline_triangle_stp_pkg = Some(PipelineTriangleStpPkg::auto_assemble(
            render_lane_prt,
            extent_width_stp,
            extent_height_stp,
        ));
    }

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

        // Step 0 · intent on Bfr (Transportable · ¬ free helper / ¬ local theater bags)
        Self::import_render_lane_for_asm1(
            bfr,
            render_lane_prt,
            extent_width_stp,
            extent_height_stp,
            surface_format_op,
        );

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
                rp_stp.color_load_op,
                rp_stp.color_store_op,
                rp_stp.depth_load_op,
                rp_stp.depth_store_op,
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
                pl_stp,
                rp_rt_pass,
                vert,
                frag,
            )?,
        );

        // asm 4/9 · steel shaders
        bfr.shaders_mesh_solid_rt_pkg = Some(
            <ShadersTriangleRtPkg as ShadersMeshSolidAuto>::auto_assemble(device_extrl)?,
        );

        // asm 5/9 · steel pipeline
        let steel = bfr.shaders_steel()?;
        let (steel_vert, steel_frag) = extract_shader_pair(&steel.shader_modules_extrl)?;
        bfr.pipeline_mesh_solid_rt_pkg = Some(
            <PipelineTriangleRtPkg as PipelineMeshSolidHandled>::handled_assemble(
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
            <ShadersTriangleRtPkg as ShadersLineAuto>::auto_assemble(device_extrl)?,
        );

        // asm 7/9 · line pipeline
        let line = bfr.shaders_line()?;
        let (line_vert, line_frag) = extract_shader_pair(&line.shader_modules_extrl)?;
        bfr.pipeline_line_rt_pkg = Some(
            <PipelineTriangleRtPkg as PipelineLineHandled>::handled_assemble(
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
            <PipelineTriangleRtPkg as PipelineLineTrisHandled>::handled_assemble(
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

    fn import_for_asm8_from_stp(
        bfr: &mut Self,
        device_default_rt_pkg: &DeviceDefaultRtPkg,
    ) -> ModulResult<()> {
        // Stp must already be written (handled knobs path) — no RenderLanePrt Auto table.
        let _ = bfr.rp_stp()?;
        let _ = bfr.pl_stp()?;
        let device_extrl = &device_default_rt_pkg.device_extrl;

        bfr.shaders_triangle_rt_pkg = Some(
            <ShadersTriangleRtPkg as ShadersTriangleAuto>::auto_assemble(device_extrl)?,
        );

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
                rp_stp.color_load_op,
                rp_stp.color_store_op,
                rp_stp.depth_load_op,
                rp_stp.depth_store_op,
            )?,
        );

        let pl_stp = *bfr.pl_stp()?;
        let rp_rt_pass = bfr.rp_rt()?.render_pass_extrl;

        let shaders = bfr.shaders_tri()?;
        let (vert, frag) = extract_shader_pair(&shaders.shader_modules_extrl)?;
        bfr.pipeline_triangle_rt_pkg = Some(
            <PipelineTriangleRtPkg as PipelineTriangleHandled>::handled_assemble(
                device_extrl,
                pl_stp,
                rp_rt_pass,
                vert,
                frag,
            )?,
        );

        bfr.shaders_mesh_solid_rt_pkg = Some(
            <ShadersTriangleRtPkg as ShadersMeshSolidAuto>::auto_assemble(device_extrl)?,
        );

        let steel = bfr.shaders_steel()?;
        let (steel_vert, steel_frag) = extract_shader_pair(&steel.shader_modules_extrl)?;
        bfr.pipeline_mesh_solid_rt_pkg = Some(
            <PipelineTriangleRtPkg as PipelineMeshSolidHandled>::handled_assemble(
                device_extrl,
                pl_stp.sample_count_op,
                pl_stp.topology_op,
                pl_stp.polygon_mode_op,
                vk::CullModeFlags::NONE,
                pl_stp.front_face_op,
                pl_stp.depth_compare_op,
                pl_stp.color_write_mask_op,
                rp_rt_pass,
                steel_vert,
                steel_frag,
            )?,
        );

        bfr.shaders_line_rt_pkg = Some(
            <ShadersTriangleRtPkg as ShadersLineAuto>::auto_assemble(device_extrl)?,
        );

        let line = bfr.shaders_line()?;
        let (line_vert, line_frag) = extract_shader_pair(&line.shader_modules_extrl)?;
        bfr.pipeline_line_rt_pkg = Some(
            <PipelineTriangleRtPkg as PipelineLineHandled>::handled_assemble(
                device_extrl,
                pl_stp.sample_count_op,
                pl_stp.depth_compare_op,
                pl_stp.color_write_mask_op,
                rp_rt_pass,
                line_vert,
                line_frag,
            )?,
        );

        bfr.pipeline_line_tris_rt_pkg = Some(
            <PipelineTriangleRtPkg as PipelineLineTrisHandled>::handled_assemble(
                device_extrl,
                pl_stp.sample_count_op,
                pl_stp.depth_compare_op,
                pl_stp.color_write_mask_op,
                rp_rt_pass,
                line_vert,
                line_frag,
            )?,
        );

        let cargo_rt = RendererDefaultRtCrg::handled_assemble(bfr)?;
        bfr.cargo_rt = Some(cargo_rt);
        Ok(())
    }

    fn export_asmed1(bfr: &Self) -> Option<&RendererDefaultRtCrg> {
        bfr.cargo_rt.as_ref()
    }
}

