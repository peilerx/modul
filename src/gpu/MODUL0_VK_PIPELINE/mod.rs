//! # `MODUL0_VK_PIPELINE` — render pass + graphics pipelines
//!
//! Builds the **renderer cargo**: render pass, shader modules, graphics pipelines.
//!
//! Product solid path embeds `shader/cubes.vert.spv` + `shader/cubes.frag.spv`
//! (steel / instanced unit cubes). Legacy triangle + line pipelines may still
//! assemble as optional fields on the renderer cargo.
//!
//! ## Layers
//!
//! - `mem` — pipeline bags, `asm_disasm` catalog
//! - `conv` — `RendererBfr` port / `RenderLanePrt` intents
//! - (no heavy `proc` leaf in the direct cut)
//!
//! ## App usage
//!
//! `RendererBfr::import_for_asm9(RenderLanePrt::TriangleSolidDepthAa4, …)` then
//! take `cargo_rt` for presentation + display.
pub mod conv;
/// Submodule `mem`.
/// Part of `gpu/MODUL0_VK_PIPELINE` under the mem/conv/proc MCG canon.
pub mod mem;
