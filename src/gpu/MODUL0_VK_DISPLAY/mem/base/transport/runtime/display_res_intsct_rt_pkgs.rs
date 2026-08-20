//! Base **display** resource intersection (`M.BASE_RES_INTSCT` · FIX-120).
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
    /// Compute color target · STORAGE + TRANSFER_SRC · no cmdDraw.
    pub soa_color_image_extrl: ash::vk::Image,
    /// Device memory of `soa_color_image_extrl`.
    pub soa_color_memory_extrl: ash::vk::DeviceMemory,
    /// Image view of `soa_color_image_extrl`.
    pub soa_color_view_extrl: ash::vk::ImageView,
    /// Compute target extent (swapchain × SSAA).
    pub soa_color_extent_rt: ash::vk::Extent2D,
    /// Heat SoA volume · `VkImage` TYPE_3D R32F · STORAGE + TRANSFER_DST.
    pub soa_heat_image_extrl: ash::vk::Image,
    /// Image view of `soa_heat_image_extrl` (`TYPE_3D`).
    pub soa_heat_view_extrl: ash::vk::ImageView,
    /// Device memory of `soa_heat_image_extrl`.
    pub soa_heat_memory_extrl: ash::vk::DeviceMemory,
    /// Lattice extent of the heat volume (nx³).
    pub soa_heat_extent_rt: ash::vk::Extent3D,
    /// Allocated bytes (driver image requirements, may exceed nx³·4).
    pub soa_heat_bytes_rt: u64,
    /// First-frame clear done.
    pub soa_heat_cleared_rt: bool,
    /// Mouse NDC x for heat brush.
    pub heat_mouse_x_rt: f32,
    /// Mouse NDC y for heat brush.
    pub heat_mouse_y_rt: f32,
    /// Frame dt seconds.
    pub heat_dt_rt: f32,
    /// LMB hold seconds.
    pub heat_hold_rt: f32,
    /// 1 = paint this frame.
    pub heat_paint_rt: u32,
    /// 1 = run heat compute this frame (paint or cooling tail).
    pub heat_run_rt: u32,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}
