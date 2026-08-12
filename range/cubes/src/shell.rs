//! **T.App** — winit bridge · owns T.Hub (PROTOCOL T · FIX-127).
//! Direct cubes demo only (no Viewsor / FramePredictor).

use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

use crate::tandem::MODUL0_TANDEM::{
    assemble_tandem_session, free_tandem, run_tandem_pulse, TandemBfr,
};

const TITLE: &str = "modul/range/cubes · direct";
pub const VIEW_W: u32 = 1280;
pub const VIEW_H: u32 = 720;

struct App {
    window: Option<Arc<Window>>,
    hub: Option<TandemBfr>,
}

pub fn run() {
    let Ok(event_loop) = EventLoop::new() else {
        eprintln!("cubes: failed to create event loop");
        return;
    };
    let mut app = App {
        window: None,
        hub: None,
    };
    if let Err(e) = event_loop.run_app(&mut app) {
        eprintln!("cubes: event loop: {e}");
    }
}

fn shutdown(app: &mut App, event_loop: &ActiveEventLoop) {
    if let Some(mut hub) = app.hub.take() {
        free_tandem(&mut hub);
    }
    event_loop.exit();
}

/// Rebuild Vulkan session after resize (swapchain extent is fixed at assemble time).
fn rebuild_session(app: &mut App) {
    let Some(window) = app.window.clone() else {
        return;
    };
    if let Some(mut hub) = app.hub.take() {
        free_tandem(&mut hub);
    }
    match assemble_tandem_session(&window) {
        Ok(hub) => {
            app.hub = Some(hub);
            window.request_redraw();
        }
        Err(e) => eprintln!("cubes: rebuild after resize failed: {e}"),
    }
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
                eprintln!("cubes: create_window: {e}");
                event_loop.exit();
                return;
            }
        };
        let _ = window.request_inner_size(target);
        match assemble_tandem_session(&window) {
            Ok(hub) => {
                self.hub = Some(hub);
                self.window = Some(window);
                if let Some(w) = self.window.as_ref() {
                    w.request_redraw();
                }
            }
            Err(e) => {
                eprintln!("assemble_tandem_session fail: {e}");
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
                // Ignore minimize / zero-size. Skip rebuild when extent matches current
                // swapchain (avoids double-assemble from request_inner_size on boot).
                if size.width == 0 || size.height == 0 {
                    return;
                }
                let Some(hub) = self.hub.as_ref() else {
                    return;
                };
                let cur = hub.presentation_rt.swapchain_default_rt_pkg.extent_rt;
                if size.width == cur.width && size.height == cur.height {
                    return;
                }
                rebuild_session(self);
            }
            WindowEvent::RedrawRequested => {
                if let Some(hub) = self.hub.as_mut() {
                    if let Err(e) = run_tandem_pulse(hub) {
                        eprintln!("run_tandem_pulse: {e}");
                    }
                    if hub.fps_sample_ready {
                        if let Some(w) = self.window.as_ref() {
                            w.set_title(&format!(
                                "{TITLE} · {fps:.0} fps · {n} cubes",
                                fps = hub.fps,
                                n = hub.mesh_gpu_rt.instance_count_rt
                            ));
                        }
                        hub.fps_sample_ready = false;
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
                    // Scroll up (dy > 0) → zoom in (closer). radius ∝ 1/zoom.
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
