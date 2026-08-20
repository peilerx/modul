//! `TandemBfr` — product warehouse after PTP session boot.

use std::time::Instant;

use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayDefaultRtCrg;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::FrameDefaultRtCrg;
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::{
    MeshGpuDefaultRtPkg, MeshPushRt,
};
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::RendererDefaultRtCrg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::conv::port::SwapchainBfr;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::PresentationDefaultRtCrg;
use crate::tandem::MODUL0_TANDEM::mem::base::transport::setup::TandemSessionStpPkg;

/// Product hub peels after session assemble (instanced solid cubes path).
pub struct TandemBfr {
    pub swapchain_bfr: SwapchainBfr,
    pub renderer_rt: RendererDefaultRtCrg,
    pub presentation_rt: PresentationDefaultRtCrg,
    pub frame_rt: FrameDefaultRtCrg,
    pub display_rt: DisplayDefaultRtCrg,
    pub mesh_gpu_rt: MeshGpuDefaultRtPkg,
    pub mesh_push_rt: MeshPushRt,
    /// Session knobs used at boot (pulse / clear / sep read these).
    pub session_stp: TandemSessionStpPkg,
    pub orbit_yaw: f32,
    pub orbit_pitch: f32,
    pub zoom: f32,
    pub dragging: bool,
    pub heat_painting: bool,
    pub last_cursor: Option<(f64, f64)>,
    pub cursor_px: (f32, f32),
    pub heat_hold_rt: f32,
    pub heat_decay_tail_rt: f32,
    pub fps: f32,
    pub fps_instant: f32,
    pub fps_sample_ready: bool,
    pub fps_frames: u32,
    pub fps_window_start: Instant,
    pub last_frame_end: Instant,
    pub pulse_t0: Instant,
    pub heat_diag_dumped: bool,
}
