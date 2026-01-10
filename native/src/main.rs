#![allow(non_snake_case)]

use core::render::LineRenderer;
use glam::{vec3, Mat4};
use glow::{COLOR_BUFFER_BIT, HasContext};
use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SwapInterval, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::num::NonZeroU32;
use std::rc::Rc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

struct State {
    gl: Rc<glow::Context>,
    glSurface: Surface<WindowSurface>,
    glContext: PossiblyCurrentContext,
    lineRenderer: LineRenderer,
}

#[derive(Default)]
struct App {
    window: Option<Window>,
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, eventLoop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }
        eventLoop.set_control_flow(ControlFlow::Poll);

        let attributes = WindowAttributes::default()
            .with_inner_size(LogicalSize::new(800, 600))
            .with_title("CatBox Native");

        let template = ConfigTemplateBuilder::new();
        let displayBuilder = DisplayBuilder::new().with_window_attributes(Some(attributes));

        let (window, glConfig) = displayBuilder
            .build(eventLoop, template, |configs| {
                configs
                    .reduce(|accum, config| {
                        if config.num_samples() > accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();
        let rwh: Option<RawWindowHandle> = window
            .as_ref()
            .and_then(|w| w.window_handle().map(Into::into).ok());

        let glDisplay = glConfig.display();
        let contextAttributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(glutin::context::Version {
                major: 4,
                minor: 6,
            })))
            .build(rwh);

        let (window, gl, glSurface, glContext) = unsafe {
            let notCurrentGlContext = glDisplay
                .create_context(&glConfig, &contextAttributes)
                .unwrap();
            let window = window.unwrap();

            let surfaceAttributes = window.build_surface_attributes(Default::default()).unwrap();
            let glSurface = glDisplay
                .create_window_surface(&glConfig, &surfaceAttributes)
                .unwrap();

            let glContext = notCurrentGlContext.make_current(&glSurface).unwrap();
            let gl = glow::Context::from_loader_function_cstr(|s| glDisplay.get_proc_address(s));
            glSurface
                .set_swap_interval(&glContext, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
                .unwrap();

            (window, Rc::new(gl), glSurface, glContext)
        };

        let lineRenderer = LineRenderer::new(gl.clone(), 1024).unwrap();

        self.window = Some(window);
        self.state = Some(State {
            gl,
            glSurface,
            glContext,
            lineRenderer,
        });

        // self.window = Some(eventLoop.create_window(Window::default_attributes()
        // 	.with_inner_size(LogicalSize::new(800, 600))
        // 	.with_title("CatBox Native")
        // ).unwrap());
    }

	fn window_event(&mut self, eventLoop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
		match event {
			// WindowEvent::ActivationTokenDone { .. } => {},
			// WindowEvent::Resized(_) => {},
			// WindowEvent::Moved(_) => {},
			WindowEvent::CloseRequested => {
				println!("The close button was pressed; stopping");
				eventLoop.exit();
			}
			// WindowEvent::Destroyed => {},
			// WindowEvent::DroppedFile(_) => {},
			// WindowEvent::HoveredFile(_) => {},
			// WindowEvent::HoveredFileCancelled => {},
			// WindowEvent::Focused(_) => {},
			// WindowEvent::KeyboardInput { .. } => {},
			// WindowEvent::ModifiersChanged(_) => {},
			// WindowEvent::Ime(_) => {},
			// WindowEvent::CursorMoved { .. } => {},
			// WindowEvent::CursorEntered { .. } => {},
			// WindowEvent::CursorLeft { .. } => {},
			// WindowEvent::MouseWheel { .. } => {},
			// WindowEvent::MouseInput { .. } => {},
			// WindowEvent::PinchGesture { .. } => {},
			// WindowEvent::PanGesture { .. } => {},
			// WindowEvent::DoubleTapGesture { .. } => {},
			// WindowEvent::RotationGesture { .. } => {},
			// WindowEvent::TouchpadPressure { .. } => {},
			// WindowEvent::AxisMotion { .. } => {},
			// WindowEvent::Touch(_) => {},
			// WindowEvent::ScaleFactorChanged { .. } => {},
			// WindowEvent::ThemeChanged(_) => {},
			// WindowEvent::Occluded(_) => {},
			WindowEvent::RedrawRequested => {
				if let Some(ref mut state) = self.state {
					unsafe {
						state.gl.clear(COLOR_BUFFER_BIT);
						state.gl.clear_color(0.0, 0.0, 0.0, 1.0);
					}

					let mat = Mat4::IDENTITY;
					state.lineRenderer.drawFlush(&mat);

					state.glSurface.swap_buffers(&state.glContext).unwrap();
				}

				// self.window.as_ref().unwrap().request_redraw();
			}
			_ => (),
		}
	}

	fn about_to_wait(&mut self, _eventLoop: &ActiveEventLoop) {
        if let Some(ref mut state) = self.state {
            let red = vec3(1.0, 0.0, 0.0);
            let green = vec3(0.0, 1.0, 0.0);
            let blue = vec3(0.0, 0.0, 1.0);
            let white = vec3(1.0, 1.0, 1.0);
            let p1 = vec3(0.0, 1.0, 0.0);
            let p2 = vec3(1.0, 0.0, 0.0);
            let p3 = vec3(0.0, -1.0, 0.0);
            let p4 = vec3(-1.0, 0.0, 0.0);
            state.lineRenderer.pushLine(p1, red, p2, green);
            state.lineRenderer.pushLine(p2, green, p3, blue);
            state.lineRenderer.pushLine(p3, blue, p4, white);
            state.lineRenderer.pushLine(p4, white, p1, red);
        }
    }

	fn exiting(&mut self, _eventLoop: &ActiveEventLoop) {
		if let Some(ref mut state) = self.state {
			state.lineRenderer.destroy();
		}
	}
}

fn main() {
    println!("Hello, world!");

    let eventLoop = EventLoop::new().unwrap();
    eventLoop
        .run_app(&mut App::default())
        .expect("Failed to run event loop");
}
