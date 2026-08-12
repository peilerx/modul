//! vk resource: `ash::Entry` (runtime load · ship-friendly).

use crate::ModulResult;

/// Catalog — load default Vulkan library (`libvulkan.so.1` / platform equivalent).
pub trait EntryAuto {
    /// Fallible load — missing loader is a soft error for ship builds.
    fn auto_assemble() -> ModulResult<Self>
    where
        Self: Sized;
}

impl EntryAuto for ash::Entry {
    fn auto_assemble() -> ModulResult<Self> {
        // SAFETY: ash documents `Entry::load` as the portable ship path when the
        // `loaded` feature is enabled; no Vulkan calls are issued until after success.
        unsafe { Self::load() }.map_err(|e| {
            format!(
                "Vulkan loader not found ({e}). Install a GPU driver with Vulkan support \
                 (e.g. mesa-vulkan-drivers or vendor NVIDIA/AMD package). \
                 Check with: vulkaninfo --summary"
            )
        })
    }
}
