//! **T.App** — winit bridge · owns T.Hub (PROTOCOL T · FIX-127).
//! Ship: logs to cubes_session_log.txt · resize recreates presentation only (not full re-boot).

use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

use crate::tandem::proc::session_log;
use crate::tandem::proc::sys_stats::CpuSampler;
use crate::tandem::proc::{
    assemble_tandem_session, free_tandem, recreate_presentation_extent, run_tandem_pulse,
};
use modul::tandem::MODUL0_TANDEM::TandemBfr;

const TITLE: &str = "modul cubes · auto";
pub const VIEW_W: u32 = 1280;
pub const VIEW_H: u32 = 720;

struct App {
    window: Option<Arc<Window>>,
    hub: Option<TandemBfr>,
    /// Consecutive presentation recreate failures (stop thrashing after N).
    recreate_fails: u32,
    /// Coalesce resize storms: apply last size once per redraw.
    pending_extent: Option<(u32, u32)>,
    /// Process + system CPU% (sampled with FPS).
    cpu: CpuSampler,
}

pub fn run() {
    let Ok(event_loop) = EventLoop::new() else {
        session_log::log_error(
            "failed to create event loop (display server / Wayland / X11 missing?)",
        );
        session_log::log("On headless machines this demo cannot open a window.");
        return;
    };
    let mut app = App {
        window: None,
        hub: None,
        recreate_fails: 0,
        pending_extent: None,
        cpu: CpuSampler::new(),
    };
    if let Err(e) = event_loop.run_app(&mut app) {
        session_log::log_error(&format!("event loop: {e}"));
    }
}

fn shutdown(app: &mut App, event_loop: &ActiveEventLoop) {
    if let Some(mut hub) = app.hub.take() {
        free_tandem(&mut hub);
        session_log::log("free_tandem done");
    }
    event_loop.exit();
}

/// Recreate swapchain + FB/depth/MSAA only (mesh/device/instance kept).
fn apply_presentation_recreate(app: &mut App, width: u32, height: u32, reason: &str) {
    let Some(hub) = app.hub.as_mut() else {
        return;
    };
    let cur = hub.presentation_rt.swapchain_default_rt_pkg.extent_rt;
    if width == cur.width && height == cur.height && !reason.contains("OUT_OF_DATE") {
        return;
    }
    session_log::log(&format!(
        "recreate presentation · {reason} · {}x{} → {width}x{height}",
        cur.width, cur.height
    ));
    match recreate_presentation_extent(hub, width, height) {
        Ok(()) => {
            let ext = hub.presentation_rt.swapchain_default_rt_pkg.extent_rt;
            session_log::log(&format!(
                "recreate presentation ok · {}x{} · mesh kept n={}",
                ext.width, ext.height, hub.mesh_gpu_rt.instance_count_rt
            ));
            app.recreate_fails = 0;
            if let Some(w) = app.window.as_ref() {
                w.request_redraw();
            }
        }
        Err(e) => {
            app.recreate_fails = app.recreate_fails.saturating_add(1);
            session_log::log_error(&format!(
                "recreate presentation failed (#{}) : {e}",
                app.recreate_fails
            ));
            if app.recreate_fails >= 8 {
                session_log::log_error("too many recreate failures — stopping redraw loop");
            }
        }
    }
}

