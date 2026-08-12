//! Host CPU load for ship logs (Linux `/proc` · Windows `Get*Times` · no extra crates).

use std::time::Instant;

/// Rolling process + system CPU percent (0…100+, process can exceed 100 on multi-core).
#[derive(Debug, Default)]
pub struct CpuSampler {
    prev_proc_ticks: Option<u64>,
    prev_proc_wall: Option<Instant>,
    prev_sys_total: Option<u64>,
    prev_sys_idle: Option<u64>,
    /// This process CPU% over last sample window (sum of cores).
    pub process_pct: f32,
    /// Whole-machine non-idle CPU% over last sample window.
    pub system_pct: f32,
}

impl CpuSampler {
    pub fn new() -> Self {
        let mut s = Self::default();
        s.sample(); // prime baselines (pct stays 0 until 2nd sample)
        s
    }

    /// Sample host counters and update `process_pct` / `system_pct`.
    pub fn sample(&mut self) {
        self.sample_process();
        self.sample_system();
    }

    fn sample_process(&mut self) {
        let Some(ticks) = read_self_cpu_ticks() else {
            return;
        };
        let now = Instant::now();
        if let (Some(prev_t), Some(prev_w)) = (self.prev_proc_ticks, self.prev_proc_wall) {
            let wall = now.duration_since(prev_w).as_secs_f32().max(1e-3);
            let d_ticks = ticks.saturating_sub(prev_t) as f32;
            self.process_pct = ticks_to_process_pct(d_ticks, wall);
        }
        self.prev_proc_ticks = Some(ticks);
        self.prev_proc_wall = Some(now);
    }

    fn sample_system(&mut self) {
        let Some((total, idle)) = read_system_cpu_totals() else {
            return;
        };
        if let (Some(pt), Some(pi)) = (self.prev_sys_total, self.prev_sys_idle) {
            let d_total = total.saturating_sub(pt) as f32;
            let d_idle = idle.saturating_sub(pi) as f32;
            if d_total > 0.0 {
                self.system_pct = ((d_total - d_idle) / d_total * 100.0).clamp(0.0, 100.0);
            }
        }
        self.prev_sys_total = Some(total);
        self.prev_sys_idle = Some(idle);
    }
}

#[cfg(target_os = "linux")]
fn ticks_to_process_pct(d_ticks: f32, wall_secs: f32) -> f32 {
    // Linux USER_HZ (almost always 100).
    const CLK_TCK: f32 = 100.0;
    (d_ticks / CLK_TCK) / wall_secs * 100.0
}

#[cfg(target_os = "windows")]
fn ticks_to_process_pct(d_ticks: f32, wall_secs: f32) -> f32 {
    // FILETIME: 100 ns units
    const PER_SEC: f32 = 10_000_000.0;
    (d_ticks / PER_SEC) / wall_secs * 100.0
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn ticks_to_process_pct(_d_ticks: f32, _wall_secs: f32) -> f32 {
    0.0
}

#[cfg(target_os = "linux")]
fn read_self_cpu_ticks() -> Option<u64> {
    let raw = std::fs::read_to_string("/proc/self/stat").ok()?;
    let after = raw.rsplit_once(')')?.1;
    let mut it = after.split_whitespace();
    let utime: u64 = it.nth(11)?.parse().ok()?;
    let stime: u64 = it.next()?.parse().ok()?;
    Some(utime.saturating_add(stime))
}

#[cfg(target_os = "linux")]
fn read_system_cpu_totals() -> Option<(u64, u64)> {
    let raw = std::fs::read_to_string("/proc/stat").ok()?;
    let line = raw.lines().next()?;
    let mut parts = line.split_whitespace();
    if parts.next()? != "cpu" {
        return None;
    }
    let mut vals = [0u64; 10];
    let mut n = 0usize;
    for p in parts {
        if n >= vals.len() {
            break;
        }
        vals[n] = p.parse().ok()?;
        n += 1;
    }
    if n < 4 {
        return None;
    }
    let total: u64 = vals[..n].iter().sum();
    let idle = vals[3].saturating_add(if n > 4 { vals[4] } else { 0 });
    Some((total, idle))
}

#[cfg(target_os = "windows")]
mod win_cpu {
    #![allow(non_snake_case, reason = "Win32 API names")]

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    pub struct Filetime {
        pub dwLowDateTime: u32,
        pub dwHighDateTime: u32,
    }

    impl Filetime {
        pub fn as_u64(self) -> u64 {
            (u64::from(self.dwHighDateTime) << 32) | u64::from(self.dwLowDateTime)
        }
    }

    #[link(name = "kernel32")]
    extern "system" {
        fn GetCurrentProcess() -> *mut core::ffi::c_void;
        fn GetProcessTimes(
            hProcess: *mut core::ffi::c_void,
            lpCreationTime: *mut Filetime,
            lpExitTime: *mut Filetime,
            lpKernelTime: *mut Filetime,
            lpUserTime: *mut Filetime,
        ) -> i32;
        fn GetSystemTimes(
            lpIdleTime: *mut Filetime,
            lpKernelTime: *mut Filetime,
            lpUserTime: *mut Filetime,
        ) -> i32;
    }

    pub fn process_ticks() -> Option<u64> {
        let mut creation = Filetime::default();
        let mut exit = Filetime::default();
        let mut kernel = Filetime::default();
        let mut user = Filetime::default();
        let ok = unsafe {
            GetProcessTimes(
                GetCurrentProcess(),
                &mut creation,
                &mut exit,
                &mut kernel,
                &mut user,
            )
        };
        if ok == 0 {
            return None;
        }
        Some(kernel.as_u64().saturating_add(user.as_u64()))
    }

    pub fn system_totals() -> Option<(u64, u64)> {
        let mut idle = Filetime::default();
        let mut kernel = Filetime::default();
        let mut user = Filetime::default();
        let ok = unsafe { GetSystemTimes(&mut idle, &mut kernel, &mut user) };
        if ok == 0 {
            return None;
        }
        // kernel includes idle on Windows.
        let total = kernel.as_u64().saturating_add(user.as_u64());
        let idle_t = idle.as_u64();
        Some((total, idle_t))
    }
}

#[cfg(target_os = "windows")]
fn read_self_cpu_ticks() -> Option<u64> {
    win_cpu::process_ticks()
}

#[cfg(target_os = "windows")]
fn read_system_cpu_totals() -> Option<(u64, u64)> {
    win_cpu::system_totals()
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn read_self_cpu_ticks() -> Option<u64> {
    None
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn read_system_cpu_totals() -> Option<(u64, u64)> {
    None
}
