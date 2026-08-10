//! # `MODUL0_VK_FRAME` — frames in flight
//!
//! Owns the **FIF** cadence: acquire next image, wait fences/semaphores,
//! submit, present handoff coordination used by display.
//!
//! ## Layers
//!
//! - `mem` — frame cargo bags
//! - `conv` — import FIF policy / export render policy peels
//! - `proc` — `begin_frame` / `end_frame` processors
//!
//! ## App usage
//!
//! ```ignore
//! let (slot, image_index) = begin_frame(device, &presentation, loader, &frame_rt)?;
//! // record…
//! end_frame(device, &presentation, loader, &mut frame_rt, &slot, image_index)?;
//! ```
pub mod conv;
/// Submodule `mem`.
/// Part of `gpu/MODUL0_VK_FRAME` under the mem/conv/proc MCG canon.
pub mod mem;
/// Submodule `proc`.
/// Part of `gpu/MODUL0_VK_FRAME` under the mem/conv/proc MCG canon.
pub mod proc;
