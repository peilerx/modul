//! FRAME domain tick — wait · acquire · submit · present · advance (P · FIX-117/118).
//!
//! Peels = *Rt from transport re-export of base (¬ parallel *Res).

use ash::vk;

use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::FrameSyncDefaultRtPkg;
use crate::gpu::MODUL0_VK_FRAME::mem::base::embedded::runtime::frame_info_res_intsct_rt::PresentInfoDefaultRt;
use crate::gpu::MODUL0_VK_FRAME::mem::base::embedded::runtime::frame_info_res_intsct_rt::SubmitInfoDefaultRt;
use crate::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_pkg::auto::present_info_res_intsct_at_asm::auto_vk_present;
use crate::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_pkg::auto::submit_info_res_intsct_at_asm::auto_vk_submit;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::FrameSlotDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::{
    DeviceDefaultRtPkg, SwapchainDefaultRtPkg, SwapchainLoaderDefaultRtPkg,
};
use crate::{map_vk, ModulResult};

/// Resolve current FIF slot from Internal sync bag.
pub fn current_slot(sync: &FrameSyncDefaultRtPkg) -> ModulResult<FrameSlotDefaultRtPkg> {
    let i = sync.current_frame_rt;
    if i >= sync.frames_in_flight_rt {
        return Err(format!(
            "frame_tick: current_frame {} out of FIF {}",
            i, sync.frames_in_flight_rt
        ));
    }
    Ok(FrameSlotDefaultRtPkg {
        slot_rt: i,
        image_available_semaphore_extrl: *sync
            .image_available_semaphores_extrl
            .get(i)
            .ok_or("frame_tick: missing image_available semaphore")?,
        render_finished_semaphore_extrl: *sync
            .render_finished_semaphores_extrl
            .get(i)
            .ok_or("frame_tick: missing render_finished semaphore")?,
        in_flight_fence_extrl: *sync
            .in_flight_fences_extrl
            .get(i)
            .ok_or("frame_tick: missing in_flight fence")?,
        command_buffer_extrl: *sync
            .command_buffers_extrl
            .get(i)
            .ok_or("frame_tick: missing command buffer")?,
        desc: "frame_slot_current",
    })
}

/// `wait_slot_fence` — function (wait slot fence).
/// Public API entry for this module.
/// # Errors
///
/// Returns [`ModulResult`] / `Result` on Vulkan or validation failure.
/// Belongs to: frames-in-flight MCG.
pub fn wait_slot_fence(device: &DeviceDefaultRtPkg, slot: &FrameSlotDefaultRtPkg) -> ModulResult<()> {
    let fences = [slot.in_flight_fence_extrl];
    map_vk(unsafe {
        device
            .device_extrl
            .wait_for_fences(&fences, true, u64::MAX)
    })?;
    map_vk(unsafe { device.device_extrl.reset_fences(&fences) })?;
    Ok(())
}

/// `acquire_next_image` — function (acquire next image).
/// Public API entry for this module.
/// Belongs to: frames-in-flight MCG.
pub fn acquire_next_image(
    loader: &SwapchainLoaderDefaultRtPkg,
    swapchain: &SwapchainDefaultRtPkg,
    slot: &FrameSlotDefaultRtPkg,
) -> ModulResult<u32> {
    let (image_index, suboptimal) = unsafe {
        loader.swapchain_loader_extrl.acquire_next_image(
            swapchain.swapchain_extrl,
            u64::MAX,
            slot.image_available_semaphore_extrl,
            vk::Fence::null(),
        )
    }
    .map_err(|e| format!("frame_tick acquire: {e:?}"))?;
    let _ = suboptimal;
    Ok(image_index)
}

