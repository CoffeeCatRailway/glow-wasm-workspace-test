#![allow(non_snake_case)]
/* Based on https://github.com/bwasty/learn-opengl-rs/blob/master/src/camera.rs */

use glam::{Mat4, Vec3};

#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Movement {
	Up,
	Down,
	Forward,
	Backward,
	Left,
	Right,
}
use self::Movement::*;

pub struct Camera {
	pub pos: Vec3,
	pub front: Vec3,
	pub up: Vec3,
	pub right: Vec3,
	pub worldUp: Vec3,
	
	pub yaw: f32,
	pub pitch: f32,
	
	pub speed: f32,
	pub sensitivity: f32,
	pub fov: f32,
}

impl Default for Camera {
	fn default() -> Camera {
		let mut camera = Camera {
			pos: Vec3::ZERO,
			front: Vec3::NEG_Z,
			up: Vec3::ZERO,
			right: Vec3::ZERO,
			worldUp: Vec3::Y,
			
			yaw: -90.0,
			pitch: 0.0,
			
			speed: 2.5,
			sensitivity: 0.1,
			fov: 45.0,
		};
		camera.updateVectors();
		camera
	}
}

#[allow(dead_code)]
impl Camera {
	pub fn getViewMatrix(&self) -> Mat4 {
		Mat4::look_at_rh(self.pos, self.pos + self.front, self.up)
	}
	
	pub fn processMovement(&mut self, dir: Movement, dt: f32) {
		let speed = self.speed * dt;
		match dir {
			Up => {self.pos += self.worldUp * speed;}
			Down => {self.pos -= self.worldUp * speed;}
			Forward => {self.pos += self.front * speed;}
			Backward => {self.pos -= self.front * speed;}
			Left => {self.pos -= self.right * speed;}
			Right => {self.pos += self.right * speed;}
		}
	}
	
	pub fn processMouseMovement(&mut self, mut xo: f32, mut yo: f32, constrainPitch: bool) {
		xo *= self.sensitivity;
		yo *= self.sensitivity;
		
		self.yaw += xo;
		self.pitch += yo;
		
		self.yaw = self.yaw % 360.0;
		
		if constrainPitch {
			self.pitch = self.pitch.clamp(-89.0, 89.0);
		}
		
		self.updateVectors();
	}
	
	pub fn processMouseScroll(&mut self, yo: f32) {
		self.fov = (self.fov - yo).clamp(1.0, 45.0);
	}
	
	fn updateVectors(&mut self) {
		let front = Vec3 {
			x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
			y: self.pitch.to_radians().sin(),
			z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
		};
		self.front = front;
		self.right = self.front.cross(self.worldUp).normalize();
		self.up = self.right.cross(self.front).normalize();
	}
}
