//! Base **frame** resource intersection (M.BASE_RES_INTSCT · N.RES_INTSCT · FIX-120).
//! All *Pkg/*Crg that field each other — **this file only**.

/// fences · semaphores · command buffers · FIF index.
pub struct FrameSyncDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `image_available_semaphores_extrl` (`image_available_semaphores` peel).
    pub image_available_semaphores_extrl: Vec<ash::vk::Semaphore>,
    /// External / raw Vulkan handle or host pointer field `render_finished_semaphores_extrl` (`render_finished_semaphores` peel).
    pub render_finished_semaphores_extrl: Vec<ash::vk::Semaphore>,
    /// External / raw Vulkan handle or host pointer field `in_flight_fences_extrl` (`in_flight_fences` peel).
    pub in_flight_fences_extrl: Vec<ash::vk::Fence>,
    /// External / raw Vulkan handle or host pointer field `command_buffers_extrl` (`command_buffers` peel).
    pub command_buffers_extrl: Vec<ash::vk::CommandBuffer>,
    /// Runtime phase field `current_frame_rt`.
    pub current_frame_rt: usize,
    /// Runtime phase field `frames_in_flight_rt`.
    pub frames_in_flight_rt: usize,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// camera / clear cache for record policy.
pub struct FrameRenderDefaultRtPkg {
    /// Runtime phase field `clear_color_rt`.
    pub clear_color_rt: [f32; 4],
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// One FIF slot handles for record / submit / present.
#[derive(Debug, Clone, Copy)]
pub struct FrameSlotDefaultRtPkg {
    /// Runtime phase field `slot_rt`.
    pub slot_rt: usize,
    /// External / raw Vulkan handle or host pointer field `image_available_semaphore_extrl` (`image_available_semaphore` peel).
    pub image_available_semaphore_extrl: ash::vk::Semaphore,
    /// External / raw Vulkan handle or host pointer field `render_finished_semaphore_extrl` (`render_finished_semaphore` peel).
    pub render_finished_semaphore_extrl: ash::vk::Semaphore,
    /// External / raw Vulkan handle or host pointer field `in_flight_fence_extrl` (`in_flight_fence` peel).
    pub in_flight_fence_extrl: ash::vk::Fence,
    /// External / raw Vulkan handle or host pointer field `command_buffer_extrl` (`command_buffer` peel).
    pub command_buffer_extrl: ash::vk::CommandBuffer,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Conv-assembled frame session export.
pub struct FrameDefaultRtCrg {
    /// Nested package bag field `frame_sync_default_rt_pkg`.
    pub frame_sync_default_rt_pkg: FrameSyncDefaultRtPkg,
    /// Nested package bag field `frame_render_default_rt_pkg`.
    pub frame_render_default_rt_pkg: FrameRenderDefaultRtPkg,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}
