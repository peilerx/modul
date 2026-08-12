//! # `MODUL0_VK_DISPLAY` — record command buffers
//!
//! Records draws into command buffers for a frame: solid mesh (steel/cubes),
//! optional line pass, clear values, render pass begin/end.
//!
//! ## Layers
//!
//! - `mem` — display cargo
//! - `conv` — display present port
//! - `proc` — `record_frame` / display ops
//!
//! ## App usage
//!
//! `record_frame_with_serial(…, mesh_gpu, mesh_push, grid_line, …)` after
//! `begin_frame`.
pub mod conv;
/// Submodule `mem`.
/// Part of `gpu/MODUL0_VK_DISPLAY` under the mem/conv/proc MCG canon.
pub mod mem;
/// Submodule `proc`.
/// Part of `gpu/MODUL0_VK_DISPLAY` under the mem/conv/proc MCG canon.
pub mod proc;
