//! Full session knobs (flexible path · override aggregate Prt).
//!
//! Handled demos set every field explicitly. Expand/merge ∈ `asm_disasm` (base ¬ impl).

use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::prt::display_present_prt::DisplayPresentPrt;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::prt::frame_fif_prt::FrameFifPrt;
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::prt::mesh_draw_prt::MeshDrawPrt;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::prt::render_lane_prt::RenderLanePrt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::prt::swapchain_prt::SwapchainPrt;
use crate::tandem::MODUL0_TANDEM::mem::base::transport::prt::sample_count_prefer_prt::SampleCountPreferPrt;
use crate::tandem::MODUL0_TANDEM::mem::base::transport::prt::validation_prefer_prt::ValidationPreferPrt;

/// Explicit session setup bag — all knobs apps may set (Handled) or expand from Prt (Auto).
#[derive(Debug, Clone)]
pub struct TandemSessionStpPkg {
    /// Validation prefer op (`NO_VALIDATION` · `PREFER_VALIDATION`).
    pub validation_prefer_op: ValidationPreferPrt,
    /// Present mode + format flavor (`SRGB_MAILBOX` · `SRGB_FIFO` · `UNORM_MAILBOX`).
    pub present_prt_op: SwapchainPrt,
    /// MSAA preference (`PREFER_4_ELSE_1` · `FORCE_1` · `PREFER_8_ELSE_4_ELSE_1`).
    pub sample_prefer_op: SampleCountPreferPrt,
    /// Frames-in-flight (`DOUBLE_BUFFERED` · `TRIPLE_BUFFERED` · `SINGLE_BUFFERED`).
    pub frame_fif_prt_op: FrameFifPrt,
    /// Mesh draw path (`SOLID` · `TRIANGLE_LIST` · `WIREFRAME` · `DISABLED`).
    pub mesh_draw_prt_op: MeshDrawPrt,
    /// Display present path (`DEFAULT_PRESENT` · `CLEAR_COLOR_ONLY` · `RECORD_TRIANGLE`).
    pub display_present_prt_op: DisplayPresentPrt,
    /// Optional explicit render lane; `None` → derive from resolved sample count.
    pub render_lane_prt_op: Option<RenderLanePrt>,
    /// Instance lattice size (cubes).
    pub cube_count_stp: usize,
    /// Lattice spacing scale.
    pub lattice_spacing_stp: f32,
    /// Clear color RGBA.
    pub clear_color_rt: [f32; 4],
    /// Pulse period seconds (shader look3.w).
    pub pulse_period_secs_stp: f32,
    /// Max lattice separation (shader look3.y).
    pub sep_max_stp: f32,
    /// Orbit yaw start (radians).
    pub orbit_yaw_stp: f32,
    /// Orbit pitch start (radians).
    pub orbit_pitch_stp: f32,
    /// Zoom start.
    pub zoom_stp: f32,
    /// Camera distance scale vs mesh radius.
    pub camera_radius_scale_stp: f32,
    /// Descriptor tag.
    pub desc: &'static str,
}
