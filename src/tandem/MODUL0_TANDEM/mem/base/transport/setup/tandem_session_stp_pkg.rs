//! Full session knobs (flexible path · override aggregate Prt).

use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::prt::frame_fif_prt::FrameFifPrt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::prt::swapchain_prt::SwapchainPrt;
use crate::tandem::MODUL0_TANDEM::mem::base::transport::prt::sample_count_prefer_prt::SampleCountPreferPrt;
use crate::tandem::MODUL0_TANDEM::mem::base::transport::prt::validation_prefer_prt::ValidationPreferPrt;

/// Explicit session setup bag — all knobs apps may override after Prt expand.
#[derive(Debug, Clone)]
pub struct TandemSessionStpPkg {
    /// Validation prefer op.
    pub validation_prefer_op: ValidationPreferPrt,
    /// Present mode + format flavor.
    pub present_prt_op: SwapchainPrt,
    /// MSAA preference (resolved against device caps).
    pub sample_prefer_op: SampleCountPreferPrt,
    /// Frames-in-flight intent.
    pub frame_fif_prt_op: FrameFifPrt,
    /// Instance lattice size (cubes).
    pub cube_count_stp: usize,
    /// Clear color RGBA.
    pub clear_color_rt: [f32; 4],
    /// Pulse period seconds (shader look3.w).
    pub pulse_period_secs_stp: f32,
    /// Max lattice separation (shader look3.y).
    pub sep_max_stp: f32,
    /// Descriptor tag.
    pub desc: &'static str,
}

impl Default for TandemSessionStpPkg {
    fn default() -> Self {
        crate::tandem::MODUL0_TANDEM::mem::base::transport::prt::TandemSessionPrt::default()
            .to_session_stp()
    }
}

impl TandemSessionStpPkg {
    /// Apply ship env overrides (`CUBES_COUNT`, `CUBES_VALIDATION`) onto knobs.
    #[must_use]
    pub fn with_ship_env(mut self) -> Self {
        if let Ok(s) = std::env::var("CUBES_COUNT") {
            if let Ok(n) = s.parse::<usize>() {
                self.cube_count_stp = n.max(1);
            }
        }
        if std::env::var("CUBES_VALIDATION")
            .is_ok_and(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
        {
            self.validation_prefer_op = ValidationPreferPrt::PreferValidation;
        }
        self
    }

    /// Merge: `override` fields win when using a pattern of full replace of selected knobs.
    #[must_use]
    pub fn merge_override(mut self, o: &Self) -> Self {
        self.validation_prefer_op = o.validation_prefer_op;
        self.present_prt_op = o.present_prt_op;
        self.sample_prefer_op = o.sample_prefer_op;
        self.frame_fif_prt_op = o.frame_fif_prt_op;
        self.cube_count_stp = o.cube_count_stp.max(1);
        self.clear_color_rt = o.clear_color_rt;
        self.pulse_period_secs_stp = o.pulse_period_secs_stp;
        self.sep_max_stp = o.sep_max_stp;
        self.desc = o.desc;
        self
    }
}
