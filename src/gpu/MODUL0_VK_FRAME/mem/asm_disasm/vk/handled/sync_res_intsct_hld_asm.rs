//! MCU **`sync_res_intsct`** — vk primitives for frame sync (`N.RES_INTSCT` · N.FREQ · FIX-120).
//! Semaphore · Fence · `CommandBuffer` allocate · impl for ash/vk · ¬ *Pkg.

use ash::vk;
use ash::Device;

use crate::{map_vk, ModulResult};

// ── Traits hot → cold ───────────────────────────────────────────────────────

/// Catalog — allocate primary command buffers from pool.
pub trait CommandBufferAllocateHandled {
    fn handled_assemble(
        device_extrl: &Device,
        command_pool_extrl: vk::CommandPool,
        frames_in_flight_stp: u32,
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

/// Catalog — create binary semaphore · pure extrl → Auto.
pub trait SemaphoreAuto {
    fn auto_assemble(device_extrl: &Device) -> ModulResult<Self>
    where
        Self: Sized;
}

/// Catalog — create fence (signaled) · pure extrl → Auto.
pub trait FenceSignaledAuto {
    fn auto_assemble(device_extrl: &Device) -> ModulResult<Self>
    where
        Self: Sized;
}

// ── Impls same order ────────────────────────────────────────────────────────

impl CommandBufferAllocateHandled for Vec<vk::CommandBuffer> {
    fn handled_assemble(
        device_extrl: &Device,
        command_pool_extrl: vk::CommandPool,
        frames_in_flight_stp: u32,
    ) -> ModulResult<Self> {
        let cmd_buf_alloc_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool_extrl)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(frames_in_flight_stp);
        map_vk(unsafe { device_extrl.allocate_command_buffers(&cmd_buf_alloc_info) })
    }
}

impl SemaphoreAuto for vk::Semaphore {
    fn auto_assemble(device_extrl: &Device) -> ModulResult<Self> {
        let semaphore_info = vk::SemaphoreCreateInfo::default();
        map_vk(unsafe { device_extrl.create_semaphore(&semaphore_info, None) })
    }
}

impl FenceSignaledAuto for vk::Fence {
    fn auto_assemble(device_extrl: &Device) -> ModulResult<Self> {
        let fence_info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);
        map_vk(unsafe { device_extrl.create_fence(&fence_info, None) })
    }
}
