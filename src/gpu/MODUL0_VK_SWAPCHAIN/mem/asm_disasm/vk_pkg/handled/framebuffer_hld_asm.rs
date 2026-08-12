//! vk_pkg — pack framebuffers using **vk** `Framebuffer` brick (FIX-120).

use ash::vk;

use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::RenderPassTriangleRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::DeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::DepthImagesDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::FramebufferDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::MsaaColorDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SwapchainDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SwapchainImageViewsDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk::handled::framebuffer_hld_asm::FramebufferHandled;
use crate::gpu::MODUL0_VK_SWAPCHAIN::proc::processor::framebuffer::update_framebuffer_attachment_layout;
use crate::ModulResult;

/// Catalog — pack `vk::Framebuffer` list (attachment layout strategy).
pub trait FramebufferDefaultHandled {
    fn handled_assemble(
        device_default_rt_pkg: &DeviceDefaultRtPkg,
        swapchain_default_rt_pkg: &SwapchainDefaultRtPkg,
        swapchain_image_views_default_rt_pkg: &SwapchainImageViewsDefaultRtPkg,
        depth_images_default_rt_pkg: &DepthImagesDefaultRtPkg,
        msaa_color_default_rt_pkg: &MsaaColorDefaultRtPkg,
        render_pass_triangle_rt_pkg: &RenderPassTriangleRtPkg,
        sample_count_op: vk::SampleCountFlags,
    ) -> ModulResult<FramebufferDefaultRtPkg>;
}

fn handled_simple_attachments(
    swapchain_view_extrl: vk::ImageView,
    depth_view_extrl: vk::ImageView,
) -> Vec<vk::ImageView> {
    vec![swapchain_view_extrl, depth_view_extrl]
}

fn handled_msaa_attachments(
    index_stp: usize,
    swapchain_view_extrl: vk::ImageView,
    depth_view_extrl: vk::ImageView,
    msaa_color_default_rt_pkg: &MsaaColorDefaultRtPkg,
) -> ModulResult<Vec<vk::ImageView>> {
    let msaa_view_extrl = msaa_color_default_rt_pkg
        .image_views_extrl
        .get(index_stp)
        .ok_or_else(|| format!("missing MSAA color view for framebuffer {index_stp}"))?;
    Ok(vec![*msaa_view_extrl, depth_view_extrl, swapchain_view_extrl])
}

impl FramebufferDefaultHandled for FramebufferDefaultRtPkg {
    fn handled_assemble(
        device_default_rt_pkg: &DeviceDefaultRtPkg,
        swapchain_default_rt_pkg: &SwapchainDefaultRtPkg,
        swapchain_image_views_default_rt_pkg: &SwapchainImageViewsDefaultRtPkg,
        depth_images_default_rt_pkg: &DepthImagesDefaultRtPkg,
        msaa_color_default_rt_pkg: &MsaaColorDefaultRtPkg,
        render_pass_triangle_rt_pkg: &RenderPassTriangleRtPkg,
        sample_count_op: vk::SampleCountFlags,
    ) -> ModulResult<FramebufferDefaultRtPkg> {
        {
            let extent_stp = swapchain_default_rt_pkg.extent_rt;
            let framebuffers_extrl = swapchain_image_views_default_rt_pkg
                .image_views_extrl
                .iter()
                .zip(depth_images_default_rt_pkg.image_views_extrl.iter())
                .enumerate()
                .map(|(index_stp, (&swapchain_view_extrl, &depth_view_extrl))| {
                    let attachments_extrl = update_framebuffer_attachment_layout(
                        sample_count_op,
                        || {
                            handled_simple_attachments(
                                swapchain_view_extrl,
                                depth_view_extrl,
                            )
                        },
                        || {
                            handled_msaa_attachments(
                                index_stp,
                                swapchain_view_extrl,
                                depth_view_extrl,
                                msaa_color_default_rt_pkg,
                            )
                        },
                    )?;
                    vk::Framebuffer::handled_assemble(
                        &device_default_rt_pkg.device_extrl,
                        render_pass_triangle_rt_pkg.render_pass_extrl,
                        &attachments_extrl,
                        extent_stp,
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Self {
                framebuffers_extrl,
                desc: "framebuffer",
            })
        }
    }
}
