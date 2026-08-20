//! Host readback of the heat SoA. Logs a histogram + a 16×16 face patch.

use std::fmt::Write as _;

use ash::vk;

use crate::tandem::proc::session_log;
use modul::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayDefaultRtCrg;
use modul::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::DeviceDefaultRtPkg;

/// Map `heat[]`, count bins, dump a front-face patch. GPU must be idle.
#[allow(dead_code, reason = "optional host histogram, not on the pulse path")]
pub fn dump_heat(
    device: &DeviceDefaultRtPkg,
    display: &DisplayDefaultRtCrg,
    tag: &str,
) {
    if display.soa_heat_buffer_extrl == vk::Buffer::null() || display.soa_heat_bytes_rt < 4 {
        session_log::log("HEAT dump skipped · buffer null");
        return;
    }
    let n = (display.soa_heat_bytes_rt / 4) as usize;
    let dev = &device.device_extrl;
    let mapped = unsafe {
        if let Err(e) = dev.device_wait_idle() {
            session_log::log(&format!("HEAT dump wait_idle failed: {e:?}"));
            return;
        }
        match dev.map_memory(
            display.soa_heat_memory_extrl,
            0,
            display.soa_heat_bytes_rt,
            vk::MemoryMapFlags::empty(),
        ) {
            Ok(p) => p,
            Err(e) => {
                session_log::log(&format!("HEAT dump map failed: {e:?}"));
                return;
            }
        }
    };
    let sl = unsafe { std::slice::from_raw_parts(mapped.cast::<f32>(), n) };

    let step = (n / 250_000).max(1);
    let mut nz = 0u32;
    let mut ge1 = 0u32;
    let mut ge2 = 0u32;
    let mut max_h = 0.0f32;
    let mut samples = 0u32;
    for i in (0..n).step_by(step) {
        let h = sl[i];
        samples += 1;
        if h > 1e-4 {
            nz += 1;
        }
        if h >= 1.0 {
            ge1 += 1;
        }
        if h >= 1.999 {
            ge2 += 1;
        }
        if h > max_h {
            max_h = h;
        }
    }

    let nx = (n as f32).cbrt().ceil() as usize;
    let ny = nx.max(1);
    let plane = nx * ny;
    let mut hot_lin = 0usize;
    let mut hot_v = 0.0f32;
    for (i, &h) in sl.iter().enumerate().step_by(step) {
        if h > hot_v {
            hot_v = h;
            hot_lin = i;
        }
    }
    let iz0 = hot_lin / plane;
    let rem = hot_lin % plane;
    let iy0 = rem / nx;
    let ix0 = rem % nx;
    let mut patch = format!("  hottest lin={hot_lin} ({ix0},{iy0},{iz0}) h={hot_v:.3} · 16x8 around it:\n");
    for row in 0..8 {
        patch.push_str("   ");
        for col in 0..16 {
            let ix = ix0.saturating_add(col).saturating_sub(8);
            let iy = iy0.saturating_add(row).saturating_sub(4);
            let h = if ix < nx && iy < ny {
                let lin = ix + iy * nx + iz0 * plane;
                sl.get(lin).copied().unwrap_or(-1.0)
            } else {
                -1.0
            };
            if h < 0.0 {
                patch.push_str("  . ");
            } else if h >= 1.999 {
                patch.push_str(" ## ");
            } else if h > 0.05 {
                let _ = write!(patch, " {h:.1}");
            } else {
                patch.push_str("  . ");
            }
        }
        patch.push('\n');
    }

    session_log::log(&format!(
        "HEAT dump · {tag} · n={n} nx={nx} · stride={step} samples={samples} · max={max_h:.3} nz={nz} ge1={ge1} ge2={ge2} · paint={} hold={:.2} mouse=({:.3},{:.3})",
        display.heat_paint_rt,
        display.heat_hold_rt,
        display.heat_mouse_x_rt,
        display.heat_mouse_y_rt
    ));
    session_log::log(&patch);

    unsafe {
        dev.unmap_memory(display.soa_heat_memory_extrl);
    }
}
