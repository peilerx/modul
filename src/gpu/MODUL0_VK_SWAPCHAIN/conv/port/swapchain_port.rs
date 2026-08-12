//! Swapchain subject port — **factory-line order only**.
//!
//! *Bfr type · `embedded/buffer/` · slots · `vk_bfr/auto/…_at_asm` · `import_for_asm8` · …

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_bfr::auto::swapchain_bfr_at_asm::SwapchainBfrAuto;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::buffer::SwapchainBfr;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_crg::handled::swapchain_rt_crg_hld_asm::SwapchainRtCrgHandled;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_pkg::auto::command_pool_at_asm::CommandPoolDefaultAuto;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_pkg::auto::device_at_asm::DeviceDefaultAuto;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_pkg::auto::entry_at_asm::EntryDefaultAuto;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_pkg::auto::physical_device_at_asm::PhysicalDeviceDefaultAuto;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_pkg::auto::surface_at_asm::SurfaceDefaultAuto;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_pkg::auto::swapchain_loader_at_asm::SwapchainLoaderDefaultAuto;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_pkg::handled::instance_hld_asm::InstanceDefaultHandled;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_pkg::handled::swapchain_hld_asm::SwapchainDefaultHandled;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::EntryDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::InstanceDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::prt::swapchain_assembly_prt::SwapchainAssemblyPrt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::prt::swapchain_prt::SwapchainPrt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::DeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::PhysicalDeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SurfaceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SwapchainCommandPoolDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SwapchainLoaderDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SwapchainRtCrg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SwapchainDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::setup::boot_res_intsct_stp_pkgs::SurfaceWindowStpPkg;
use crate::ModulResult;

/// Boot `import_for_asm8`: **8** assemblies · 7 atom + 1 cargo pack.
pub const IMPORT_FOR_ASM_FACTORY_LINE_N: u8 = 8;

/// Present `import_present_for_asm1`: **1** assembly · KHR *`RtPkg`.
pub const IMPORT_PRESENT_FOR_ASM_FACTORY_LINE_N: u8 = 1;

/// `SwapchainTransportable` — trait (swapchain transportable).
///
/// Transportable surface: import/export peels for PTP slot-factory-line wiring.
/// Belongs to: swapchain / device bootstrap MCG.
/// Module path context: `gpu/MODUL0_VK_SWAPCHAIN/conv/port`.
pub trait SwapchainTransportable {
    /// Boot factory-line · **8** = asm 1..7 atom · asm 8 pack.
    fn import_for_asm8(
        bfr: &mut Self,
        assembly_intent: SwapchainAssemblyPrt,
        surface_window_stp_pkg: SurfaceWindowStpPkg,
    ) -> ModulResult<()>;

    /// Present factory-line · **1** = KHR product.
    fn import_present_for_asm1(
        bfr: &mut Self,
        present_intent: SwapchainPrt,
        extent_width_stp: u32,
        extent_height_stp: u32,
    ) -> ModulResult<()>;

    /// Export asmed boot cargo · **1** product.
    fn export_asmed1(bfr: &Self) -> Option<&SwapchainRtCrg>;

    /// Export asmed KHR package · **1** product.
    fn export_asmed_swapchain1(bfr: &Self) -> Option<&SwapchainDefaultRtPkg>;
}

impl SwapchainTransportable for SwapchainBfr {
    fn import_for_asm8(
        bfr: &mut Self,
        assembly_intent: SwapchainAssemblyPrt,
        surface_window_stp_pkg: SurfaceWindowStpPkg,
    ) -> ModulResult<()> {
        debug_assert_eq!(IMPORT_FOR_ASM_FACTORY_LINE_N, 8);

        bfr.surface_window_stp_pkg = Some(surface_window_stp_pkg);

        let validation_layers_stp = match assembly_intent {
            SwapchainAssemblyPrt::GraphicsPresentValidation => true,
            SwapchainAssemblyPrt::GraphicsPresentNoValidation => false,
        };

        // asm 1/8 · atom · entry
        bfr.entry_default_rt = Some(EntryDefaultRt::auto_assemble()?);

        // asm 2/8 · atom · instance
        bfr.instance_default_rt = Some(InstanceDefaultRt::handled_assemble(
            bfr.entry()?,
            validation_layers_stp,
            bfr.surface_window()?.display_handle_extrl,
        )?);

        // asm 3/8 · atom · surface package
        bfr.surface_default_rt_pkg = Some(SurfaceDefaultRtPkg::auto_assemble(
            bfr.entry()?,
            bfr.instance()?,
            bfr.surface_window()?.display_handle_extrl,
            bfr.surface_window()?.window_handle_extrl,
        )?);

        // asm 4/8 · atom · physical device
        bfr.physical_device_default_rt_pkg = Some(PhysicalDeviceDefaultRtPkg::auto_assemble(
            bfr.instance()?,
            bfr.surface_pkg()?,
        )?);

        // asm 5/8 · atom · logical device
        bfr.device_default_rt_pkg = Some(DeviceDefaultRtPkg::auto_assemble(
            bfr.physical_device()?,
            bfr.instance()?,
        )?);

        // asm 6/8 · atom · command pool
        bfr.swapchain_command_pool_default_rt_pkg =
            Some(SwapchainCommandPoolDefaultRtPkg::auto_assemble(
                bfr.device()?,
                bfr.physical_device()?,
            )?);

        // asm 7/8 · atom · swapchain loader
        bfr.swapchain_loader_default_rt_pkg = Some(SwapchainLoaderDefaultRtPkg::auto_assemble(
            bfr.instance()?,
            bfr.device()?,
        ));

        // asm 8/8 · pack cargo from bfr slots (takes inside handled · ¬ let theater in port)
        let cargo_rt = SwapchainRtCrg::handled_assemble(bfr, assembly_intent)?;
        bfr.cargo_rt = Some(cargo_rt);

        Ok(())
    }

    fn import_present_for_asm1(
        bfr: &mut Self,
        present_intent: SwapchainPrt,
        extent_width_stp: u32,
        extent_height_stp: u32,
    ) -> ModulResult<()> {
        debug_assert_eq!(IMPORT_PRESENT_FOR_ASM_FACTORY_LINE_N, 1);

        let cargo = bfr.cargo()?;

        // asm 1/1 · KHR swapchain product
        bfr.swapchain_default_rt_pkg = Some(SwapchainDefaultRtPkg::handled_assemble(
            &cargo.surface_default_rt_pkg,
            &cargo.physical_device_default_rt_pkg,
            &cargo.swapchain_loader_default_rt_pkg,
            present_intent,
            extent_width_stp,
            extent_height_stp,
        )?);
        Ok(())
    }

    fn export_asmed1(bfr: &Self) -> Option<&SwapchainRtCrg> {
        bfr.cargo_rt.as_ref()
    }

    fn export_asmed_swapchain1(bfr: &Self) -> Option<&SwapchainDefaultRtPkg> {
        bfr.swapchain_default_rt_pkg.as_ref()
    }
}
