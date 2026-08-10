//! Catalog — empty seed + **slot** accessors · all via trait `SwapchainBfrAuto`.
//! Type: `mem/base/embedded/buffer/swapchain_bfr.rs`.

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::buffer::SwapchainBfr;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::EntryDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::InstanceDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::DeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::PhysicalDeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SurfaceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SwapchainRtCrg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::setup::boot_res_intsct_stp_pkgs::SurfaceWindowStpPkg;
use crate::ModulResult;

/// Catalog Auto · empty seed + slot API on subject *Bfr.
pub trait SwapchainBfrAuto: Sized {
    fn auto_assemble() -> Self;

    fn slot_ref<'a, T>(slot: &'a Option<T>, name: &'static str) -> ModulResult<&'a T> {
        slot.as_ref()
            .ok_or_else(|| format!("swapchain_bfr: slot `{name}` empty"))
    }

    fn slot_take<T>(slot: &mut Option<T>, name: &'static str) -> ModulResult<T> {
        slot.take()
            .ok_or_else(|| format!("swapchain_bfr: slot `{name}` empty (take)"))
    }

    fn surface_window(&self) -> ModulResult<&SurfaceWindowStpPkg>;
    fn entry(&self) -> ModulResult<&EntryDefaultRt>;
    fn instance(&self) -> ModulResult<&InstanceDefaultRt>;
    fn surface_pkg(&self) -> ModulResult<&SurfaceDefaultRtPkg>;
    fn physical_device(&self) -> ModulResult<&PhysicalDeviceDefaultRtPkg>;
    fn device(&self) -> ModulResult<&DeviceDefaultRtPkg>;
    fn cargo(&self) -> ModulResult<&SwapchainRtCrg>;
}

impl SwapchainBfrAuto for SwapchainBfr {
    fn auto_assemble() -> Self {
        Self {
            surface_window_stp_pkg: None,
            entry_default_rt: None,
            instance_default_rt: None,
            surface_default_rt_pkg: None,
            physical_device_default_rt_pkg: None,
            device_default_rt_pkg: None,
            swapchain_command_pool_default_rt_pkg: None,
            swapchain_loader_default_rt_pkg: None,
            cargo_rt: None,
            swapchain_default_rt_pkg: None,
        }
    }

    fn surface_window(&self) -> ModulResult<&SurfaceWindowStpPkg> {
        Self::slot_ref(&self.surface_window_stp_pkg, "surface_window_stp_pkg")
    }

    fn entry(&self) -> ModulResult<&EntryDefaultRt> {
        Self::slot_ref(&self.entry_default_rt, "entry_default_rt")
    }

    fn instance(&self) -> ModulResult<&InstanceDefaultRt> {
        Self::slot_ref(&self.instance_default_rt, "instance_default_rt")
    }

    fn surface_pkg(&self) -> ModulResult<&SurfaceDefaultRtPkg> {
        Self::slot_ref(&self.surface_default_rt_pkg, "surface_default_rt_pkg")
    }

    fn physical_device(&self) -> ModulResult<&PhysicalDeviceDefaultRtPkg> {
        Self::slot_ref(
            &self.physical_device_default_rt_pkg,
            "physical_device_default_rt_pkg",
        )
    }

    fn device(&self) -> ModulResult<&DeviceDefaultRtPkg> {
        Self::slot_ref(&self.device_default_rt_pkg, "device_default_rt_pkg")
    }

    fn cargo(&self) -> ModulResult<&SwapchainRtCrg> {
        Self::slot_ref(&self.cargo_rt, "cargo_rt")
    }
}
