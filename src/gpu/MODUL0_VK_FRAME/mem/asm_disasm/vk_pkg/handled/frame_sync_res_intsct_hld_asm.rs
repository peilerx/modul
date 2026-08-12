//! `vk_pkg` **`frame_sync`** — pack `FrameSyncDefaultRtPkg` via **vk** `sync_res_intsct` (FIX-120).

use ash::vk;
use ash::Device;

use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::FrameSyncDefaultRtPkg;
use crate::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk::handled::sync_res_intsct_hld_asm::{
    CommandBufferAllocateHandled, FenceSignaledAuto, SemaphoreAuto,
};
use crate::gpu::MODUL0_VK_FRAME::proc::processor::frame_sync::{
    update_frame_sync_semaphores, PRESENT_SIGNAL_SEMAPHORE_POOL,
};
use crate::ModulResult;

/// Catalog — pack FIF sync bag (hottest frame session API).
pub trait FrameSyncDefaultHandled {
    fn handled_assemble(
        device_extrl: &Device,
        command_pool_extrl: vk::CommandPool,
        frames_in_flight_stp: u32,
    ) -> ModulResult<FrameSyncDefaultRtPkg>;
}

impl FrameSyncDefaultHandled for FrameSyncDefaultRtPkg {
    fn handled_assemble(
        device_extrl: &Device,
        command_pool_extrl: vk::CommandPool,
        frames_in_flight_stp: u32,
    ) -> ModulResult<FrameSyncDefaultRtPkg> {
        let frames_in_flight_count_stp = frames_in_flight_stp as usize;
        let (image_available_semaphores_extrl, render_finished_semaphores_extrl, in_flight_fences_extrl) =
            update_frame_sync_semaphores(
                frames_in_flight_count_stp,
                PRESENT_SIGNAL_SEMAPHORE_POOL,
                || vk::Semaphore::auto_assemble(device_extrl),
                || vk::Fence::auto_assemble(device_extrl),
            )?;
        let command_buffers_extrl = <Vec<vk::CommandBuffer> as CommandBufferAllocateHandled>::handled_assemble(
            device_extrl,
            command_pool_extrl,
            frames_in_flight_stp,
        )?;

        Ok(Self {
            image_available_semaphores_extrl,
            render_finished_semaphores_extrl,
            in_flight_fences_extrl,
            command_buffers_extrl,
            current_frame_rt: 0,
            frames_in_flight_rt: frames_in_flight_count_stp,
            desc: "frame_sync",
        })
    }
}
