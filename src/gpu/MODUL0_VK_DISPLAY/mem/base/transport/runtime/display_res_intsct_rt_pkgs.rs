//! Base **display** resource intersection (M.BASE_RES_INTSCT · FIX-120).
//! Transport + former embedded display RTs that field the session crg — **one file**.

/// Default display input snapshot (runtime).
pub struct DisplayInputDefaultRtPkg {
    /// Runtime phase field `cursor_x_rt`.
    pub cursor_x_rt: f32,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// command state
pub struct DisplayCommandDefaultRt {
    /// Runtime phase field `recording_rt`.
    pub recording_rt: bool,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// render lane
pub struct DisplayRenderDefaultRt {
    /// Runtime phase field `frame_serial_rt`.
    pub frame_serial_rt: u64,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// display session
pub struct VulkanDisplayDefaultRt {
    /// Runtime phase field `ready_rt`.
    pub ready_rt: bool,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Conv-assembled display session export.
pub struct DisplayDefaultRtCrg {
    /// Nested package bag field `display_input_default_rt_pkg`.
    pub display_input_default_rt_pkg: DisplayInputDefaultRtPkg,
    /// Runtime phase field `command_rt`.
    pub command_rt: DisplayCommandDefaultRt,
    /// Runtime phase field `display_render_default_rt`.
    pub display_render_default_rt: DisplayRenderDefaultRt,
    /// Runtime phase field `vulkan_display_default_rt`.
    pub vulkan_display_default_rt: VulkanDisplayDefaultRt,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}
