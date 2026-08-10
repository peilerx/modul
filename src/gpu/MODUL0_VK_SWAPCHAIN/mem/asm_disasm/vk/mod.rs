//! Rank **vk** — materialize ash / `vk::*` resources only (FIX-120).
//!
//! Pattern (per resource file):
//! ```text
//! pub trait {Resource}{Auto|Handled} {
//!     fn auto_assemble|handled_assemble(...) -> … Self;
//! }
//! impl {Resource}{Auto|Handled} for ash::… | vk::… { … }
//! ```
//! ¬ product = `*Pkg` / `*Crg` · ¬ impl create on bag types.

pub mod auto;
/// Submodule `handled`.
/// Part of `gpu/MODUL0_VK_SWAPCHAIN/mem/asm_disasm/vk` under the mem/conv/proc MCG canon.
pub mod handled;
