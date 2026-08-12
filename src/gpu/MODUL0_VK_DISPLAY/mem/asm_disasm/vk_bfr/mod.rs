//! Assembler rank module `gpu/MODUL0_VK_DISPLAY/mem/asm_disasm/vk_bfr`.
//!
//! Auto|Handled create/disassemble traits for Vulkan resources (FIX-129).

pub mod auto;
pub mod handled;
pub use crate::gpu::MODUL0_VK_DISPLAY::mem::base::embedded::buffer::DisplayBfr;
pub use handled::DisplayBfrHandled;
