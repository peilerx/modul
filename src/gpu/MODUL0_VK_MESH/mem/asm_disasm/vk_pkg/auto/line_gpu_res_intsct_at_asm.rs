//! `vk_pkg` **auto** — `LineGpuDefaultRtPkg` list + tris (`N.RES_INTSCT`).

use ash::vk;
use ash::{Device, Instance};

use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::line_gpu_default_rt_pkg::LineGpuDefaultRtPkg;
use crate::gpu::MODUL0_VK_MESH::mem::asm_disasm::vk::handled::buffer_hld_asm::{
    handled_upload_host_visible, BufferHostVisibleHandled,
};
use crate::gpu::MODUL0_VK_MESH::proc::processor::line_gpu_counts::{
    line_gpu_counts, line_gpu_min_floats,
};
use crate::gpu::MODUL0_VK_MESH::proc::processor::line_gpu_destroy::destroy_line_gpu_buffers;
use crate::gpu::MODUL0_VK_MESH::proc::processor::pack_line_lists::f32_pos_to_bytes;
use crate::ModulResult;

/// Catalog — host-visible line VBO (`LINE_LIST`).
pub trait LineGpuDefaultAuto {
    fn auto_assemble(
        device_extrl: &Device,
        instance_extrl: &Instance,
        physical_device_extrl: vk::PhysicalDevice,
        positions_extrl: &[f32],
        color_rt: [f32; 4],
    ) -> ModulResult<LineGpuDefaultRtPkg>;

    fn auto_disassemble(device_extrl: &Device, line_gpu_default_rt_pkg: &mut LineGpuDefaultRtPkg);
}

/// Catalog — host-visible thick-outline ribbons (`TRIANGLE_LIST`).
pub trait LineGpuTrisDefaultAuto {
    fn auto_assemble(
        device_extrl: &Device,
        instance_extrl: &Instance,
        physical_device_extrl: vk::PhysicalDevice,
        positions_extrl: &[f32],
        color_rt: [f32; 4],
    ) -> ModulResult<LineGpuDefaultRtPkg>;
}

impl LineGpuDefaultAuto for LineGpuDefaultRtPkg {
    fn auto_assemble(
        device_extrl: &Device,
        instance_extrl: &Instance,
        physical_device_extrl: vk::PhysicalDevice,
        positions_extrl: &[f32],
        color_rt: [f32; 4],
    ) -> ModulResult<LineGpuDefaultRtPkg> {
        let as_tris_rt = false;
        let min_stp = line_gpu_min_floats(as_tris_rt);
        let positions_len_stp = positions_extrl.len();
        match positions_len_stp {
            positions_len_stp if positions_len_stp < min_stp => {
                Ok(Self::empty("line_gpu_list"))
            }
            positions_len_stp => {
                let bytes_extrl = f32_pos_to_bytes(positions_extrl);
                let size_stp = bytes_extrl.len() as vk::DeviceSize;
                let (vertex_buffer_extrl, vertex_memory_extrl) =
                    <(vk::Buffer, vk::DeviceMemory) as BufferHostVisibleHandled>::handled_assemble(
                        device_extrl,
                        instance_extrl,
                        physical_device_extrl,
                        size_stp,
                        vk::BufferUsageFlags::VERTEX_BUFFER,
                    )?;
                handled_upload_host_visible(device_extrl, vertex_memory_extrl, &bytes_extrl)?;
                let (vertex_count_rt, line_count_rt) =
                    line_gpu_counts(positions_len_stp, as_tris_rt);
                Ok(Self {
                    vertex_buffer_extrl,
                    vertex_memory_extrl,
                    vertex_count_rt,
                    line_count_rt,
                    color_rt,
                    as_tris_rt,
                    ready_rt: true,
                    desc: "line_gpu_list",
                })
            }
        }
    }

    fn auto_disassemble(device_extrl: &Device, line_gpu_default_rt_pkg: &mut LineGpuDefaultRtPkg) {
        destroy_line_gpu_buffers(device_extrl, line_gpu_default_rt_pkg);
    }
}

impl LineGpuTrisDefaultAuto for LineGpuDefaultRtPkg {
    fn auto_assemble(
        device_extrl: &Device,
        instance_extrl: &Instance,
        physical_device_extrl: vk::PhysicalDevice,
        positions_extrl: &[f32],
        color_rt: [f32; 4],
    ) -> ModulResult<LineGpuDefaultRtPkg> {
        let as_tris_rt = true;
        let min_stp = line_gpu_min_floats(as_tris_rt);
        let positions_len_stp = positions_extrl.len();
        match positions_len_stp {
            positions_len_stp if positions_len_stp < min_stp => {
                Ok(Self::empty("line_gpu_tris"))
            }
            positions_len_stp => {
                let bytes_extrl = f32_pos_to_bytes(positions_extrl);
                let size_stp = bytes_extrl.len() as vk::DeviceSize;
                let (vertex_buffer_extrl, vertex_memory_extrl) =
                    <(vk::Buffer, vk::DeviceMemory) as BufferHostVisibleHandled>::handled_assemble(
                        device_extrl,
                        instance_extrl,
                        physical_device_extrl,
                        size_stp,
                        vk::BufferUsageFlags::VERTEX_BUFFER,
                    )?;
                handled_upload_host_visible(device_extrl, vertex_memory_extrl, &bytes_extrl)?;
                let (vertex_count_rt, line_count_rt) =
                    line_gpu_counts(positions_len_stp, as_tris_rt);
                Ok(Self {
                    vertex_buffer_extrl,
                    vertex_memory_extrl,
                    vertex_count_rt,
                    line_count_rt,
                    color_rt,
                    as_tris_rt,
                    ready_rt: true,
                    desc: "line_gpu_tris",
                })
            }
        }
    }
}
