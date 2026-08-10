//! `TandemBfr` — hub after PTP boot (direct cubes · no Viewsor).

use std::time::Instant;

use modul::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayDefaultRtCrg;
use modul::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::FrameDefaultRtCrg;
use modul::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::line_gpu_default_rt_pkg::LineGpuDefaultRtPkg;
use modul::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::{
    CadSteelPushRt, MeshGpuDefaultRtPkg,
};
use modul::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::RendererDefaultRtCrg;
use modul::gpu::MODUL0_VK_SWAPCHAIN::conv::port::SwapchainBfr;
use modul::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::PresentationDefaultRtCrg;

/// Product hub after `assemble_tandem_session` (direct draw path only).
pub struct TandemBfr {
    pub swapchain_bfr: SwapchainBfr,
    pub renderer_rt: RendererDefaultRtCrg,
    pub presentation_rt: PresentationDefaultRtCrg,
    pub frame_rt: FrameDefaultRtCrg,
    pub display_rt: DisplayDefaultRtCrg,
    pub mesh_gpu_rt: MeshGpuDefaultRtPkg,
    pub grid_line_rt: LineGpuDefaultRtPkg,
    pub steel_push_rt: CadSteelPushRt,
    pub orbit_yaw: f32,
    pub orbit_pitch: f32,
    pub zoom: f32,
    pub dragging: bool,
    pub last_cursor: Option<(f64, f64)>,
    pub fps: f32,
    pub fps_instant: f32,
    pub fps_sample_ready: bool,
    pub(crate) fps_frames: u32,
    pub(crate) fps_window_start: Instant,
    pub(crate) last_frame_end: Instant,
    pub pulse_t0: Instant,
}