/// Flush coalesced resize, if any.
fn flush_pending_resize(app: &mut App) {
    let Some((w, h)) = app.pending_extent.take() else {
        return;
    };
    apply_presentation_recreate(app, w, h, "window Resized");
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let target = PhysicalSize::new(VIEW_W, VIEW_H);
        let attrs = Window::default_attributes()
            .with_title(TITLE)
            .with_inner_size(target)
            .with_resizable(true);
        let window = match event_loop.create_window(attrs) {
            Ok(w) => Arc::new(w),
            Err(e) => {
                session_log::log_error(&format!("create_window: {e}"));
                event_loop.exit();
                return;
            }
        };
        let size = window.inner_size();
        session_log::log(&format!(
            "window created · requested {VIEW_W}x{VIEW_H} · actual {}x{}",
            size.width, size.height
        ));
        match assemble_tandem_session(&window) {
            Ok(hub) => {
                session_log::log(&format!(
                    "session ready · {} cubes · fps+cpu in title + cubes_session_log.txt",
                    hub.mesh_gpu_rt.instance_count_rt
                ));
                self.hub = Some(hub);
                self.window = Some(window);
                if let Some(w) = self.window.as_ref() {
                    w.request_redraw();
                }
            }
            Err(e) => {
                session_log::log("============================================================");
                session_log::log_error("Vulkan session failed to start");
                session_log::log(&e);
                session_log::log("------------------------------------------------------------");
                session_log::log("Typical fixes:");
                session_log::log("  • Install a Vulkan GPU driver (mesa-vulkan-drivers / vendor)");
                session_log::log("  • Check:  vulkaninfo --summary");
                session_log::log("  • Wayland issues: try  WAYLAND_DISPLAY= ./cubes  (force X11)");
                session_log::log("  • Desktop session with GPU (not pure SSH without display)");
                session_log::log(&format!(
                    "  • Send {} and {} to the author",
                    session_log::session_path().display(),
                    session_log::vk_validation_path().display()
                ));
                session_log::log("============================================================");
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => shutdown(self, event_loop),
            WindowEvent::KeyboardInput { event, .. }
                if event.state == winit::event::ElementState::Pressed
                    && matches!(event.physical_key, PhysicalKey::Code(KeyCode::Escape)) =>
            {
                shutdown(self, event_loop);
            }
            WindowEvent::Resized(size) => {
                if size.width == 0 || size.height == 0 {
                    session_log::log_quiet(&format!(
                        "resize ignored zero {}x{}",
                        size.width, size.height
                    ));
                    return;
                }
                // Coalesce storm: only last size is applied on next redraw.
                self.pending_extent = Some((size.width, size.height));
                if let Some(w) = self.window.as_ref() {
                    w.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if self.recreate_fails >= 8 {
                    return;
                }
                flush_pending_resize(self);
                if self.recreate_fails >= 8 {
                    return;
                }
                if let Some(hub) = self.hub.as_mut() {
                    match run_tandem_pulse(hub) {
                        Ok(()) => {}
                        Err(e) if e.contains("OUT_OF_DATE") || e.contains("SUBOPTIMAL") => {
                            session_log::log(&format!("present/acquire needs recreate: {e}"));
                            let size = self.window.as_ref().map_or(
                                PhysicalSize::new(VIEW_W, VIEW_H),
                                |w| w.inner_size(),
                            );
                            apply_presentation_recreate(
                                self,
                                size.width.max(1),
                                size.height.max(1),
                                "OUT_OF_DATE_KHR",
                            );
                            return;
                        }
                        Err(e) => {
                            session_log::log_error(&format!("run_tandem_pulse: {e}"));
                        }
                    }
                    if let Some(hub) = self.hub.as_mut() {
                        if hub.fps_sample_ready {
                            self.cpu.sample();
                            let fps = hub.fps;
                            let inst = hub.fps_instant;
                            let n = hub.mesh_gpu_rt.instance_count_rt;
                            let ext = hub.presentation_rt.swapchain_default_rt_pkg.extent_rt;
                            let cpu_p = self.cpu.process_pct;
                            let cpu_s = self.cpu.system_pct;
                            // Visible in stderr + cubes_session_log.txt (~2 Hz with 0.5s window).
                            session_log::log(&format!(
                                "FPS · {fps:.1} · instant={inst:.1} · cpu_proc={cpu_p:.0}% · cpu_sys={cpu_s:.0}% · n={n} · {}x{}",
                                ext.width, ext.height
                            ));
                            if let Some(w) = self.window.as_ref() {
                                w.set_title(&format!(
                                    "{TITLE} · {fps:.0} fps · cpu {cpu_p:.0}% · {n} cubes"
                                ));
                            }
                            hub.fps_sample_ready = false;
                        }
                    }
                    if let Some(w) = self.window.as_ref() {
                        w.request_redraw();
                    }
                }
            }
            WindowEvent::MouseInput {
                state: winit::event::ElementState::Pressed,
                button: winit::event::MouseButton::Left,
                ..
            } => {
                if let Some(hub) = self.hub.as_mut() {
                    hub.dragging = true;
                }
            }
            WindowEvent::MouseInput {
                state: winit::event::ElementState::Released,
                button: winit::event::MouseButton::Left,
                ..
            } => {
                if let Some(hub) = self.hub.as_mut() {
                    hub.dragging = false;
                    hub.last_cursor = None;
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                if let Some(hub) = self.hub.as_mut() {
                    if hub.dragging {
                        if let Some((lx, ly)) = hub.last_cursor {
                            hub.orbit_yaw =
                                ((position.x - lx) as f32).mul_add(0.005, hub.orbit_yaw);
                            hub.orbit_pitch = ((position.y - ly) as f32)
                                .mul_add(0.005, hub.orbit_pitch)
                                .clamp(-1.4, 1.4);
                        }
                        hub.last_cursor = Some((position.x, position.y));
                    }
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                if let Some(hub) = self.hub.as_mut() {
                    let dy = match delta {
                        winit::event::MouseScrollDelta::LineDelta(_, y) => y,
                        winit::event::MouseScrollDelta::PixelDelta(p) => p.y as f32 * 0.01,
                    };
                    hub.zoom = (hub.zoom * (1.0 + dy * 0.08)).clamp(0.3, 4.0);
                }
            }
            _ => {}
        }
    }
}
