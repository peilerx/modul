//! High-level session pictures (aggregate child MCG intents).

use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::prt::frame_fif_prt::FrameFifPrt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::prt::{
    SwapchainAssemblyPrt, SwapchainPrt,
};
use crate::tandem::MODUL0_TANDEM::mem::base::transport::prt::sample_count_prefer_prt::SampleCountPreferPrt;
use crate::tandem::MODUL0_TANDEM::mem::base::transport::prt::validation_prefer_prt::ValidationPreferPrt;
use crate::tandem::MODUL0_TANDEM::mem::base::transport::setup::tandem_session_stp_pkg::TandemSessionStpPkg;

/// Closed gestalt arms for the cubes/product tandem session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TandemSessionPrt {
    /// Telegram ship: MAILBOX · prefer AA4 · no validation · double FIF · 1M cubes.
    #[default]
    ShipMailboxAa4NoValidation,
    /// Same as ship but prefer Khronos validation (debug ship).
    ShipMailboxAa4PreferValidation,
    /// Dev: FIFO vsync · force 1× · validation · double FIF.
    DevFifoAa1PreferValidation,
    /// Benchmark: MAILBOX · force 1× · no validation.
    BenchmarkMailboxAa1NoValidation,
    /// Low-end: FIFO · force 1× · no validation.
    LowEndFifoAa1NoValidation,
}

impl TandemSessionPrt {
    /// Expand aggregate arm into full session setup knobs (then app may override Stp fields).
    #[must_use]
    pub const fn to_session_stp(self) -> TandemSessionStpPkg {
        match self {
            Self::ShipMailboxAa4NoValidation => TandemSessionStpPkg {
                validation_prefer_op: ValidationPreferPrt::NoValidation,
                present_prt_op: SwapchainPrt::SrgbMailbox,
                sample_prefer_op: SampleCountPreferPrt::Prefer4Else1,
                frame_fif_prt_op: FrameFifPrt::DoubleBuffered,
                cube_count_stp: 1_000_000,
                clear_color_rt: [0.05, 0.05, 0.08, 1.0],
                pulse_period_secs_stp: 12.0,
                sep_max_stp: 1.6,
                desc: "tandem_session_ship_mailbox_aa4_no_validation",
            },
            Self::ShipMailboxAa4PreferValidation => TandemSessionStpPkg {
                validation_prefer_op: ValidationPreferPrt::PreferValidation,
                present_prt_op: SwapchainPrt::SrgbMailbox,
                sample_prefer_op: SampleCountPreferPrt::Prefer4Else1,
                frame_fif_prt_op: FrameFifPrt::DoubleBuffered,
                cube_count_stp: 1_000_000,
                clear_color_rt: [0.05, 0.05, 0.08, 1.0],
                pulse_period_secs_stp: 12.0,
                sep_max_stp: 1.6,
                desc: "tandem_session_ship_mailbox_aa4_prefer_validation",
            },
            Self::DevFifoAa1PreferValidation => TandemSessionStpPkg {
                validation_prefer_op: ValidationPreferPrt::PreferValidation,
                present_prt_op: SwapchainPrt::SrgbFifo,
                sample_prefer_op: SampleCountPreferPrt::Force1,
                frame_fif_prt_op: FrameFifPrt::DoubleBuffered,
                cube_count_stp: 100_000,
                clear_color_rt: [0.05, 0.05, 0.08, 1.0],
                pulse_period_secs_stp: 12.0,
                sep_max_stp: 1.6,
                desc: "tandem_session_dev_fifo_aa1_prefer_validation",
            },
            Self::BenchmarkMailboxAa1NoValidation => TandemSessionStpPkg {
                validation_prefer_op: ValidationPreferPrt::NoValidation,
                present_prt_op: SwapchainPrt::SrgbMailbox,
                sample_prefer_op: SampleCountPreferPrt::Force1,
                frame_fif_prt_op: FrameFifPrt::DoubleBuffered,
                cube_count_stp: 1_000_000,
                clear_color_rt: [0.02, 0.02, 0.03, 1.0],
                pulse_period_secs_stp: 12.0,
                sep_max_stp: 1.6,
                desc: "tandem_session_benchmark_mailbox_aa1_no_validation",
            },
            Self::LowEndFifoAa1NoValidation => TandemSessionStpPkg {
                validation_prefer_op: ValidationPreferPrt::NoValidation,
                present_prt_op: SwapchainPrt::SrgbFifo,
                sample_prefer_op: SampleCountPreferPrt::Force1,
                frame_fif_prt_op: FrameFifPrt::DoubleBuffered,
                cube_count_stp: 50_000,
                clear_color_rt: [0.05, 0.05, 0.08, 1.0],
                pulse_period_secs_stp: 12.0,
                sep_max_stp: 1.6,
                desc: "tandem_session_low_end_fifo_aa1_no_validation",
            },
        }
    }

    /// Map arm to swapchain assembly intent.
    #[must_use]
    pub const fn swapchain_assembly_prt(self) -> SwapchainAssemblyPrt {
        match self {
            Self::ShipMailboxAa4PreferValidation | Self::DevFifoAa1PreferValidation => {
                SwapchainAssemblyPrt::GraphicsPresentValidation
            }
            Self::ShipMailboxAa4NoValidation
            | Self::BenchmarkMailboxAa1NoValidation
            | Self::LowEndFifoAa1NoValidation => SwapchainAssemblyPrt::GraphicsPresentNoValidation,
        }
    }
}
