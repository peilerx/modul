//! Port — subject `*_port.rs` · factory-line order only.

pub mod presentation_port;
/// Submodule `swapchain_port`.
/// Part of `gpu/MODUL0_VK_SWAPCHAIN/conv/port` under the mem/conv/proc MCG canon.
pub mod swapchain_port;

pub use presentation_port::{
    PresentationTransportable, IMPORT_FOR_ASM_FACTORY_LINE_N as PRESENTATION_IMPORT_FOR_ASM_FACTORY_LINE_N,
};
pub use swapchain_port::{
    SwapchainTransportable, IMPORT_FOR_ASM_FACTORY_LINE_N, IMPORT_PRESENT_FOR_ASM_FACTORY_LINE_N,
};
pub use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::buffer::{
    PresentationBfr, SwapchainBfr,
};
pub use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_bfr::auto::presentation_bfr_at_asm::PresentationBfrAuto;
pub use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_bfr::auto::swapchain_bfr_at_asm::SwapchainBfrAuto;
