//! Session log — all cubes ship output goes to a txt next to the binary / cwd.
//!
//! Files (created on start):
//! - `cubes_session_log.txt`  — timeline (boot, GPU, FPS, CPU load, errors)
//! - `cubes_vk_validation.txt` — Khronos validation ERROR/WARNING lines

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

static SESSION: Mutex<Option<File>> = Mutex::new(None);
static VK_VAL: Mutex<Option<File>> = Mutex::new(None);
static LOG_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);

/// Prefer directory of the running exe; fall back to current working directory.
pub fn log_dir() -> PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            return parent.to_path_buf();
        }
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Open session + validation log files (truncate previous run in same folder).
pub fn init() -> PathBuf {
    let dir = log_dir();
    let _ = std::fs::create_dir_all(&dir);
    {
        let mut g = LOG_DIR.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        *g = Some(dir.clone());
    }

    let session_path = dir.join("cubes_session_log.txt");
    let vk_path = dir.join("cubes_vk_validation.txt");

    let open = |path: &Path| {
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
    };

    match open(&session_path) {
        Ok(f) => {
            let mut g = SESSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            *g = Some(f);
        }
        Err(e) => eprintln!(
            "cubes: cannot open session log {}: {e}",
            session_path.display()
        ),
    }
    match open(&vk_path) {
        Ok(f) => {
            let mut g = VK_VAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            *g = Some(f);
        }
        Err(e) => eprintln!("cubes: cannot open vk log {}: {e}", vk_path.display()),
    }

    let unix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    log(&format!(
        "=== cubes session start unix={unix} pid={} ===",
        std::process::id()
    ));
    log(&format!("log_dir={}", dir.display()));
    log(&format!(
        "exe={:?}",
        std::env::current_exe().ok()
    ));
    log(&format!("cwd={:?}", std::env::current_dir().ok()));
    for key in [
        "CUBES_COUNT",
        "CUBES_GPU",
        "CUBES_VALIDATION",
        "WAYLAND_DISPLAY",
        "DISPLAY",
        "XDG_SESSION_TYPE",
        "XDG_CURRENT_DESKTOP",
        "DISABLE_EXPLICIT_SYNC",
        "NW_EXPLICIT_SYNC",
        "VK_LAYER_PATH",
        "VK_INSTANCE_LAYERS",
        "VK_LOADER_DEBUG",
        "MESA_VK_DEVICE_SELECT",
        "AMD_VULKAN_ICD",
        "VK_ICD_FILENAMES",
    ] {
        if let Ok(v) = std::env::var(key) {
            log(&format!("{key}={v}"));
        }
    }

    eprintln!(
        "cubes: logging → {} and {}",
        session_path.display(),
        vk_path.display()
    );
    dir
}

fn write_locked(slot: &Mutex<Option<File>>, line: &str) {
    if let Ok(mut g) = slot.lock() {
        if let Some(f) = g.as_mut() {
            let _ = writeln!(f, "{line}");
            let _ = f.flush();
        }
    }
}

/// Append one line to session log + stderr.
pub fn log(msg: &str) {
    eprintln!("{msg}");
    write_locked(&SESSION, msg);
}

/// Append to session log only (no stderr spam) — e.g. high-frequency FPS.
pub fn log_quiet(msg: &str) {
    write_locked(&SESSION, msg);
}

/// Vulkan validation / messenger line → vk validation file + session file + stderr.
/// (Optional helper; ship path also writes via `set_vk_validation_log_path`.)
#[allow(dead_code, reason = "available for app-side dual-write if needed")]
pub fn log_vk(severity: &str, message: &str) {
    let line = format!("[{severity}] {message}");
    eprintln!("{line}");
    write_locked(&VK_VAL, &line);
    write_locked(&SESSION, &line);
}

pub fn log_error(msg: &str) {
    log(&format!("ERROR: {msg}"));
}

pub fn session_path() -> PathBuf {
    log_dir().join("cubes_session_log.txt")
}

pub fn vk_validation_path() -> PathBuf {
    log_dir().join("cubes_vk_validation.txt")
}
