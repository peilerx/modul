//! vk_pkg depth — one import: **vk::image_res_intsct** MCU (`ImageResIntsctHandled`).

use ash::vk;

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::InstanceDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::DeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::PhysicalDeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::DepthImagesDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SampleCountDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SwapchainDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk::handled::image_res_intsct_hld_asm::ImageResIntsctHandled;
use crate::gpu::MODUL0_VK_SWAPCHAIN::proc::processor::repeat_lane::update_repeat_lanes;
use crate::ModulResult;

/// Catalog — pack depth image paths (format knob).
pub trait DepthImagesDefaultHandled {
    fn handled_assemble(
        instance_default_rt: &InstanceDefaultRt,
        physical_device_default_rt_pkg: &PhysicalDeviceDefaultRtPkg,
        device_default_rt_pkg: &DeviceDefaultRtPkg,
        swapchain_default_rt_pkg: &SwapchainDefaultRtPkg,
        sample_count_default_rt_pkg: &SampleCountDefaultRtPkg,
        depth_format_op: vk::Format,
    ) -> ModulResult<DepthImagesDefaultRtPkg>;
}

impl DepthImagesDefaultHandled for DepthImagesDefaultRtPkg {
    fn handled_assemble(
        instance_default_rt: &InstanceDefaultRt,
        physical_device_default_rt_pkg: &PhysicalDeviceDefaultRtPkg,
        device_default_rt_pkg: &DeviceDefaultRtPkg,
        swapchain_default_rt_pkg: &SwapchainDefaultRtPkg,
        sample_count_default_rt_pkg: &SampleCountDefaultRtPkg,
        depth_format_op: vk::Format,
    ) -> ModulResult<DepthImagesDefaultRtPkg> {
        match depth_format_op {
            depth_format_op => {
                let sample_count_op = sample_count_default_rt_pkg.sample_count_op;
                let extent_stp = swapchain_default_rt_pkg.extent_rt;
                let image_count_stp = swapchain_default_rt_pkg.images_extrl.len();
                let (images_extrl, device_memories_extrl, image_views_extrl) =
                    update_repeat_lanes(image_count_stp, || {
                        <(vk::Image, vk::DeviceMemory, vk::ImageView) as ImageResIntsctHandled>::handled_assemble(
                            &device_default_rt_pkg.device_extrl,
                            &instance_default_rt.instance_extrl,
                            physical_device_default_rt_pkg.physical_device_extrl,
                            depth_format_op,
                            extent_stp,
                            sample_count_op,
                            vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
                            vk::ImageAspectFlags::DEPTH,
                            vk::MemoryPropertyFlags::DEVICE_LOCAL,
                        )
                    })?;
                Ok(DepthImagesDefaultRtPkg {
                    images_extrl,
                    image_views_extrl,
                    device_memories_extrl,
                    desc: "depth_images",
                })
            }
        }
    }
}
