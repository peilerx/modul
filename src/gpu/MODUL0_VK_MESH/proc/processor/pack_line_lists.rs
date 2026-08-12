//! Pack world-space line lists (optional line lists).

/// Interleaved f32 pos.xyz for `LINE_LIST`.
#[must_use]
pub fn pack_grid_lines(half: f32, step: f32) -> Vec<f32> {
    let mut v = Vec::new();
    let n = ((half / step).ceil() as i32).max(1);
    for i in -n..=n {
        let t = i as f32 * step;
        v.extend_from_slice(&[-half, 0.0, t, half, 0.0, t]);
        v.extend_from_slice(&[t, 0.0, -half, t, 0.0, half]);
    }
    v.extend_from_slice(&[-half, 0.0, 0.0, half, 0.0, 0.0]);
    v.extend_from_slice(&[0.0, 0.0, -half, 0.0, 0.0, half]);
    v
}

/// `f32_pos_to_bytes` — function (f 32 pos to bytes).
/// Public API entry for this module.
/// Belongs to: mesh upload / solid draw MCG.
#[must_use]
pub fn f32_pos_to_bytes(v: &[f32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(std::mem::size_of_val(v));
    for f in v {
        out.extend_from_slice(&f.to_ne_bytes());
    }
    out
}
