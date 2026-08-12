use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::InstanceDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::PhysicalDeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SurfaceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk::auto::physical_device_at_asm::PhysicalDeviceSelectAuto;
use crate::ModulResult;

/// Catalog — pack selected physical device + queue family (FIX-120).
pub trait PhysicalDeviceDefaultAuto {
    fn auto_assemble(
        instance_default_rt: &InstanceDefaultRt,
        surface_default_rt_pkg: &SurfaceDefaultRtPkg,
    ) -> ModulResult<PhysicalDeviceDefaultRtPkg>;
}

impl PhysicalDeviceDefaultAuto for PhysicalDeviceDefaultRtPkg {
    fn auto_assemble(
        instance_default_rt: &InstanceDefaultRt,
        surface_default_rt_pkg: &SurfaceDefaultRtPkg,
    ) -> ModulResult<PhysicalDeviceDefaultRtPkg> {
        let (physical_device_extrl, queue_family_index_stp) =
            <(ash::vk::PhysicalDevice, u32)>::auto_assemble(
                &instance_default_rt.instance_extrl,
                &surface_default_rt_pkg.surface_loader_extrl,
                surface_default_rt_pkg.surface_extrl,
            )?;
        Ok(Self {
            physical_device_extrl,
            queue_family_index_rt: queue_family_index_stp,
            desc: "vulkan_physical_device",
        })
    }
}
