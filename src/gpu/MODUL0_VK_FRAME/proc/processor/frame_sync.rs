use ash::vk;

use crate::ModulResult;

/// Loop over frames-in-flight — processor owns control flow (B34/B63).
/// Single-unit materialize is passed as closures from gen (A1-GEN-MATERIALIZE).
pub(crate) fn update_frame_sync_semaphores<S, F>(
    frames_in_flight_stp: usize,
    mut semaphore_stp: S,
    mut fence_stp: F,
) -> ModulResult<(Vec<vk::Semaphore>, Vec<vk::Semaphore>, Vec<vk::Fence>)>
where
    S: FnMut() -> ModulResult<vk::Semaphore>,
    F: FnMut() -> ModulResult<vk::Fence>,
{
    let mut image_available_semaphores_extrl = Vec::with_capacity(frames_in_flight_stp);
    let mut render_finished_semaphores_extrl = Vec::with_capacity(frames_in_flight_stp);
    let mut in_flight_fences_extrl = Vec::with_capacity(frames_in_flight_stp);

    for _ in 0..frames_in_flight_stp {
        image_available_semaphores_extrl.push(semaphore_stp()?);
        render_finished_semaphores_extrl.push(semaphore_stp()?);
        in_flight_fences_extrl.push(fence_stp()?);
    }

    Ok((
        image_available_semaphores_extrl,
        render_finished_semaphores_extrl,
        in_flight_fences_extrl,
    ))
}
