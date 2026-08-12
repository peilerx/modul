//! PortMatch Auto · validation / sample prefer peels (base ¬ impl).

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::prt::SwapchainAssemblyPrt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::proc::processor::device_caps::SampleCountPrefer;
use crate::tandem::MODUL0_TANDEM::mem::base::transport::prt::sample_count_prefer_prt::SampleCountPreferPrt;
use crate::tandem::MODUL0_TANDEM::mem::base::transport::prt::validation_prefer_prt::ValidationPreferPrt;

/// `SampleCountPreferPrt` → device-caps prefer enum.
#[must_use]
#[inline]
pub const fn sample_count_prefer_from_prt(prt: SampleCountPreferPrt) -> SampleCountPrefer {
    match prt {
        SampleCountPreferPrt::PREFER_4_ELSE_1 => SampleCountPrefer::PREFER_4_ELSE_1,
        SampleCountPreferPrt::FORCE_1 => SampleCountPrefer::FORCE_1,
        SampleCountPreferPrt::PREFER_8_ELSE_4_ELSE_1 => SampleCountPrefer::PREFER_8_ELSE_4_ELSE_1,
    }
}

/// Setup bool for `SwapchainAssemblyDefaultStpPkg.validation_layers_stp`.
#[must_use]
#[inline]
pub const fn validation_layers_stp_from_prt(prt: ValidationPreferPrt) -> bool {
    matches!(prt, ValidationPreferPrt::PREFER_VALIDATION)
}

/// `ValidationPreferPrt` → swapchain assembly intent.
#[must_use]
#[inline]
pub const fn swapchain_assembly_from_validation_prt(
    prt: ValidationPreferPrt,
) -> SwapchainAssemblyPrt {
    match prt {
        ValidationPreferPrt::PREFER_VALIDATION => {
            SwapchainAssemblyPrt::GRAPHICS_PRESENT_VALIDATION
        }
        ValidationPreferPrt::NO_VALIDATION => {
            SwapchainAssemblyPrt::GRAPHICS_PRESENT_NO_VALIDATION
        }
    }
}
