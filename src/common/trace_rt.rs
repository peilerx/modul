//! Runtime trace peel — **X-RAY** observability for modul + modulcad.
//!
//! **X-RAY** (product name) ≡ this TRACE channel: runtime logging of app shell,
//! tandem order, CAD recompute, mesh stats, paint/frame. It is **not** a solid
//! renderer and not the optional CAD peel scorecard (`xray_view_mesh`).
//!
//! Enable:
//! - UI: modulcad Settings → «X-RAY on → TRACE log»
//! - Env: `MODULCAD_TRACE=1` (optional `MODULCAD_TRACE_DEEP=0` to leave deep off)
//! - File: `MODULCAD_TRACE_FILE` or `modul/modulcad/MODULCAD_TRACE.log`
//!
//! Deep mode (`trace_deep`): extra samples (paint_diag, scorecard hooks).
//! UI checkbox turns deep on with TRACE.

use std::fs::OpenOptions;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static ENABLED: AtomicBool = AtomicBool::new(false);
/// Deep mode: paint_diag + zbuffer void flood + frame samples (UI checkbox = deep).
static DEEP: AtomicBool = AtomicBool::new(false);
static SEQ: AtomicU64 = AtomicU64::new(0);
static INIT: AtomicBool = AtomicBool::new(false);

/// `trace_init_from_env` — function (trace init from env).
/// Optional host tracing (env-gated).
/// Belongs to: common helpers.
pub fn trace_init_from_env() {
    if INIT.swap(true, Ordering::SeqCst) {
        return;
    }
    let on = std::env::var("MODULCAD_TRACE")
        .or_else(|_| std::env::var("MODUL_TRACE"))
        .map(|v| v != "0" && !v.is_empty())
        .unwrap_or(false);
    let deep = std::env::var("MODULCAD_TRACE_DEEP")
        .map(|v| v != "0" && !v.is_empty())
        .unwrap_or(on); // env TRACE implies deep unless TRACE_DEEP=0
    ENABLED.store(on, Ordering::SeqCst);
    DEEP.store(on && deep, Ordering::SeqCst);
    if on {
        let _ = std::fs::write(
            trace_log_path(),
            format!("# MODULCAD_TRACE start unix_ms={} deep={}\n", now_ms(), deep),
        );
        trace_emit(
            "COMMON",
            "trace_init",
            &format!("enabled=1 deep={} diag=BUG_flags", deep as u8),
        );
    }
}

/// `trace_enabled` — function (trace enabled).
/// Optional host tracing (env-gated).
/// Belongs to: common helpers.
pub fn trace_enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

/// Deep diagnostics (extra frame samples / scorecard hooks). On with UI X-RAY.
pub fn trace_deep() -> bool {
    ENABLED.load(Ordering::Relaxed) && DEEP.load(Ordering::Relaxed)
}

/// X-RAY UI / API toggle → TRACE on|off. Deep follows enable (full runtime log).
pub fn trace_set_enabled(on: bool) {
    let was = ENABLED.swap(on, Ordering::SeqCst);
    DEEP.store(on, Ordering::SeqCst); // X-RAY checkbox = TRACE + deep samples
    if on && !was {
        INIT.store(true, Ordering::SeqCst);
        let _ = OpenOptions::new()
            .create(true)
            .append(true)
            .open(trace_log_path())
            .and_then(|mut f| {
                writeln!(
                    f,
                    "# MODULCAD_TRACE X-RAY enable_ui unix_ms={} (runtime log)",
                    now_ms()
                )
            });
        let seq = SEQ.fetch_add(1, Ordering::Relaxed);
        let line = format!(
            "{} | {:>6} | {:<18} | {:<28} | {}\n",
            now_ms(),
            seq,
            "COMMON",
            "xray_trace_on",
            "enabled=1 deep=1 source=modulcad_xray (runtime TRACE)"
        );
        eprint!("{line}");
        let _ = OpenOptions::new()
            .create(true)
            .append(true)
            .open(trace_log_path())
            .and_then(|mut f| f.write_all(line.as_bytes()));
    } else if !on && was {
        let seq = SEQ.fetch_add(1, Ordering::Relaxed);
        let line = format!(
            "{} | {:>6} | {:<18} | {:<28} | {}\n",
            now_ms(),
            seq,
            "COMMON",
            "xray_trace_off",
            "enabled=0 deep=0 source=modulcad_xray"
        );
        eprint!("{line}");
        let _ = OpenOptions::new()
            .create(true)
            .append(true)
            .open(trace_log_path())
            .and_then(|mut f| f.write_all(line.as_bytes()));
        ENABLED.store(false, Ordering::SeqCst);
        DEEP.store(false, Ordering::SeqCst);
    }
}

