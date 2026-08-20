//! DISPLAY record — compute-only lattice (`vkCmdDispatch`). No `vkCmdDraw*`.

use ash::vk;
use std::mem::size_of;

use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayDefaultRtCrg;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::record_line_layers_rt::RecordLineLayersRt;
use crate::gpu::MODUL0_VK_FRAME::mem::base::embedded::runtime::frame_info_res_intsct_rt::FrameCommandBeginInfoDefaultRt;
use crate::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_pkg::auto::command_begin_info_at_asm::auto_vk_cmd_begin;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::{FrameRenderDefaultRtPkg, FrameSlotDefaultRtPkg};
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::{
    MeshGpuDefaultRtPkg, MeshPushRt,
};
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::RendererDefaultRtCrg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::{DeviceDefaultRtPkg, PresentationDefaultRtCrg};
use crate::{map_vk, ModulResult};

#[repr(C)]
struct SoaRayPushRt {
    inv_mvp: [f32; 16],
    cam_pos: [f32; 4],
    light_dir: [f32; 4],
    base_color: [f32; 4],
    look3: [f32; 4],
    grid: [u32; 4],
    view: [f32; 4],
    brush: [f32; 4],
}

fn mat4_inverse(m: &[f32; 16]) -> [f32; 16] {
    let a00 = m[0];
    let a01 = m[1];
    let a02 = m[2];
    let a03 = m[3];
    let a10 = m[4];
    let a11 = m[5];
    let a12 = m[6];
    let a13 = m[7];
    let a20 = m[8];
    let a21 = m[9];
    let a22 = m[10];
    let a23 = m[11];
    let a30 = m[12];
    let a31 = m[13];
    let a32 = m[14];
    let a33 = m[15];
    let b00 = a00 * a11 - a01 * a10;
    let b01 = a00 * a12 - a02 * a10;
    let b02 = a00 * a13 - a03 * a10;
    let b03 = a01 * a12 - a02 * a11;
    let b04 = a01 * a13 - a03 * a11;
    let b05 = a02 * a13 - a03 * a12;
    let b06 = a20 * a31 - a21 * a30;
    let b07 = a20 * a32 - a22 * a30;
    let b08 = a20 * a33 - a23 * a30;
    let b09 = a21 * a32 - a22 * a31;
    let b10 = a21 * a33 - a23 * a31;
    let b11 = a22 * a33 - a23 * a32;
    let det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;
    let inv = if det.abs() < 1e-12 { 1.0 } else { 1.0 / det };
    [
        (a11 * b11 - a12 * b10 + a13 * b09) * inv,
        (a02 * b10 - a01 * b11 - a03 * b09) * inv,
        (a31 * b05 - a32 * b04 + a33 * b03) * inv,
        (a22 * b04 - a21 * b05 - a23 * b03) * inv,
        (a12 * b08 - a10 * b11 - a13 * b07) * inv,
        (a00 * b11 - a02 * b08 + a03 * b07) * inv,
        (a32 * b02 - a30 * b05 - a33 * b01) * inv,
        (a20 * b05 - a22 * b02 + a23 * b01) * inv,
        (a10 * b10 - a11 * b08 + a13 * b06) * inv,
        (a01 * b08 - a00 * b10 - a03 * b06) * inv,
        (a30 * b04 - a31 * b02 + a33 * b00) * inv,
        (a21 * b02 - a20 * b04 - a23 * b00) * inv,
        (a11 * b07 - a10 * b09 - a12 * b06) * inv,
        (a00 * b09 - a01 * b07 + a02 * b06) * inv,
        (a31 * b01 - a30 * b03 - a32 * b00) * inv,
        (a20 * b03 - a21 * b01 + a22 * b00) * inv,
    ]
}

