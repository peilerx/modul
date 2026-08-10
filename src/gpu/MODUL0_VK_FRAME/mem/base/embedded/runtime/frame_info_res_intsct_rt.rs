//! Embedded **frame_info** resource intersection (M.BASE_RES_INTSCT · FIX-120).
//! Ephemeral begin/submit/present/rp-begin bags · co-used on frame tick.

/// ephemeral begin
pub struct FrameCommandBeginInfoDefaultRt {
    /// Operator / knob field `buffer_usage_flags_op`.
    pub buffer_usage_flags_op: ash::vk::CommandBufferUsageFlags,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// ephemeral present
pub struct PresentInfoDefaultRt {
    /// External / raw Vulkan handle or host pointer field `wait_semaphore_extrl` (`wait_semaphore` peel).
    pub wait_semaphore_extrl: ash::vk::Semaphore,
    /// External / raw Vulkan handle or host pointer field `swapchain_extrl` (`swapchain` peel).
    pub swapchain_extrl: ash::vk::SwapchainKHR,
    /// Runtime phase field `image_index_rt`.
    pub image_index_rt: u32,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// ephemeral submit
pub struct SubmitInfoDefaultRt {
    /// External / raw Vulkan handle or host pointer field `wait_semaphore_extrl` (`wait_semaphore` peel).
    pub wait_semaphore_extrl: ash::vk::Semaphore,
    /// Operator / knob field `wait_dst_stage_mask_op`.
    pub wait_dst_stage_mask_op: Vec<ash::vk::PipelineStageFlags>,
    /// External / raw Vulkan handle or host pointer field `command_buffer_extrl` (`command_buffer` peel).
    pub command_buffer_extrl: ash::vk::CommandBuffer,
    /// External / raw Vulkan handle or host pointer field `signal_semaphore_extrl` (`signal_semaphore` peel).
    pub signal_semaphore_extrl: ash::vk::Semaphore,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Ephemeral triangle render-pass begin — runtime bag (FIX-085/089).
pub struct RenderPassBeginInfoTriangleRt {
    /// External / raw Vulkan handle or host pointer field `render_pass_extrl` (`render_pass` peel).
    pub render_pass_extrl: ash::vk::RenderPass,
    /// External / raw Vulkan handle or host pointer field `framebuffer_extrl` (`framebuffer` peel).
    pub framebuffer_extrl: ash::vk::Framebuffer,
    /// Runtime phase field `extent_rt`.
    pub extent_rt: ash::vk::Extent2D,
    /// Runtime phase field `clear_values_rt`.
    pub clear_values_rt: [ash::vk::ClearValue; 2],
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}
