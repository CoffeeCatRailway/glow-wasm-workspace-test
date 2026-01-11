#![allow(non_snake_case)]

use std::rc::Rc;
use glam::{vec3, Mat4};
use glow::{HasContext, COLOR_BUFFER_BIT};
use crate::render::LineRenderer;

pub struct TestApp {
	gl: Rc<glow::Context>,
	lineRenderer: LineRenderer,
}

impl TestApp {
	pub fn new(gl: Rc<glow::Context>) -> Self {
		let lineRenderer = LineRenderer::new(gl.clone(), 1024).unwrap();
		
		TestApp {
			gl,
			lineRenderer,
		}
	}
	
	pub fn resize(&mut self, width: u32, height: u32) {
		unsafe {
			self.gl.viewport(0, 0, width as i32, height as i32);
		}
	}
	
	pub fn update(&mut self) {
		let red = vec3(1.0, 0.0, 0.0);
		let green = vec3(0.0, 1.0, 0.0);
		let blue = vec3(0.0, 0.0, 1.0);
		let white = vec3(1.0, 1.0, 1.0);
		let p1 = vec3(0.0, 1.0, 0.0);
		let p2 = vec3(1.0, 0.0, 0.0);
		let p3 = vec3(0.0, -1.0, 0.0);
		let p4 = vec3(-1.0, 0.0, 0.0);
		self.lineRenderer.pushLine(p1, red, p2, green);
		self.lineRenderer.pushLine(p2, green, p3, blue);
		self.lineRenderer.pushLine(p3, blue, p4, white);
		self.lineRenderer.pushLine(p4, white, p1, red);
	}
	
	pub fn render(&mut self) {
		unsafe {
			self.gl.clear(COLOR_BUFFER_BIT);
			self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
		}
		
		let mat = Mat4::IDENTITY;
		self.lineRenderer.drawFlush(&mat);
	}
	
	pub fn destroy(&mut self) {
		self.lineRenderer.destroy();
	}
}
