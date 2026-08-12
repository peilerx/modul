use ash::vk;

use crate::ModulResult;

/// Pool of present/signal semaphores indexed by **swapchain image**, not FIF slot.
/// Avoids VUID-vkQueueSubmit-pSignalSemaphores-00067 (binary semaphore still in use by present).
/// Typical swapchain is 2–3 images; 8 covers min+1 and high `maxImageCount` desktops.
pub const PRESENT_SIGNAL_SEMAPHORE_POOL: usize = 8;

/// Loop over frames-in-flight — processor owns control flow (B34/B63).
/// Single-unit materialize is passed as closures from gen (A1-GEN-MATERIALIZE).
///
/// * `image_available` + fences: one per FIF frame slot  
/// * `render_finished`: pool sized for swapchain images (present wait / submit signal)
pub(crate) fn update_frame_sync_semaphores<S, F>(
    frames_in_flight_stp: usize,
    present_signal_pool_stp: usize,
    mut semaphore_stp: S,
    mut fence_stp: F,
) -> ModulResult<(Vec<vk::Semaphore>, Vec<vk::Semaphore>, Vec<vk::Fence>)>
where
    S: FnMut() -> ModulResult<vk::Semaphore>,
    F: FnMut() -> ModulResult<vk::Fence>,
{
    let mut image_available_semaphores_extrl = Vec::with_capacity(frames_in_flight_stp);
    let mut in_flight_fences_extrl = Vec::with_capacity(frames_in_flight_stp);
    for _ in 0..frames_in_flight_stp {
        image_available_semaphores_extrl.push(semaphore_stp()?);
        in_flight_fences_extrl.push(fence_stp()?);
    }

    let present_n = present_signal_pool_stp.max(1);
    let mut render_finished_semaphores_extrl = Vec::with_capacity(present_n);
    for _ in 0..present_n {
        render_finished_semaphores_extrl.push(semaphore_stp()?);
    }

    Ok((
        image_available_semaphores_extrl,
        render_finished_semaphores_extrl,
        in_flight_fences_extrl,
    ))
}