/// `submit_slot` — function (submit slot).
/// Public API entry for this module.
/// # Errors
///
/// Returns [`ModulResult`] / `Result` on Vulkan or validation failure.
/// Belongs to: frames-in-flight MCG.
pub fn submit_slot(device: &DeviceDefaultRtPkg, slot: &FrameSlotDefaultRtPkg) -> ModulResult<()> {
    // Peels only — bag literal in proc; catalog assemble stays in asm_disasm (FIX-131).
    let submit_rt = SubmitInfoDefaultRt {
        wait_semaphore_extrl: slot.image_available_semaphore_extrl,
        wait_dst_stage_mask_op: vec![vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT],
        command_buffer_extrl: slot.command_buffer_extrl,
        signal_semaphore_extrl: slot.render_finished_semaphore_extrl,
        desc: "submit_info",
    };
    let submit_info = auto_vk_submit(&submit_rt);
    let submits = [submit_info];
    map_vk(unsafe {
        device.device_extrl.queue_submit(
            device.graphics_queue_extrl,
            &submits,
            slot.in_flight_fence_extrl,
        )
    })?;
    Ok(())
}

/// `present_image` — function (present image).
/// Public API entry for this module.
/// Belongs to: frames-in-flight MCG.
pub fn present_image(
    device: &DeviceDefaultRtPkg,
    loader: &SwapchainLoaderDefaultRtPkg,
    swapchain: &SwapchainDefaultRtPkg,
    slot: &FrameSlotDefaultRtPkg,
    image_index: u32,
) -> ModulResult<()> {
    // Peels only — bag literal in proc; catalog assemble stays in asm_disasm (FIX-131).
    let present_rt = PresentInfoDefaultRt {
        wait_semaphore_extrl: slot.render_finished_semaphore_extrl,
        swapchain_extrl: swapchain.swapchain_extrl,
        image_index_rt: image_index,
        desc: "present_info",
    };
    let present_info = auto_vk_present(&present_rt);
    let result = unsafe {
        loader
            .swapchain_loader_extrl
            .queue_present(device.present_queue_extrl, &present_info)
    };
    match result {
        Ok(_suboptimal) => Ok(()),
        Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
            Err("frame_tick present: OUT_OF_DATE_KHR (recreate swapchain)".into())
        }
        Err(e) => Err(format!("frame_tick present: {e:?}")),
    }
}

/// `advance_frame` — function (advance frame).
/// Public API entry for this module.
/// Belongs to: frames-in-flight MCG.
pub const fn advance_frame(sync: &mut FrameSyncDefaultRtPkg) {
    if sync.frames_in_flight_rt == 0 {
        return;
    }
    sync.current_frame_rt = (sync.current_frame_rt + 1) % sync.frames_in_flight_rt;
}


use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::FrameDefaultRtCrg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::PresentationDefaultRtCrg;

/// Wait + acquire for current FIF slot. Returns (slot, `image_index`).
pub fn begin_frame(
    device: &DeviceDefaultRtPkg,
    presentation: &PresentationDefaultRtCrg,
    loader: &SwapchainLoaderDefaultRtPkg,
    frame_rt: &FrameDefaultRtCrg,
) -> ModulResult<(FrameSlotDefaultRtPkg, u32)> {
    let slot = current_slot(&frame_rt.frame_sync_default_rt_pkg)?;
    wait_slot_fence(device, &slot)?;
    let image_index = acquire_next_image(loader, &presentation.swapchain_default_rt_pkg, &slot)?;
    Ok((slot, image_index))
}

/// Submit + present + advance FIF after DISPLAY recorded the slot CB.
pub fn end_frame(
    device: &DeviceDefaultRtPkg,
    presentation: &PresentationDefaultRtCrg,
    loader: &SwapchainLoaderDefaultRtPkg,
    frame_rt: &mut FrameDefaultRtCrg,
    slot: &FrameSlotDefaultRtPkg,
    image_index: u32,
) -> ModulResult<()> {
    submit_slot(device, slot)?;
    present_image(
        device,
        loader,
        &presentation.swapchain_default_rt_pkg,
        slot,
        image_index,
    )?;
    advance_frame(&mut frame_rt.frame_sync_default_rt_pkg);
    Ok(())
}
