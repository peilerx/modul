/// Display session recipe — setup knobs only (FIX-086/091 · closed gestalt FIX-118).
///
/// All levers of `DisplayPresentPrt` that assemble needs live here.
/// `frames_in_flight_stp` may be **external** (aligned with FRAME) · still set per arm.
pub struct DisplayPresentDefaultStpPkg {
    /// Setup phase field `frames_in_flight_stp`.
    pub frames_in_flight_stp: u32,
    /// Clear attachments only · ¬ geometry bind/draw.
    pub clear_only_stp: bool,
    /// Bind pipeline + draw geometry (triangle etalon path).
    pub bind_geometry_stp: bool,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}
