//! vk_pkg **handled** — MeshGpuDefaultRtPkg catalog.

use ash::vk;
use ash::{Device, Instance};

use crate::gpu::MODUL0_VK_MESH::mem::base::transport::prt::mesh_draw_prt::MeshDrawPrt;
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::MeshGpuDefaultRtPkg;
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_soa_rt_bfr::MeshSoaRtBfr;
use crate::gpu::MODUL0_VK_MESH::mem::asm_disasm::vk::handled::buffer_hld_asm::{
    handled_upload_host_visible, BufferHostVisibleHandled,
};
use crate::cpu::MODUL0_MESH::proc::processor::{
    instance_count, pack_instance_xyzw_bytes, world_bounds_from_local,
};
use crate::gpu::MODUL0_VK_MESH::mem::asm_disasm::vk_pkg::auto::mesh_gpu_default_rt_pkg_at_asm::MeshGpuDefaultRtAuto;
use crate::gpu::MODUL0_VK_MESH::proc::processor::mesh_gpu_destroy::destroy_mesh_gpu_buffers;
use crate::gpu::MODUL0_VK_MESH::proc::processor::mesh_upload_prep::prepare_mesh_upload;
use crate::ModulResult;

/// Catalog — steel mesh VBO/IBO.
pub trait MeshGpuDefaultHandled {
    fn handled_assemble(
        device_extrl: &Device,
        instance_extrl: &Instance,
        physical_device_extrl: vk::PhysicalDevice,
        mesh_soa_rt_bfr: &MeshSoaRtBfr,
        mesh_draw_prt: MeshDrawPrt,
    ) -> ModulResult<MeshGpuDefaultRtPkg>;

    fn handled_disassemble(
        device_extrl: &Device,
        mesh_gpu_default_rt_pkg: &mut MeshGpuDefaultRtPkg,
    );
}

impl MeshGpuDefaultHandled for MeshGpuDefaultRtPkg {
    fn handled_assemble(
        device_extrl: &Device,
        instance_extrl: &Instance,
        physical_device_extrl: vk::PhysicalDevice,
        mesh_soa_rt_bfr: &MeshSoaRtBfr,
        mesh_draw_prt: MeshDrawPrt,
    ) -> ModulResult<MeshGpuDefaultRtPkg> {
        let prep = prepare_mesh_upload(mesh_soa_rt_bfr, mesh_draw_prt);
        let empty_stp = prep.empty_stp;
        match empty_stp {
            0 => {
                let size_vert_stp = prep.vert_bytes_extrl.len() as vk::DeviceSize;
                let (vertex_buffer_extrl, vertex_memory_extrl) =
                    <(vk::Buffer, vk::DeviceMemory) as BufferHostVisibleHandled>::handled_assemble(
                        device_extrl,
                        instance_extrl,
                        physical_device_extrl,
                        size_vert_stp,
                        vk::BufferUsageFlags::VERTEX_BUFFER,
                    )?;
                handled_upload_host_visible(
                    device_extrl,
                    vertex_memory_extrl,
                    &prep.vert_bytes_extrl,
                )?;
                let size_idx_stp = prep.idx_bytes_extrl.len() as vk::DeviceSize;
                let (index_buffer_extrl, index_memory_extrl) =
                    <(vk::Buffer, vk::DeviceMemory) as BufferHostVisibleHandled>::handled_assemble(
                        device_extrl,
                        instance_extrl,
                        physical_device_extrl,
                        size_idx_stp,
                        vk::BufferUsageFlags::INDEX_BUFFER,
                    )?;
                handled_upload_host_visible(
                    device_extrl,
                    index_memory_extrl,
                    &prep.idx_bytes_extrl,
                )?;
                let inst_bytes = pack_instance_xyzw_bytes(mesh_soa_rt_bfr);
                let instance_count_rt = instance_count(mesh_soa_rt_bfr) as u32;
                let size_inst_stp = inst_bytes.len() as vk::DeviceSize;
                let (instance_buffer_extrl, instance_memory_extrl) =
                    <(vk::Buffer, vk::DeviceMemory) as BufferHostVisibleHandled>::handled_assemble(
                        device_extrl,
                        instance_extrl,
                        physical_device_extrl,
                        size_inst_stp,
                        vk::BufferUsageFlags::VERTEX_BUFFER,
                    )?;
                handled_upload_host_visible(device_extrl, instance_memory_extrl, &inst_bytes)?;
                let (soa_world_buffer_extrl, soa_world_memory_extrl) =
                    (vk::Buffer::null(), vk::DeviceMemory::null());
                let (bounds_min_rt, bounds_max_rt) =
                    world_bounds_from_local(mesh_soa_rt_bfr, prep.bounds_min_rt, prep.bounds_max_rt);
                let triangle_count_rt = prep.triangle_count_rt.saturating_mul(instance_count_rt);
                crate::common::trace_emit(
                    "VK_MESH",
                    "mesh_gpu_upload",
                    &format!(
                        "verts={} idx={} inst={} mode={}",
                        prep.vertex_count_rt,
                        prep.index_count_rt,
                        instance_count_rt,
                        prep.mode_rt
                    ),
                );
                Ok(Self {
                    vertex_buffer_extrl,
                    index_buffer_extrl,
                    vertex_memory_extrl,
                    index_memory_extrl,
                    instance_buffer_extrl,
                    instance_memory_extrl,
                    soa_world_buffer_extrl,
                    soa_world_memory_extrl,
                    vertex_count_rt: prep.vertex_count_rt,
                    index_count_rt: prep.index_count_rt,
                    instance_count_rt,
                    instance_capacity_rt: instance_count_rt,
                    triangle_count_rt,
                    mode_rt: prep.mode_rt,
                    base_r_rt: 0.70,
                    base_g_rt: 0.725,
                    base_b_rt: 0.765,
                    bounds_min_rt,
                    bounds_max_rt,
                    ready_rt: true,
                    desc: "mesh_gpu_steel_solid",
                })
            }
            _ => Ok(<Self as MeshGpuDefaultRtAuto>::auto_assemble()),
        }
    }

    fn handled_disassemble(
        device_extrl: &Device,
        mesh_gpu_default_rt_pkg: &mut MeshGpuDefaultRtPkg,
    ) {
        destroy_mesh_gpu_buffers(device_extrl, mesh_gpu_default_rt_pkg);
    }
}
