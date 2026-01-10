#![allow(non_snake_case)]

use std::rc::Rc;
use glam::{Mat4, Vec3};
use glow::{Buffer, HasContext, VertexArray, DYNAMIC_DRAW, FLOAT, LINES};
use crate::render::Shader;

pub struct LineRenderer {
    gl: Rc<glow::Context>,
    vec: Vec<f32>,
    shader: Shader,
    vao: VertexArray,
    vbo: Buffer,
    floatsPushed: usize,
    lastFloatsPushed: usize,
    pub enabled: bool,
    destroyed: bool,
}

/*
 * Shader data:
 * - float3 pos1
 * - float3 color1
 * - float3 pos2
 * - float3 color2
 *
 * Floats: 6
 * Bytes: 48
 */
const FLOATS: usize = 6;
const FLOAT_SIZE: usize = size_of::<f32>();
#[cfg(target_arch = "wasm32")]
const SHADER_VERT: &str = include_str!("../../../resources/shaders/web/line_renderer.vert");
#[cfg(target_arch = "wasm32")]
const SHADER_FRAG: &str = include_str!("../../../resources/shaders/web/line_renderer.frag");
#[cfg(not(target_arch = "wasm32"))]
const SHADER_VERT: &str = include_str!("../../../resources/shaders/native/line_renderer.vert");
#[cfg(not(target_arch = "wasm32"))]
const SHADER_FRAG: &str = include_str!("../../../resources/shaders/native/line_renderer.frag");

impl LineRenderer {
    pub fn new(gl: Rc<glow::Context>, capacity: usize) -> Result<Self, String> {
        unsafe {
            let shader = Shader::newVertFrag(gl.clone(), SHADER_VERT, SHADER_FRAG)?;



            let vao = gl.create_vertex_array().map_err(|e| format!("Failed to create vertex array: {}", e))?;
            let vbo = gl.create_named_buffer().map_err(|e| format!("Failed to create buffer object: {}", e))?;
            gl.bind_vertex_array(Some(vao));

            // gl.named_buffer_data_size(vbo, (capacity * FLOAT_SIZE) as i32, DYNAMIC_DRAW);
            let vec = Vec::with_capacity(capacity);
            gl.named_buffer_data_u8_slice(vbo, bytemuck::cast_slice(&vec), DYNAMIC_DRAW);
            gl.vertex_array_vertex_buffer(vao, 0, Some(vbo), 0, (FLOATS * FLOAT_SIZE) as i32);

            let locPos = shader.getAttribLocation("i_position").unwrap();
            let locCol = shader.getAttribLocation("i_color").unwrap();

            let mut offset: usize = 0;
            gl.vertex_array_attrib_format_f32(vao, locPos, 3, FLOAT, false, offset as u32);
            gl.vertex_array_attrib_binding_f32(vao, locPos, 0);
            offset += 3 * FLOAT_SIZE;

            gl.vertex_array_attrib_format_f32(vao, locCol, 3, FLOAT, false, offset as u32);
            gl.vertex_array_attrib_binding_f32(vao, locCol, 0);
            // offset += 3 * FLOAT_SIZE;

            gl.enable_vertex_array_attrib(vao, locPos);
            gl.enable_vertex_array_attrib(vao, locCol);

            Ok(LineRenderer {
                gl,
                vec,
                shader,
                vao,
                vbo,
                floatsPushed: 0,
                lastFloatsPushed: capacity,
                enabled: true,
                destroyed: false,
            })
        }
    }

    pub fn pushLine(&mut self, pos1: Vec3, color1: Vec3, pos2: Vec3, color2: Vec3) {
        if !self.enabled {
            return;
        }
        self.vec.push(pos1.x);
        self.vec.push(pos1.y);
        self.vec.push(pos1.z);
        self.vec.push(color1.x);
        self.vec.push(color1.y);
        self.vec.push(color1.z);

        self.vec.push(pos2.x);
        self.vec.push(pos2.y);
        self.vec.push(pos2.z);
        self.vec.push(color2.x);
        self.vec.push(color2.y);
        self.vec.push(color2.z);

        self.floatsPushed += FLOATS * 2;
    }

    pub fn drawFlush(&mut self, pvMatrix: &Mat4) {
        if self.vec.len() < FLOATS * 2 || self.floatsPushed < FLOATS * 2 {
            return;
        }

        self.shader.bind();
        self.shader.setMatrix4f("u_pvm", pvMatrix);

        unsafe {
            if self.floatsPushed > self.lastFloatsPushed {
                self.gl.named_buffer_data_u8_slice(self.vbo, bytemuck::cast_slice(&self.vec), DYNAMIC_DRAW);
            } else {
                self.gl.named_buffer_sub_data_u8_slice(self.vbo, 0, bytemuck::cast_slice(&self.vec));
            }

            self.gl.bind_vertex_array(Some(self.vao));
            let drawCount = self.vec.len() / FLOATS;
            // println!("drawCount: {}", drawCount);
            self.gl.draw_arrays(LINES, 0, drawCount as i32);
        }

        self.vec.clear();
        self.lastFloatsPushed = self.floatsPushed;
        self.floatsPushed = 0;
    }

    pub fn destroy(&mut self) {
        if self.destroyed {
            return;
        }
        println!("Destroying line renderer");
        self.shader.delete();
        unsafe {
            self.gl.delete_buffer(self.vbo);
            self.gl.delete_vertex_array(self.vao);
        }
        self.destroyed = true;
    }
}

impl Drop for LineRenderer {
    fn drop(&mut self) {
        self.destroy();
    }
}