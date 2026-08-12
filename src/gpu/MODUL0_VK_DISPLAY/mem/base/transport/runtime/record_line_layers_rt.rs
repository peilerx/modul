//! Optional line layers for one display record pass (grid · sketch · outline).

use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::line_gpu_default_rt_pkg::LineGpuDefaultRtPkg;

/// Peels for optional line draws in one frame record.
#[derive(Clone, Copy)]
pub struct RecordLineLayersRt<'a> {
    /// Nested package bag field `grid_line_gpu_default_rt_pkg`.
    pub grid_line_gpu_default_rt_pkg: Option<&'a LineGpuDefaultRtPkg>,
    /// Nested package bag field `sketch_line_gpu_default_rt_pkg`.
    pub sketch_line_gpu_default_rt_pkg: Option<&'a LineGpuDefaultRtPkg>,
    /// Nested package bag field `outline_line_gpu_default_rt_pkg`.
    pub outline_line_gpu_default_rt_pkg: Option<&'a LineGpuDefaultRtPkg>,
}

/// Empty optional line layers peel.
pub const RECORD_LINE_LAYERS_EMPTY: RecordLineLayersRt<'static> = RecordLineLayersRt {
    grid_line_gpu_default_rt_pkg: None,
    sketch_line_gpu_default_rt_pkg: None,
    outline_line_gpu_default_rt_pkg: None,
};