/// Throttled sample helper (ms since last). Returns true if should emit.
pub fn trace_throttle(slot: u32, min_ms: u64) -> bool {
    if !trace_enabled() {
        return false;
    }
    static LAST: [AtomicU64; 8] = [
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
        AtomicU64::new(0),
    ];
    let i = (slot as usize) % 8;
    let now = now_ms() as u64;
    let prev = LAST[i].load(Ordering::Relaxed);
    if now.saturating_sub(prev) >= min_ms {
        LAST[i].store(now, Ordering::Relaxed);
        true
    } else {
        false
    }
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

fn trace_log_path() -> String {
    std::env::var("MODULCAD_TRACE_FILE")
        .unwrap_or_else(|_| "modul/modulcad/MODULCAD_TRACE.log".into())
}

/// `trace_emit` — function (trace emit).
/// Optional host tracing (env-gated).
/// Belongs to: common helpers.
pub fn trace_emit(mcg: &str, event: &str, detail: &str) {
    if !trace_enabled() {
        return;
    }
    let seq = SEQ.fetch_add(1, Ordering::Relaxed);
    let line = format!(
        "{} | {:>6} | {:<18} | {:<28} | {}\n",
        now_ms(),
        seq,
        mcg,
        event,
        detail
    );
    eprint!("{line}");
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(trace_log_path())
    {
        let _ = f.write_all(line.as_bytes());
    }
}

/// Sketch 2D loop diagnostics (concave / open / self-cross risk).
pub fn trace_sketch_loop(xs: &[f64], ys: &[f64], fronts: &[u32], backs: &[u32]) {
    if !trace_enabled() {
        return;
    }
    let n = xs.len();
    if n == 0 {
        trace_emit("SKETCH", "BUG/empty_loop", "pts=0");
        return;
    }
    let mut area2 = 0.0;
    for i in 0..n {
        let j = (i + 1) % n;
        area2 += xs[i] * ys[j] - xs[j] * ys[i];
    }
    let area = area2 * 0.5;
    // segment count vs pts
    let segs = fronts.len().min(backs.len());
    let mut min_edge = f64::MAX;
    for i in 0..segs {
        let a = fronts[i] as usize;
        let b = backs[i] as usize;
        if a < n && b < n {
            let dx = xs[b] - xs[a];
            let dy = ys[b] - ys[a];
            min_edge = min_edge.min((dx * dx + dy * dy).sqrt());
        }
    }
    if min_edge == f64::MAX {
        min_edge = 0.0;
    }
    // simple concave test: any internal angle turn flips relative to area sign
    let mut reflex = 0u32;
    if n >= 3 {
        let sign = if area >= 0.0 { 1.0 } else { -1.0 };
        for i in 0..n {
            let i0 = (i + n - 1) % n;
            let i1 = i;
            let i2 = (i + 1) % n;
            let e1x = xs[i1] - xs[i0];
            let e1y = ys[i1] - ys[i0];
            let e2x = xs[i2] - xs[i1];
            let e2y = ys[i2] - ys[i1];
            let cross = e1x * e2y - e1y * e2x;
            if cross * sign < -1e-12 {
                reflex += 1;
            }
        }
    }
    let pts_s: String = xs
        .iter()
        .zip(ys.iter())
        .take(12)
        .map(|(x, y)| format!("({x:.3},{y:.3})"))
        .collect::<Vec<_>>()
        .join(" ");
    trace_emit(
        "SKETCH",
        "loop_diag",
        &format!(
            "pts={n} segs={segs} area={area:.5} min_edge={min_edge:.5} reflex={reflex} pts_xy={pts_s}"
        ),
    );
    if area.abs() < 1e-8 {
        trace_emit("SKETCH", "BUG/zero_loop_area", &format!("area={area}"));
    }
    if segs < n {
        trace_emit(
            "SKETCH",
            "BUG/open_or_short_wire",
            &format!("segs={segs} pts={n}"),
        );
    }
    if reflex > 0 {
        trace_emit(
            "SKETCH",
            "WARN/concave_loop",
            &format!("reflex_verts={reflex} (need ear-clip)"),
        );
    }
}

/// Full mesh diagnostics: zero area, outward score, open edges, bbox.
pub fn trace_mesh_stats(
    mcg: &str,
    event: &str,
    kind: &str,
    n_pos: usize,
    indices: &[u32],
    pos_xs: &[f32],
    pos_ys: &[f32],
    pos_zs: &[f32],
) {
    if !trace_enabled() {
        return;
    }
    let mut zero_area = 0u32;
    let mut bad_idx = 0u32;
    let mut outward = 0i32;
    let mut min_a2 = f32::MAX;
    let mut max_a2 = 0.0f32;
    let mut cx = 0.0f32;
    let mut cy = 0.0f32;
    let mut cz = 0.0f32;
    if n_pos > 0 {
        for i in 0..n_pos {
            cx += pos_xs[i];
            cy += pos_ys[i];
            cz += pos_zs[i];
        }
        let inv = 1.0 / n_pos as f32;
        cx *= inv;
        cy *= inv;
        cz *= inv;
    }
    // open edge count (edges with count != 2)
    use std::collections::HashMap;
    let mut edge_use: HashMap<(u32, u32), u32> = HashMap::new();
    let mut i = 0;
    while i + 2 < indices.len() {
        let ia = indices[i] as usize;
        let ib = indices[i + 1] as usize;
        let ic = indices[i + 2] as usize;
        if ia >= n_pos || ib >= n_pos || ic >= n_pos {
            bad_idx += 1;
            i += 3;
            continue;
        }
        for (a, b) in [
            (indices[i], indices[i + 1]),
            (indices[i + 1], indices[i + 2]),
            (indices[i + 2], indices[i]),
        ] {
            let e = if a < b { (a, b) } else { (b, a) };
            *edge_use.entry(e).or_insert(0) += 1;
        }
        let e1x = pos_xs[ib] - pos_xs[ia];
        let e1y = pos_ys[ib] - pos_ys[ia];
        let e1z = pos_zs[ib] - pos_zs[ia];
        let e2x = pos_xs[ic] - pos_xs[ia];
        let e2y = pos_ys[ic] - pos_ys[ia];
        let e2z = pos_zs[ic] - pos_zs[ia];
        let nx = e1y * e2z - e1z * e2y;
        let ny = e1z * e2x - e1x * e2z;
        let nz = e1x * e2y - e1y * e2x;
        let a2 = nx * nx + ny * ny + nz * nz;
        min_a2 = min_a2.min(a2);
        max_a2 = max_a2.max(a2);
        if a2 < 1e-16 {
            zero_area += 1;
        } else {
            let mx = (pos_xs[ia] + pos_xs[ib] + pos_xs[ic]) / 3.0 - cx;
            let my = (pos_ys[ia] + pos_ys[ib] + pos_ys[ic]) / 3.0 - cy;
            let mz = (pos_zs[ia] + pos_zs[ib] + pos_zs[ic]) / 3.0 - cz;
            if nx * mx + ny * my + nz * mz > 0.0 {
                outward += 1;
            } else {
                outward -= 1;
            }
        }
        i += 3;
    }
    let n_tris = indices.len() / 3;
    let mut boundary = 0u32;
    let mut nonmanifold = 0u32;
    for c in edge_use.values() {
        if *c == 1 {
            boundary += 1;
        } else if *c > 2 {
            nonmanifold += 1;
        }
    }
    if min_a2 == f32::MAX {
        min_a2 = 0.0;
    }
    trace_emit(
        mcg,
        event,
        &format!(
            "kind={kind} verts={n_pos} tris={n_tris} zero_area={zero_area} bad_idx={bad_idx} outward_score={outward} boundary_edges={boundary} nonmanifold_edges={nonmanifold} min_a2={min_a2:.3e} max_a2={max_a2:.3e} centroid=({cx:.3},{cy:.3},{cz:.3})"
        ),
    );
    if zero_area > 0 {
        trace_emit(
            mcg,
            "BUG/zero_area_tris",
            &format!("count={zero_area}/{n_tris}"),
        );
    }
    if bad_idx > 0 {
        trace_emit(mcg, "BUG/bad_indices", &format!("count={bad_idx}"));
    }
    if boundary > 0 {
        trace_emit(
            mcg,
            "BUG/open_surface",
            &format!("boundary_edges={boundary} (closed solid expects 0)"),
        );
    }
    if nonmanifold > 0 {
        trace_emit(
            mcg,
            "BUG/nonmanifold",
            &format!("edges={nonmanifold}"),
        );
    }
    // Centroid test is reliable for genus-0 convex-ish shells only.
    // Torus (and other genus≥1) has solid volume away from mesh centroid → false positives.
    // Fire only on true majority invert (score negative).
    if n_tris > 0 && outward < 0 {
        trace_emit(
            mcg,
            "BUG/inward_winding_majority",
            &format!("outward_score={outward} tris={n_tris}"),
        );
    } else if n_tris > 8 && outward < (n_tris as i32) / 4 && boundary == 0 {
        // weak signal for closed genus-0 only
        trace_emit(
            mcg,
            "WARN/mixed_winding",
            &format!("outward_score={outward} tris={n_tris} (ok for torus/genus≥1)"),
        );
    }
}

/// Paint-path diagnostics (where visual voids / hairlines actually appear).
pub fn trace_paint_diag(
    kind: &str,
    mesh_tris: usize,
    drawn: usize,
    culled_back: u32,
    culled_degen: u32,
    screen_zero: u32,
    screen_sliver: u32,
    depth_min: f32,
    depth_max: f32,
    screen_bbox: (f32, f32, f32, f32),
) {
    if !trace_enabled() {
        return;
    }
    let (x0, y0, x1, y1) = screen_bbox;
    trace_emit(
        "CAD_VIEW",
        "paint_diag",
        &format!(
            "kind={kind} mesh_tris={mesh_tris} drawn={drawn} cull_back={culled_back} cull_degen={culled_degen} scr_zero={screen_zero} scr_sliver={screen_sliver} depth=[{depth_min:.3},{depth_max:.3}] scr_bbox=({x0:.1},{y0:.1})-({x1:.1},{y1:.1})"
        ),
    );
    if mesh_tris > 0 && drawn == 0 {
        trace_emit(
            "CAD_VIEW",
            "BUG/paint_empty",
            "all tris culled — solid invisible",
        );
    }
    // more than 55% culled may mean inverted mesh or wrong cull for this view
    if mesh_tris > 4 && culled_back as usize * 100 / mesh_tris > 70 {
        trace_emit(
            "CAD_VIEW",
            "WARN/heavy_backface_cull",
            &format!(
                "cull_back={culled_back}/{} — possible inverted faces or voids",
                mesh_tris
            ),
        );
    }
    if screen_zero > 0 {
        trace_emit(
            "CAD_VIEW",
            "BUG/screen_zero_area",
            &format!("count={screen_zero} (invisible tris after project)"),
        );
    }
    // Dense UV silhouettes always produce sub-px tris — not a solid defect.
    if screen_sliver > 32 {
        trace_emit(
            "CAD_VIEW",
            "WARN/screen_sliver",
            &format!("count={screen_sliver} area_px<0.5"),
        );
    }
    let bw = x1 - x0;
    let bh = y1 - y0;
    if drawn > 0 && (bw < 2.0 || bh < 2.0) {
        trace_emit(
            "CAD_VIEW",
            "BUG/tiny_screen_bbox",
            &format!("w={bw:.2} h={bh:.2}"),
        );
    }
}

/// Z-buffer coverage — true void metric (pixels of bg inside solid bbox).
pub fn trace_zbuffer_coverage(
    kind: &str,
    covered: u32,
    total: u32,
    void_interior: u32,
    fragments: u32,
    w: usize,
    h: usize,
) {
    if !trace_enabled() {
        return;
    }
    let cov = if total > 0 {
        100.0 * covered as f32 / total as f32
    } else {
        0.0
    };
    trace_emit(
        "CAD_VIEW",
        "zbuffer_diag",
        &format!(
            "kind={kind} {w}x{h} covered={covered}/{total} ({cov:.1}%) void_interior={void_interior} fragments={fragments}"
        ),
    );
    // Prefer X-RAY cluster classification (gap vs topo_hole). This line is legacy summary.
    if void_interior > 20 {
        trace_emit(
            "CAD_VIEW",
            "WARN/enclosed_empty",
            &format!(
                "enclosed_void_px={void_interior} — see XRAY scorecard (topo_hole=donut OK · gap=mesh crack)"
            ),
        );
    } else if void_interior > 0 {
        trace_emit(
            "CAD_VIEW",
            "WARN/micro_voids",
            &format!("enclosed_void_px={void_interior}"),
        );
    }
}
