use std::ptr;

use gl::types::{GLbitfield, GLenum};

use crate::rendering::rgl::*;

pub mod window;
pub mod rgl;
pub mod texture;

pub struct Renderer {
    vao: Vao,
    vertex_buffer: BufferObject,
    index_buffer: BufferObject,
    attributes: Vec<VertexAttribute>
}

impl Renderer {
    pub fn new() -> Renderer {
        let vao = Vao::new();
        let vertex_buffer = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
        let index_buffer = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
        let attributes = Vec::new();

        Renderer { vao, vertex_buffer, index_buffer, attributes }
    }

    pub fn with_attribute(&mut self, attribute: VertexAttribute) {
        self.attributes.push(attribute);
    }

    pub fn remove_attributes(&mut self) {
        self.attributes.clear();
    }

    pub fn enable_attributes(&self) {
        for attribute in &self.attributes {
            attribute.enable();
        }
    }

    pub fn disable_attributes(&self) {
        for attribute in &self.attributes {
            attribute.disable();
        }
    }

    pub fn bind(&self) {
        self.vao.bind();
        self.vertex_buffer.bind();
        self.index_buffer.bind();
    }

    pub fn unbind(&self) {
        self.vao.unbind();
        self.vertex_buffer.unbind();
        self.index_buffer.unbind();
    }

    pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }

    pub fn clear(mask: GLbitfield) {
        unsafe {
            gl::Clear(mask);
        }
    }

    pub fn draw_arrays(&self, draw_mode: GLenum, vertices: Vec<f32>, count: i32) {
        self.vertex_buffer.store_f32(vertices.as_slice());

        unsafe {
            gl::DrawArrays(draw_mode, 0, count);
        }
    }

    pub fn draw_elements(&self, draw_mode: GLenum, vertices: Vec<f32>, indices: Vec<i32>, count: i32) {
        self.vertex_buffer.store_f32(vertices.as_slice());
        self.index_buffer.store_i32(indices.as_slice());

        unsafe {
            gl::DrawElements(draw_mode, count, gl::UNSIGNED_INT, ptr::null())
        }
    }
}

pub struct VertexFormat<T> {
    attributes: Vec<VertexAttribute>,
    convert_to_f32v: Box<dyn Fn(T) -> Vec<f32>>,
}

impl <T> VertexFormat<T> {
    pub fn new(attributes: Vec<VertexAttribute>, convert_to_f32v: Box<dyn Fn(T) -> Vec<f32>>) -> Box<VertexFormat<T>> {
        Box::new(VertexFormat { attributes, convert_to_f32v })
    }

    pub fn convert_to_vertices(&self, to_verts: Vec<T>) -> Vec<f32> {
        let mut buffer = Vec::new();

        for to_vert in to_verts {
            buffer.append(&mut (self.convert_to_f32v)(to_vert));
        }

        buffer
    }

    pub fn enable_attributes(&self) {
        for attribute in &self.attributes {
            attribute.enable();
        }
    }

    pub fn disable_attributes(&self) {
        for attribute in &self.attributes {
            attribute.disable();
        }
    }

    pub fn get_attributes_clone(&self) -> Vec<VertexAttribute> {
        self.attributes.clone()
    }
}