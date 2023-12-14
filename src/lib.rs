pub mod logger;
pub mod errors;
pub mod rendering;
pub mod camera;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use std::mem;
    use gl::types::{GLfloat, GLint, GLsizei};
    use crate::rendering::Renderer;
    use crate::rendering::rgl::{ShaderProgram, VertexAttribute};
    use crate::rendering::texture::*;
    use crate::rendering::window::Window;
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        let mut window = Window::new(800, 800, "Hello Window!");
        window.init_gl();

        let mut renderer = Renderer::new();
        renderer.bind();
        renderer.with_attribute(VertexAttribute::new(
            0,
            3,
            gl::FLOAT,
            false,
            8 * mem::size_of::<GLfloat>() as GLsizei,
            0
        ));
        renderer.with_attribute(VertexAttribute::new(
            1,
            3,
            gl::FLOAT,
            false,
            8 * mem::size_of::<GLfloat>() as GLsizei,
            3 * mem::size_of::<GLfloat>() as i32
        ));
        renderer.with_attribute(VertexAttribute::new(
            2,
            2,
            gl::FLOAT,
            false,
            8 * mem::size_of::<GLfloat>() as GLsizei,
            6 * mem::size_of::<GLfloat>() as i32
        ));
        renderer.enable_attributes();

        let vertices = [
            //positions        //color          UV
            -0.5, -0.5, 0.0,   1.0, 1.0, 1.0,   1.0, 1.0,
             0.5, -0.5, 0.0,   1.0, 1.0, 1.0,   1.0, 0.0,
             0.5,  0.5, 0.0,   1.0, 1.0, 1.0,   0.0, 0.0,
             0.5,  0.5, 0.0,   1.0, 1.0, 1.0,   0.0, 0.0,
            -0.5,  0.5, 0.0,   1.0, 1.0, 1.0,   1.0, 0.0,
            -0.5, -0.5, 0.0,   1.0, 1.0, 1.0,   1.0, 1.0,
        ];

        let mut shaders = ShaderProgram::new("test_shaders/shader.vsh", "test_shaders/shader.fsh");
        shaders.bind();

        let mut texture = Texture::from("test_shaders/wall.jpg").unwrap();
        texture.bind();
        texture.with_parameter(TextureParameter::new_i(
            gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as GLint
        ));
        texture.with_parameter(TextureParameter::new_i(
            gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as GLint
        ));
        texture.with_parameter(TextureParameter::new_i(
            gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as GLint
        ));
        texture.with_parameter(TextureParameter::new_i(
            gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint
        ));
        texture.apply_parameters();

        while !window.should_close() {
            Renderer::clear_color(0.3, 0.5, 0.3, 1.0);
            Renderer::clear(gl::COLOR_BUFFER_BIT);
            renderer.draw_arrays(gl::TRIANGLES, vertices.to_vec(), 6);
            window.update();
        }

        Ok(())
    }
}
