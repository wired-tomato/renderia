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
    use glam::{Mat4, Quat, vec3};
    use crate::camera::PerspectiveCamera;
    use crate::rendering::Renderer;
    use crate::rendering::rgl::{ShaderProgram, VertexAttribute};
    use crate::rendering::rgl::UniformValue::UniformMatrix4F;
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
            5 * mem::size_of::<GLfloat>() as GLsizei,
            0
        ));
        renderer.with_attribute(VertexAttribute::new(
            1,
            2,
            gl::FLOAT,
            false,
            5 * mem::size_of::<GLfloat>() as GLsizei,
            3 * mem::size_of::<GLfloat>() as i32
        ));
        renderer.enable_attributes();

        let vertices = [
            -0.5, -0.5, -0.5,  0.0, 0.0,
            0.5, -0.5, -0.5,  1.0, 0.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5,  0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 0.0,

            -0.5, -0.5,  0.5,  0.0, 0.0,
            0.5, -0.5,  0.5,  1.0, 0.0,
            0.5,  0.5,  0.5,  1.0, 1.0,
            0.5,  0.5,  0.5,  1.0, 1.0,
            -0.5,  0.5,  0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,

            -0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5,  0.5,  1.0, 0.0,

            0.5,  0.5,  0.5,  1.0, 0.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            0.5, -0.5, -0.5,  0.0, 1.0,
            0.5, -0.5, -0.5,  0.0, 1.0,
            0.5, -0.5,  0.5,  0.0, 0.0,
            0.5,  0.5,  0.5,  1.0, 0.0,

            -0.5, -0.5, -0.5,  0.0, 1.0,
            0.5, -0.5, -0.5,  1.0, 1.0,
            0.5, -0.5,  0.5,  1.0, 0.0,
            0.5, -0.5,  0.5,  1.0, 0.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,

            -0.5,  0.5, -0.5,  0.0, 1.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            0.5,  0.5,  0.5,  1.0, 0.0,
            0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5, -0.5,  0.0, 1.0
        ];

        let cube_positions = [
            vec3( 0.0,  0.0,  0.0),
            vec3( 2.0,  5.0, -15.0),
            vec3(-1.5, -2.2, -2.5),
            vec3(-3.8, -2.0, -12.3),
            vec3( 2.4, -0.4, -3.5),
            vec3(-1.7,  3.0, -7.5),
            vec3( 1.3, -2.0, -2.5),
            vec3( 1.5,  2.0, -2.5),
            vec3( 1.5,  0.2, -1.5),
            vec3(-1.3,  1.0, -1.5)
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

        let camera = PerspectiveCamera::new(vec3(0.0, 0.0, -8.0), Quat::default(), 45.0, 800.0, 800.0, 0.1, 100.0);
        shaders.create_uniform("model");
        shaders.create_uniform("view");

        unsafe {
            gl::Enable(gl::DEPTH_TEST)
        }

        while !window.should_close() {
            Renderer::clear_color(0.3, 0.5, 0.3, 1.0);
            Renderer::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            camera.apply_pm_to_uniform("projection", &mut shaders);
            camera.apply_vm_to_uniform("view", &mut shaders);

            renderer.bind();

            //let mut view = Mat4::IDENTITY;
            //let translate = Mat4::from_translation(vec3(0.0, 0.0, -3.0));
            //view *= translate;
            //shaders.set_uniform("view", UniformMatrix4F { value: view });


            let mut i = 0.0;
            for pos in &cube_positions {
                let mut model = Mat4::IDENTITY;
                let transformation = Mat4::from_translation(*pos);
                model *= transformation;
                println!("{model}");

                shaders.set_uniform("model", UniformMatrix4F { value: model });
                renderer.draw_arrays(gl::TRIANGLES, vertices.to_vec(), 36);
                i += 1.0;
            }

            window.update();
        }

        Ok(())
    }
}