fn image_barrier(
    image: vk::Image,
    src_stage: vk::PipelineStageFlags,
    dst_stage: vk::PipelineStageFlags,
    src_access: vk::AccessFlags,
    dst_access: vk::AccessFlags,
    old_layout: vk::ImageLayout,
    new_layout: vk::ImageLayout,
) -> (vk::ImageMemoryBarrier<'static>, vk::PipelineStageFlags, vk::PipelineStageFlags) {
    let barrier = vk::ImageMemoryBarrier::default()
        .src_access_mask(src_access)
        .dst_access_mask(dst_access)
        .old_layout(old_layout)
        .new_layout(new_layout)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(image)
        .subresource_range(vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        });
    (barrier, src_stage, dst_stage)
}

/// Record one frame: `vkCmdDispatch` into storage image, blit to swapchain. No draw.
pub fn record_display_frame(
    device: &DeviceDefaultRtPkg,
    presentation: &PresentationDefaultRtCrg,
    renderer: &RendererDefaultRtCrg,
    slot: &FrameSlotDefaultRtPkg,
    _render_policy: &FrameRenderDefaultRtPkg,
    _bind_geometry_stp: bool,
    mesh_gpu: Option<&MeshGpuDefaultRtPkg>,
    mesh_push: Option<&MeshPushRt>,
    _lines: RecordLineLayersRt<'_>,
    image_index: u32,
    display: &mut DisplayDefaultRtCrg,
) -> ModulResult<()> {
    let Some(comp) = renderer.pipeline_mesh_soa_comp_rt_pkg.as_ref() else {
        return Err("record: compute pipeline missing (soa-vulkan forbids cmdDraw)".into());
    };
    let Some(set) = renderer
        .descriptor_sets_default_rt_pkg
        .as_ref()
        .and_then(|s| s.descriptor_sets_extrl.first())
        .copied()
    else {
        return Err("record: compute descriptor set missing".into());
    };
    if display.soa_color_image_extrl == vk::Image::null() {
        return Err("record: soa color image missing".into());
    }
    let Some(push) = mesh_push else {
        return Err("record: mesh push missing".into());
    };
    let swap_image = *presentation
        .swapchain_default_rt_pkg
        .images_extrl
        .get(image_index as usize)
        .ok_or_else(|| format!("record: no swapchain image {image_index}"))?;
    let extent = presentation.swapchain_default_rt_pkg.extent_rt;
    let src_ext = display.soa_color_extent_rt;
    let src_w = src_ext.width.max(1);
    let src_h = src_ext.height.max(1);
    let n = mesh_gpu.map_or(1, |m| m.instance_count_rt.max(1));
    let nx = f64::from(n).cbrt().round().max(1.0) as u32;
    let ny = nx;
    let nz = nx;
    // AABB includes unit-cube ±0.5 — subtract that or pitch is too big (field looks far).
    let pitch = mesh_gpu.map_or(1.25, |m| {
        let span = (m.bounds_max_rt[0] - m.bounds_min_rt[0]).abs();
        let centers = (span - 1.0).max(0.25);
        if nx <= 1 {
            1.25
        } else {
            (centers / (nx - 1) as f32).max(0.25)
        }
    });

    let begin_info = auto_vk_cmd_begin(&FrameCommandBeginInfoDefaultRt {
        buffer_usage_flags_op: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
        desc: "command_begin_info",
    });
    unsafe {
        map_vk(
            device
                .device_extrl
                .begin_command_buffer(slot.command_buffer_extrl, &begin_info),
        )?;
    }

    let cmd = slot.command_buffer_extrl;
    let color = display.soa_color_image_extrl;
    unsafe {
        let (b0, s0, d0) = image_barrier(
            color,
            vk::PipelineStageFlags::TOP_OF_PIPE,
            vk::PipelineStageFlags::COMPUTE_SHADER,
            vk::AccessFlags::empty(),
            vk::AccessFlags::SHADER_WRITE,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::GENERAL,
        );
        device.device_extrl.cmd_pipeline_barrier(
            cmd,
            s0,
            d0,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            std::slice::from_ref(&b0),
        );

        if display.soa_heat_buffer_extrl != vk::Buffer::null() && !display.soa_heat_cleared_rt {
            device.device_extrl.cmd_fill_buffer(
                cmd,
                display.soa_heat_buffer_extrl,
                0,
                display.soa_heat_bytes_rt,
                0,
            );
            display.soa_heat_cleared_rt = true;
            let fill_bar = vk::BufferMemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(vk::AccessFlags::SHADER_READ | vk::AccessFlags::SHADER_WRITE)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .buffer(display.soa_heat_buffer_extrl)
                .offset(0)
                .size(display.soa_heat_bytes_rt);
            device.device_extrl.cmd_pipeline_barrier(
                cmd,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::DependencyFlags::empty(),
                &[],
                std::slice::from_ref(&fill_bar),
                &[],
            );
        }

        device.device_extrl.cmd_bind_pipeline(
            cmd,
            vk::PipelineBindPoint::COMPUTE,
            comp.pipeline_extrl,
        );
        device.device_extrl.cmd_bind_descriptor_sets(
            cmd,
            vk::PipelineBindPoint::COMPUTE,
            comp.pipeline_layout_extrl,
            0,
            &[set],
            &[],
        );
        let pc = SoaRayPushRt {
            inv_mvp: mat4_inverse(&push.mvp),
            cam_pos: push.cam_pos,
            light_dir: push.light_dir,
            base_color: push.base_color,
            look3: push.look3,
            grid: [nx, ny, nz, n],
            view: [src_w as f32, src_h as f32, pitch, 150.0],
            brush: [
                display.heat_mouse_x_rt,
                display.heat_mouse_y_rt,
                display.heat_dt_rt.max(1e-4),
                display.heat_paint_rt as f32,
            ],
        };
        let pc_bytes = std::slice::from_raw_parts(
            std::ptr::from_ref(&pc).cast::<u8>(),
            size_of::<SoaRayPushRt>(),
        );
        device.device_extrl.cmd_push_constants(
            cmd,
            comp.pipeline_layout_extrl,
            vk::ShaderStageFlags::COMPUTE,
            0,
            pc_bytes,
        );
        let gx = src_w.div_ceil(8).max(1);
        let gy = src_h.div_ceil(8).max(1);
        device.device_extrl.cmd_dispatch(cmd, gx, gy, 1);

        let (b1, s1, d1) = image_barrier(
            color,
            vk::PipelineStageFlags::COMPUTE_SHADER,
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::SHADER_WRITE,
            vk::AccessFlags::TRANSFER_READ,
            vk::ImageLayout::GENERAL,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
        );
        let (b2, _, _) = image_barrier(
            swap_image,
            vk::PipelineStageFlags::TOP_OF_PIPE,
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::empty(),
            vk::AccessFlags::TRANSFER_WRITE,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        );
        device.device_extrl.cmd_pipeline_barrier(
            cmd,
            s1,
            d1,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &[b1, b2],
        );

        let sub = vk::ImageSubresourceLayers {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            mip_level: 0,
            base_array_layer: 0,
            layer_count: 1,
        };
        let blit = vk::ImageBlit::default()
            .src_subresource(sub)
            .src_offsets([
                vk::Offset3D { x: 0, y: 0, z: 0 },
                vk::Offset3D {
                    x: src_w as i32,
                    y: src_h as i32,
                    z: 1,
                },
            ])
            .dst_subresource(sub)
            .dst_offsets([
                vk::Offset3D { x: 0, y: 0, z: 0 },
                vk::Offset3D {
                    x: extent.width as i32,
                    y: extent.height as i32,
                    z: 1,
                },
            ]);
        device.device_extrl.cmd_blit_image(
            cmd,
            color,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            swap_image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            std::slice::from_ref(&blit),
            vk::Filter::NEAREST,
        );

        let (b3, s3, d3) = image_barrier(
            swap_image,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::BOTTOM_OF_PIPE,
            vk::AccessFlags::TRANSFER_WRITE,
            vk::AccessFlags::empty(),
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::PRESENT_SRC_KHR,
        );
        device.device_extrl.cmd_pipeline_barrier(
            cmd,
            s3,
            d3,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            std::slice::from_ref(&b3),
        );

        map_vk(device.device_extrl.end_command_buffer(cmd))?;
    }
    Ok(())
}
