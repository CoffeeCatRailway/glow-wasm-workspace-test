#![allow(non_snake_case)]
// Build: wasm-pack build --target web
// Run (npn): http-server

use core::add;

use wasm_bindgen::prelude::*;
use web_sys::{console, WebGl2RenderingContext};
// use winit::application::ApplicationHandler;
// use winit::event::WindowEvent;
// use winit::event_loop::{ActiveEventLoop, EventLoop};
// use winit::window::WindowId;

// #[derive(Default)]
// struct App {
// 	gl: Option<glow::Context>,
// }
//
// impl ApplicationHandler for App {
// 	fn resumed(&mut self, _eventLoop: &ActiveEventLoop) {
// 		// let document = web_sys::window().unwrap().document().unwrap();
// 		// let canvas = document.get_element_by_id("canvas").unwrap();
// 		// let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
// 		//
// 		// let webGlContext = canvas.get_context("webgl").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
// 		// self.gl = Some(glow::Context::from_webgl2_context(webGlContext));
// 	}
//
// 	fn window_event(&mut self, eventLoop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
// 		match event {
// 			// WindowEvent::ActivationTokenDone { .. } => {},
// 			// WindowEvent::Resized(_) => {},
// 			// WindowEvent::Moved(_) => {},
// 			WindowEvent::CloseRequested => {
// 				println!("The close button was pressed; stopping");
// 				eventLoop.exit();
// 			},
// 			// WindowEvent::Destroyed => {},
// 			// WindowEvent::DroppedFile(_) => {},
// 			// WindowEvent::HoveredFile(_) => {},
// 			// WindowEvent::HoveredFileCancelled => {},
// 			// WindowEvent::Focused(_) => {},
// 			// WindowEvent::KeyboardInput { .. } => {},
// 			// WindowEvent::ModifiersChanged(_) => {},
// 			// WindowEvent::Ime(_) => {},
// 			// WindowEvent::CursorMoved { .. } => {},
// 			// WindowEvent::CursorEntered { .. } => {},
// 			// WindowEvent::CursorLeft { .. } => {},
// 			// WindowEvent::MouseWheel { .. } => {},
// 			// WindowEvent::MouseInput { .. } => {},
// 			// WindowEvent::PinchGesture { .. } => {},
// 			// WindowEvent::PanGesture { .. } => {},
// 			// WindowEvent::DoubleTapGesture { .. } => {},
// 			// WindowEvent::RotationGesture { .. } => {},
// 			// WindowEvent::TouchpadPressure { .. } => {},
// 			// WindowEvent::AxisMotion { .. } => {},
// 			// WindowEvent::Touch(_) => {},
// 			// WindowEvent::ScaleFactorChanged { .. } => {},
// 			// WindowEvent::ThemeChanged(_) => {},
// 			// WindowEvent::Occluded(_) => {},
// 			// WindowEvent::RedrawRequested => {},
// 			_ => (),
// 		}
// 	}
// }

#[wasm_bindgen(start)]
pub fn mainJs() -> Result<(), JsValue> {
	// console_log::
	println!("Hello, world!");
	println!("1 + 1 = {}", add(2, 2));
	console_error_panic_hook::set_once();
	// console_log::init_with_level(log::Level::Info).unwrap_throw();
	console::log_1(&JsValue::from_str("Hello from Rust!2"));
	
	// let eventLoop = EventLoop::new().unwrap();
	// eventLoop.run_app(&mut App::default()).expect("Failed to run event loop");
	
	let document = web_sys::window().unwrap().document().unwrap();
	let canvas = document.get_element_by_id("canvas").unwrap();
	let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
	
	let webGlContext = canvas.get_context("webgl2")?.unwrap().dyn_into::<WebGl2RenderingContext>()?;
	let gl = Some(glow::Context::from_webgl2_context(webGlContext));
	
	console::log_1(&format!("{:?}", gl).into());
	
	Ok(())
}
