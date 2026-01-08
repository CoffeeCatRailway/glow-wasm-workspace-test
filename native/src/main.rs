#![allow(non_snake_case)]

use core::add;

use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct App {
	window: Option<Window>,
}

impl ApplicationHandler for App {
	fn resumed(&mut self, eventLoop: &ActiveEventLoop) {
		self.window = Some(eventLoop.create_window(Window::default_attributes()
			.with_inner_size(LogicalSize::new(800, 600))
			.with_title("Hello world!")
		).unwrap());
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
				self.window.as_ref().unwrap().request_redraw();
			},
			_ => (),
		}
	}
}

fn main() {
    println!("Hello, world!");
	println!("1 + 1 = {}", add(2, 2));
	
	let eventLoop = EventLoop::new().unwrap();
	eventLoop.set_control_flow(ControlFlow::Poll);
	eventLoop.run_app(&mut App::default()).expect("Failed to run event loop");
}
