#![allow(non_snake_case)]

use core::TestApp;
use glow::HasContext;
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
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};
use winit_input_helper::WinitInputHelper;

struct State {
    glSurface: Surface<WindowSurface>,
    glContext: PossiblyCurrentContext,
    testApp: TestApp,
}

#[derive(Default)]
struct App {
    window: Option<Window>,
    state: Option<State>,
	input: WinitInputHelper,
}

impl ApplicationHandler for App {
	fn new_events(&mut self, _eventLoop: &ActiveEventLoop, _cause: StartCause) {
		self.input.step();
	}
	
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
				minor: 1,
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
			let gl = Rc::new(glow::Context::from_loader_function_cstr(|s| glDisplay.get_proc_address(s)));
			glSurface.set_swap_interval(&glContext, SwapInterval::Wait(NonZeroU32::new(1).unwrap())).unwrap();
			// glSurface.set_swap_interval(&glContext, SwapInterval::DontWait).unwrap();
			
			gl.viewport(0, 0, 800, 600);

			(window, gl, glSurface, glContext)
		};

		let testApp = TestApp::new(gl.clone(), (800, 600));

		self.window = Some(window);
		self.state = Some(State {
			glSurface,
			glContext,
			testApp,
		});

		// self.window = Some(eventLoop.create_window(Window::default_attributes()
		// 	.with_inner_size(LogicalSize::new(800, 600))
		// 	.with_title("CatBox Native")
		// ).unwrap());
	}
	
	fn window_event(&mut self, eventLoop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
		self.input.process_window_event(&event);
		match event {
			// WindowEvent::ActivationTokenDone { .. } => {},
			WindowEvent::Resized(size) => {
				if let Some(ref mut state) = self.state {
					state.testApp.resize(size.width, size.height);
				}
			},
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
			// WindowEvent::KeyboardInput {..} => {},
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
					state.testApp.render();
					state.glSurface.swap_buffers(&state.glContext).unwrap();
				}

				self.window.as_ref().unwrap().request_redraw();
			},
			_ => (),
		}
	}
	
	fn device_event(&mut self, _eventLoop: &ActiveEventLoop, _id: DeviceId, event: DeviceEvent) {
		self.input.process_device_event(&event);
	}

	fn about_to_wait(&mut self, _eventLoop: &ActiveEventLoop) {
		self.input.end_step();
        if let Some(ref mut state) = self.state {
            state.testApp.update(self.input.delta_time().unwrap().as_secs_f64(), &self.input);
        }
    }

	fn exiting(&mut self, _eventLoop: &ActiveEventLoop) {
		if let Some(ref mut state) = self.state {
			state.testApp.destroy();
		}
	}
}

fn main() {
    println!("Hello, world!");

    let eventLoop = EventLoop::new().unwrap();
    eventLoop
        .run_app(&mut App {
			window: None,
			state: None,
			input: WinitInputHelper::new(),
		}).expect("Failed to run event loop");
}
