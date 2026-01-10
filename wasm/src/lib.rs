#![allow(non_snake_case)]
// Build: wasm-pack build --target web
// Run (npn): http-server
// Run (py3): python -m http.server

use std::rc::Rc;
use glam::{vec3, Mat4};
use glow::{HasContext, COLOR_BUFFER_BIT};
use core::render::LineRenderer;
use wasm_bindgen::prelude::*;
use web_sys::{console, WebGl2RenderingContext};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowAttributesExtWebSys;

struct State {
	gl: Rc<glow::Context>,
	lineRenderer: LineRenderer,
}

#[derive(Default)]
struct App {
	window: Option<Window>,
	state: Option<State>,
}

impl ApplicationHandler for App {
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

		let webGlContext = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
		let gl = Rc::new(glow::Context::from_webgl2_context(webGlContext));

		// #[cfg(target_arch = "wasm32")]
		let attributes = WindowAttributes::default()
			.with_title("CatBox Web")
			.with_canvas(Some(canvas));
		let window = eventLoop.create_window(attributes).unwrap();

		let lineRenderer = LineRenderer::new(gl.clone(), 1024).unwrap();

		self.window = Some(window);
		self.state = Some(State {
			gl,
			lineRenderer,
		})

		// console::log_1(&format!("{:?}", self.gl).into());
	}

	fn window_event(&mut self, eventLoop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
		match event {
			// WindowEvent::ActivationTokenDone { .. } => {},
			// WindowEvent::Resized(_) => {},
			// WindowEvent::Moved(_) => {},
			WindowEvent::CloseRequested => {
				println!("The close button was pressed; stopping");
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
					unsafe {
						state.gl.clear(COLOR_BUFFER_BIT);
						state.gl.clear_color(0.0, 0.0, 0.0, 1.0);
					}

					let mat = Mat4::IDENTITY;
					state.lineRenderer.drawFlush(&mat);
				}

				self.window.as_ref().unwrap().request_redraw();
			},
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

#[wasm_bindgen(start)]
pub fn mainJs() -> Result<(), JsValue> {
	// console_log::
	println!("Hello, world!");
	console_error_panic_hook::set_once();
	console_log::init_with_level(log::Level::Info).unwrap_throw();
	console::log_1(&JsValue::from_str("Hello from Rust!"));
	
	let eventLoop = EventLoop::new().unwrap();
	eventLoop.run_app(&mut App::default()).expect("Failed to run event loop");
	
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
