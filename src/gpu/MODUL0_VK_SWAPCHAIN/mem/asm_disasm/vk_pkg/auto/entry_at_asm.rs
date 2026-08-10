use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::EntryDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk::auto::entry_at_asm::EntryAuto;

/// Catalog — pack `ash::Entry` into embedded bag (FIX-120).
pub trait EntryDefaultAuto {
    fn auto_assemble() -> crate::ModulResult<EntryDefaultRt>;
}

impl EntryDefaultAuto for EntryDefaultRt {
    fn auto_assemble() -> crate::ModulResult<EntryDefaultRt> {
        Ok(EntryDefaultRt {
            entry_extrl: ash::Entry::auto_assemble(),
            desc: "vulkan_entry",
        })
    }
}
