#![cfg(target_arch = "wasm32")]
#![allow(non_snake_case)]
// Build: wasm-pack build --target web
// Run (npn): http-server
// Run (py3): python -m http.server

use core::TestApp;
use std::rc::Rc;
use log::{debug, error, info, trace, warn};
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};
use winit_input_helper::WinitInputHelper;

use winit::platform::web::WindowAttributesExtWebSys;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct State {
	testApp: TestApp,
}

#[derive(Default)]
struct App {
	window: Option<Rc<Window>>,
	state: Option<State>,
	input: WinitInputHelper,
}

impl ApplicationHandler for App {
	fn new_events(&mut self, _eventLoop: &ActiveEventLoop, _cause: StartCause) {
		self.input.step();
	}
	
	fn resumed(&mut self, eventLoop: &ActiveEventLoop) {
		eventLoop.set_control_flow(ControlFlow::Poll);
		// self.window = Some(web_sys::window().unwrap());
		// let document = self.window.as_ref().unwrap().document().unwrap();
		// let canvas = document.get_element_by_id("canvas").unwrap();
		// let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
		//
		// let webGlContext = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
		// self.gl = Some(glow::Context::from_webgl2_context(webGlContext));

		let webWindow = web_sys::window().unwrap();
		let document = webWindow.document().unwrap();
		let canvas = document.get_element_by_id("canvas").unwrap();
		let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
		canvas.set_width(WIDTH);
		canvas.set_height(HEIGHT);

		let webGlContext = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
		let gl = Rc::new(glow::Context::from_webgl2_context(webGlContext));
		
		// unsafe {
		// 	gl.viewport(0, 0, 800, 600);
		// }

		// #[cfg(target_arch = "wasm32")]
		let attributes = WindowAttributes::default()
			.with_inner_size(PhysicalSize::new(WIDTH, HEIGHT))
			.with_title("CatBox Web")
			.with_canvas(Some(canvas));
		let window = eventLoop.create_window(attributes).unwrap();
		let window = Rc::new(window);
		
		let testApp = TestApp::new(window.clone(), gl.clone());

		self.window = Some(window.clone());
		self.state = Some(State {
			testApp,
		})

		// console::log_1(&format!("{:?}", self.gl).into());
	}

	fn window_event(&mut self, eventLoop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
		self.input.process_window_event(&event);
		match event {
			// WindowEvent::ActivationTokenDone { .. } => {},
			// WindowEvent::Resized(size) => unsafe {
			// 	if let Some(ref mut state) = self.state {
			// 		state.testApp.resize(size.width, size.height);
			// 	}
			// },
			// WindowEvent::Moved(_) => {},
			WindowEvent::CloseRequested => {
				info!("The close button was pressed; stopping");
				eventLoop.exit();
			},
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
					state.testApp.render();
				}

				self.window.as_ref().unwrap().request_redraw();
			},
			_ => (),
		}
	}
	
	fn device_event(&mut self, _eventLoop: &ActiveEventLoop, _id: DeviceId, event: DeviceEvent) {
		self.input.process_device_event(&event);
	}
	
	fn about_to_wait(&mut self, eventLoop: &ActiveEventLoop) {
		self.input.end_step();
		if let Some(ref mut state) = self.state {
			state.testApp.update(self.input.delta_time().unwrap().as_secs_f64(), &self.input, eventLoop);
		}
	}

	fn exiting(&mut self, _eventLoop: &ActiveEventLoop) {
		if let Some(ref mut state) = self.state {
			state.testApp.destroy();
		}
	}
}

#[wasm_bindgen(start)]
pub fn mainJs() -> Result<(), JsValue> {
	console_error_panic_hook::set_once();
	console_log::init_with_level(log::Level::Info).unwrap_throw();
	
	info!("Hello, world!");
	
	error!("This is an error");
	warn!("This is a warning");
	info!("This is info");
	debug!("This is a debug message");
	trace!("This is a trace message");
	
	let eventLoop = EventLoop::new().unwrap();
	eventLoop.run_app(&mut App {
		window: None,
		state: None,
		input: WinitInputHelper::new(),
	}).expect("Failed to run event loop");
	
	// let document = web_sys::window().unwrap().document().unwrap();
	// let canvas = document.get_element_by_id("canvas").unwrap();
	// let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
	//
	// let webGlContext = canvas.get_context("webgl2")?.unwrap().dyn_into::<WebGl2RenderingContext>()?;
	// let gl = Some(glow::Context::from_webgl2_context(webGlContext));
	//
	// console::log_1(&format!("{:?}", gl).into());
	
	Ok(())
}
