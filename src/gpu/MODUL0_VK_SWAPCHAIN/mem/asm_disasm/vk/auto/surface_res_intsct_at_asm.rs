//! MCU **surface** — `vk::SurfaceKHR` + `khr::surface::Instance` (local only · FIX-120).

use ash::khr::surface;
use ash::vk;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use crate::{map_vk, ModulResult};

/// Catalog — window surface handle.
pub trait SurfaceKhrAuto {
    fn auto_assemble(
        entry_extrl: &ash::Entry,
        instance_extrl: &ash::Instance,
        display_handle_extrl: RawDisplayHandle,
        window_handle_extrl: RawWindowHandle,
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

/// Catalog — KHR surface loader.
pub trait SurfaceLoaderAuto {
    fn auto_assemble(entry_extrl: &ash::Entry, instance_extrl: &ash::Instance) -> Self;
}

impl SurfaceKhrAuto for vk::SurfaceKHR {
    fn auto_assemble(
        entry_extrl: &ash::Entry,
        instance_extrl: &ash::Instance,
        display_handle_extrl: RawDisplayHandle,
        window_handle_extrl: RawWindowHandle,
    ) -> ModulResult<Self> {
        map_vk(unsafe {
            ash_window::create_surface(
                entry_extrl,
                instance_extrl,
                display_handle_extrl,
                window_handle_extrl,
                None,
            )
        })
    }
}

impl SurfaceLoaderAuto for surface::Instance {
    fn auto_assemble(entry_extrl: &ash::Entry, instance_extrl: &ash::Instance) -> Self {
        Self::new(entry_extrl, instance_extrl)
    }
}
