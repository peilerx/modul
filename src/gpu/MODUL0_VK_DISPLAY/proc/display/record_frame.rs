//! DISPLAY record — product mesh solid + grid/sketch/outline lines on MODUL0_VK.

use ash::vk;
use std::mem::size_of;

use crate::gpu::MODUL0_VK_FRAME::mem::base::embedded::runtime::frame_info_res_intsct_rt::FrameCommandBeginInfoDefaultRt;
use crate::gpu::MODUL0_VK_FRAME::mem::base::embedded::runtime::frame_info_res_intsct_rt::RenderPassBeginInfoTriangleRt;
use crate::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_pkg::auto::command_begin_info_at_asm::auto_vk_cmd_begin;
use crate::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_pkg::handled::render_pass_begin_triangle_hld_asm::handled_vk_rp_begin;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::{FrameRenderDefaultRtPkg, FrameSlotDefaultRtPkg};
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::record_line_layers_rt::RecordLineLayersRt;
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::line_gpu_default_rt_pkg::LinePushRt;
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::{
    MeshPushRt, MeshGpuDefaultRtPkg,
};
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::RendererDefaultRtCrg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::{DeviceDefaultRtPkg, PresentationDefaultRtCrg};
use crate::{map_vk, ModulResult};

/// Record one frame into `slot.command_buffer`.
pub fn record_display_frame(
    device: &DeviceDefaultRtPkg,
    presentation: &PresentationDefaultRtCrg,
    renderer: &RendererDefaultRtCrg,
    slot: &FrameSlotDefaultRtPkg,
    render_policy: &FrameRenderDefaultRtPkg,
    bind_geometry_stp: bool,
    mesh_gpu: Option<&MeshGpuDefaultRtPkg>,
    mesh_push: Option<&MeshPushRt>,
    lines: RecordLineLayersRt<'_>,
    image_index: u32,
) -> ModulResult<()> {
    let image_index_usize = image_index as usize;
    let framebuffer_extrl = *presentation
        .framebuffer_default_rt_pkg
        .framebuffers_extrl
        .get(image_index_usize)
        .ok_or_else(|| {
            format!(
                "record_display_frame: no framebuffer for image_index {image_index} (have {})",
                presentation.framebuffer_default_rt_pkg.framebuffers_extrl.len()
            )
        })?;

    let extent_rt = presentation.swapchain_default_rt_pkg.extent_rt;
    // Peels only — bag literals in proc; catalog assemble stays in asm_disasm (FIX-131).
    let begin_rt = FrameCommandBeginInfoDefaultRt {
        buffer_usage_flags_op: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
        desc: "command_begin_info",
    };
    let begin_info = auto_vk_cmd_begin(&begin_rt);

    unsafe {
        map_vk(
            device
                .device_extrl
                .begin_command_buffer(slot.command_buffer_extrl, &begin_info),
        )?;
    }

    let clear_color = render_policy.clear_color_rt;
    let rp_begin_rt = RenderPassBeginInfoTriangleRt {
        render_pass_extrl: renderer.render_pass_triangle_rt_pkg.render_pass_extrl,
        framebuffer_extrl,
        extent_rt,
        clear_values_rt: [
            vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: clear_color,
                },
            },
            vk::ClearValue {
                depth_stencil: vk::ClearDepthStencilValue {
                    depth: 1.0,
                    stencil: 0,
                },
            },
        ],
        desc: "render_pass_begin_info_triangle",
    };
    let rp_begin = handled_vk_rp_begin(&rp_begin_rt);

    unsafe {
        device.device_extrl.cmd_begin_render_pass(
            slot.command_buffer_extrl,
            &rp_begin,
            vk::SubpassContents::INLINE,
        );

        let viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: extent_rt.width as f32,
            height: extent_rt.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };
        let scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: extent_rt,
        };
        device
            .device_extrl
            .cmd_set_viewport(slot.command_buffer_extrl, 0, std::slice::from_ref(&viewport));
        device
            .device_extrl
            .cmd_set_scissor(slot.command_buffer_extrl, 0, std::slice::from_ref(&scissor));

        // 1) mesh solid steel (all three peels required for a draw)
        if let (Some(mesh), Some(steel_pl), Some(push)) = (
            mesh_gpu.filter(|m| m.ready_rt && m.index_count_rt > 0),
            renderer.pipeline_mesh_solid_rt_pkg.as_ref(),
            mesh_push,
        ) {
            device.device_extrl.cmd_bind_pipeline(
                slot.command_buffer_extrl,
                vk::PipelineBindPoint::GRAPHICS,
                steel_pl.pipeline_extrl,
            );
            let vb = [mesh.vertex_buffer_extrl, mesh.instance_buffer_extrl];
            let offsets = [0_u64, 0_u64];
            device.device_extrl.cmd_bind_vertex_buffers(
                slot.command_buffer_extrl,
                0,
                &vb,
                &offsets,
            );
            device.device_extrl.cmd_bind_index_buffer(
                slot.command_buffer_extrl,
                mesh.index_buffer_extrl,
                0,
                vk::IndexType::UINT32,
            );
            let push_bytes = std::slice::from_raw_parts(
                std::ptr::from_ref::<MeshPushRt>(push).cast::<u8>(),
                size_of::<MeshPushRt>(),
            );
            device.device_extrl.cmd_push_constants(
                slot.command_buffer_extrl,
                steel_pl.pipeline_layout_extrl,
                vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
                0,
                push_bytes,
            );
            let instances = mesh.instance_count_rt.max(1);
            device.device_extrl.cmd_draw_indexed(
                slot.command_buffer_extrl,
                mesh.index_count_rt,
                instances,
                0,
                0,
                0,
            );
        } else if bind_geometry_stp {
            device.device_extrl.cmd_bind_pipeline(
                slot.command_buffer_extrl,
                vk::PipelineBindPoint::GRAPHICS,
                renderer.pipeline_triangle_rt_pkg.pipeline_extrl,
            );
            device
                .device_extrl
                .cmd_draw(slot.command_buffer_extrl, 3, 1, 0, 0);
        }

        // 2) Lines (grid · sketch · outline) after solid so they composite on depth.
        // Outline may be solid TRIANGLE_LIST quads (thick Borderline); grid/sketch stay LINE_LIST.
        if let Some(line_pl) = renderer.pipeline_line_rt_pkg.as_ref() {
            let mvp = mesh_push.map_or([
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ], |p| p.mvp);
            let tri_pl = renderer.pipeline_line_tris_rt_pkg.as_ref();
            for layer in [
                lines.grid_line_gpu_default_rt_pkg,
                lines.sketch_line_gpu_default_rt_pkg,
                lines.outline_line_gpu_default_rt_pkg,
            ] {
                let Some(line) = layer else { continue };
                if !line.ready_rt || line.vertex_count_rt < 2 {
                    continue;
                }
                let pl = if line.as_tris_rt {
                    tri_pl.unwrap_or(line_pl)
                } else {
                    line_pl
                };
                device.device_extrl.cmd_bind_pipeline(
                    slot.command_buffer_extrl,
                    vk::PipelineBindPoint::GRAPHICS,
                    pl.pipeline_extrl,
                );
                let push = LinePushRt::from_mvp_color(mvp, line.color_rt);
                let push_bytes = std::slice::from_raw_parts(
                    (&raw const push).cast::<u8>(),
                    size_of::<LinePushRt>(),
                );
                device.device_extrl.cmd_push_constants(
                    slot.command_buffer_extrl,
                    pl.pipeline_layout_extrl,
                    vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
                    0,
                    push_bytes,
                );
                let vb = [line.vertex_buffer_extrl];
                let offsets = [0_u64];
                device.device_extrl.cmd_bind_vertex_buffers(
                    slot.command_buffer_extrl,
                    0,
                    &vb,
                    &offsets,
                );
                device.device_extrl.cmd_draw(
                    slot.command_buffer_extrl,
                    line.vertex_count_rt,
                    1,
                    0,
                    0,
                );
            }
        }

        device
            .device_extrl
            .cmd_end_render_pass(slot.command_buffer_extrl);
        map_vk(
            device
                .device_extrl
                .end_command_buffer(slot.command_buffer_extrl),
        )?;
    }

    Ok(())
}
