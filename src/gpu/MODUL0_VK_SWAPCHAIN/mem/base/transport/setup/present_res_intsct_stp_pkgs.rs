//! Base **present** setup intersection (M.BASE_RES_INTSCT · FIX-120).
//! Swapchain + presentation recipes co-matched in assemble.

use ash::vk;

/// Swapchain recipe — setup knobs only (FIX-086/091/097).
pub struct SwapchainDefaultStpPkg {
    /// Setup phase field `extent_width_stp`.
    pub extent_width_stp: u32,
    /// Setup phase field `extent_height_stp`.
    pub extent_height_stp: u32,
    /// Operator / knob field `surface_format_op`.
    pub surface_format_op: vk::Format,
    /// Operator / knob field `present_mode_op`.
    pub present_mode_op: vk::PresentModeKHR,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Presentation lane recipe — setup knobs only (FIX-086/091/097).
pub struct PresentationDefaultStpPkg {
    /// Operator / knob field `sample_count_op`.
    pub sample_count_op: vk::SampleCountFlags,
    /// Operator / knob field `depth_format_op`.
    pub depth_format_op: vk::Format,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}
