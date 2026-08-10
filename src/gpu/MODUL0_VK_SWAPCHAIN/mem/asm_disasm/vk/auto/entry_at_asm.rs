//! vk resource: `ash::Entry`.

/// Catalog — linked Vulkan entry loader.
pub trait EntryAuto {
    fn auto_assemble() -> Self;
}

impl EntryAuto for ash::Entry {
    fn auto_assemble() -> Self {
        ash::Entry::linked()
    }
}
