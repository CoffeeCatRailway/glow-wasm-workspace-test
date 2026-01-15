#![allow(non_snake_case)]

use std::rc::Rc;
use glam::{vec3, Mat4, Vec3};
use glow::*;
use log::info;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
use winit::window::Window;
use winit_input_helper::WinitInputHelper;
use crate::camera::{Camera, Movement};
use crate::render::LineRenderer;

pub struct TestApp {
	window: Rc<Window>,
	gl: Rc<Context>,
	camera: Camera,
	lineRenderer: LineRenderer,

	mouseCaptured: bool,
}

fn norm(v: Vec3) -> Vec3 {
	v * 0.5 + 0.5
}

impl TestApp {
	pub fn new(window: Rc<Window>, gl: Rc<Context>) -> Self {
		unsafe {
			gl.enable(DEPTH_TEST);
			gl.polygon_mode(FRONT_AND_BACK, FILL);
		}
		
		let camera = Camera {
			pos: Vec3::new(0.0, 0.0, 5.0),
			..Camera::default()
		};
		
		let lineRenderer = LineRenderer::new(gl.clone(), 1024).unwrap();
		
		TestApp {
			window,
			gl,
			camera,
			lineRenderer,
			
			// windowSize: I16Vec2::new(width, height),
			mouseCaptured: false,
		}
	}

	#[allow(unused)]
	pub fn resize(&mut self, width: u32, height: u32) {
		// Not needed on linux for some reason?
		// Tested on: Arch Wayland
		#[cfg(not(target_os = "linux"))]
		unsafe {
			// info!("{:?}", self.window.inner_size());
			// info!("{} {}", width, height);
			self.gl.viewport(0, 0, width as i32, height as i32);
			// self.gl.viewport(0, 0, self.window.inner_size().width as i32, self.window.inner_size().height as i32);
		}
	}

	#[allow(unused)]
	pub fn update(&mut self, dt: f64, input: &WinitInputHelper, eventLoop: &ActiveEventLoop) {
		// Don't allow user to escape loop in web env
		#[cfg(not(target_arch = "wasm32"))]
		if input.key_pressed(KeyCode::Escape) {
			eventLoop.exit();
		}

		if input.key_pressed(KeyCode::Digit1) {
			self.mouseCaptured = !self.mouseCaptured;
			info!("mouseCaptured: {}", self.mouseCaptured);

			// Confines to window, setting cursor pos is only allowed in Locked for wayland
			// #[cfg(not(target_arch = "wasm32"))]
			// if self.mouseCaptured {
			// 	self.window.set_cursor_grab(CursorGrabMode::Confined).expect("Failed to confine cursor");
			// 	self.window.set_cursor_visible(false);
			// } else {
			// 	self.window.set_cursor_grab(CursorGrabMode::None).expect("Failed to unconfine cursor");
			// 	self.window.set_cursor_visible(true);
			// }
		}
		
		if input.key_held(KeyCode::KeyW) {
			self.camera.processMovement(Movement::Forward, dt as f32);
		}
		if input.key_held(KeyCode::KeyS) {
			self.camera.processMovement(Movement::Backward, dt as f32);
		}
		if input.key_held(KeyCode::KeyA) {
			self.camera.processMovement(Movement::Left, dt as f32);
		}
		if input.key_held(KeyCode::KeyD) {
			self.camera.processMovement(Movement::Right, dt as f32);
		}
		if input.key_held(KeyCode::Space) {
			self.camera.processMovement(Movement::Up, dt as f32);
		}
		if input.key_held(KeyCode::ShiftLeft) {
			self.camera.processMovement(Movement::Down, dt as f32);
		}
		
		if self.mouseCaptured {
			info!("{:?}", input.cursor_diff());
			self.camera.processMouseScroll(input.scroll_diff().1);
			self.camera.processMouseMovement(input.cursor_diff().0, -input.cursor_diff().1, true);
		}
		
		// let white = vec3(1.0, 1.0, 1.0);
		// let red = vec3(1.0, 0.0, 0.0);
		// let green = vec3(0.0, 1.0, 0.0);
		// let blue = vec3(0.0, 0.0, 1.0);
		
		let b1 = vec3(-1.0, -1.0, -1.0);
		let b2 = vec3(1.0, -1.0, -1.0);
		let b3 = vec3(1.0, -1.0, 1.0);
		let b4 = vec3(-1.0, -1.0, 1.0);
		let t1 = vec3(-1.0, 1.0, -1.0);
		let t2 = vec3(1.0, 1.0, -1.0);
		let t3 = vec3(1.0, 1.0, 1.0);
		let t4 = vec3(-1.0, 1.0, 1.0);
		
		self.lineRenderer.pushLine(b1, norm(b1), b2, norm(b2));
		self.lineRenderer.pushLine(b2, norm(b2), b3, norm(b3));
		self.lineRenderer.pushLine(b3, norm(b3), b4, norm(b4));
		self.lineRenderer.pushLine(b4, norm(b4), b1, norm(b1));
		
		self.lineRenderer.pushLine(t1, norm(t1), t2, norm(t2));
		self.lineRenderer.pushLine(t2, norm(t2), t3, norm(t3));
		self.lineRenderer.pushLine(t3, norm(t3), t4, norm(t4));
		self.lineRenderer.pushLine(t4, norm(t4), t1, norm(t1));
		
		self.lineRenderer.pushLine(b1, norm(b1), t1, norm(t1));
		self.lineRenderer.pushLine(b2, norm(b2), t2, norm(t2));
		self.lineRenderer.pushLine(b3, norm(b3), t3, norm(t3));
		self.lineRenderer.pushLine(b4, norm(b4), t4, norm(t4));
	}
	
	pub fn render(&mut self) {
		unsafe {
			self.gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
			self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
		}

		// let aspect = self.windowSize.x as f32 / self.windowSize.y as f32;
		let aspect = self.window.inner_size().width as f32 / self.window.inner_size().height as f32;
		let projection = Mat4::perspective_rh(self.camera.fov.to_radians(), aspect, 0.1, 100.0);
		let view = self.camera.getViewMatrix();
		let pvm = projection * view;
		self.lineRenderer.drawFlush(&pvm);
	}
	
	pub fn destroy(&mut self) {
		self.lineRenderer.destroy();
	}
}
