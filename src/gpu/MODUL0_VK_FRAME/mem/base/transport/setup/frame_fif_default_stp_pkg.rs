/// Frame session recipe — setup knobs only (FIX-086/091 · closed gestalt FIX-118).
///
/// All levers of `FrameFifPrt` that assemble needs live here.
pub struct FrameFifDefaultStpPkg {
    /// Frames in flight (FIF) for sync + primary command buffers.
    pub frames_in_flight_stp: u32,
    /// Create in-flight fences already signaled (wait-first frame path).
    pub fences_signaled_stp: bool,
    /// Primary command buffers (vs secondary) for the session pool.
    pub primary_command_buffers_stp: bool,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}
