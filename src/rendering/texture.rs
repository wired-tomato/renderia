use std::ffi::c_void;
use gl::types::*;
use image::ColorType;
use image::io::Reader as ImageReader;

pub struct Texture {
    id: GLuint,
    width: u32,
    height: u32,
    parameters: Vec<TextureParameter>,
}

impl Texture {
    pub fn new(texture_data: Vec<u8>, color_format: GLenum, color_format_type: GLenum, width: u32, height: u32, parameters: Vec<TextureParameter>) -> Texture {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                color_format as GLint,
                width as GLsizei,
                height as GLsizei,
                0,
                color_format,
                color_format_type,
                texture_data.as_ptr() as *const c_void,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Texture { id, width, height, parameters }
    }

    pub fn from(file_path: &str) -> Result<Texture, String> {
        let reader_option = ImageReader::open(file_path);

        match reader_option {
            Ok(reader) => {
                let texture_result = reader.decode();

                if texture_result.is_err() {
                    return Err(texture_result.err().unwrap().to_string())
                }

                let texture = texture_result.unwrap();
                let ct = texture.color();

                let mut gl_ct = gl::RGB;
                let gl_ct_type = gl::UNSIGNED_BYTE;

                match ct {
                    ColorType::L8 => {
                        return Err("Unsupported color format!".to_string());
                    }
                    ColorType::La8 => {
                        return Err("Unsupported color format!".to_string());
                    }
                    ColorType::Rgb8 => {}
                    ColorType::Rgba8 => {
                        gl_ct = gl::RGBA;
                    }
                    ColorType::L16 => {
                        return Err("Unsupported color format!".to_string());
                    }
                    ColorType::La16 => {
                        return Err("Unsupported color format!".to_string());
                    }
                    ColorType::Rgb16 => {
                        gl_ct = gl::RGB16;
                    }
                    ColorType::Rgba16 => {
                        gl_ct = gl::RGBA16;
                    }
                    ColorType::Rgb32F => {
                        gl_ct = gl::RGB32F;
                    }
                    ColorType::Rgba32F => {
                        gl_ct = gl::RGBA32F;
                    }
                    _ => {
                        return Err("Unsupported color format!".to_string());
                    }
                }

                let width = texture.width();
                let height = texture.height();
                let data = texture.as_bytes();

                Ok(Texture::new(data.to_vec(), gl_ct, gl_ct_type, width, height, Vec::new()))
            }
            Err(val) => {
                Err(val.to_string())
            }
        }
    }

    pub fn with_parameter(&mut self, parameter: TextureParameter) {
        self.parameters.push(parameter);
    }

    pub fn apply_parameters(&self) {
        for parameter in &self.parameters {
            parameter.apply();
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

pub struct TextureParameter {
    r#type: GLenum,
    parameter: GLenum,
    parameter_value_i: Option<GLint>,
    parameter_value_fv: Option<Vec<f32>>,
}

impl TextureParameter {
    pub fn new_i(r#type: GLenum, parameter: GLenum, parameter_value: GLint) -> TextureParameter {
        TextureParameter { r#type, parameter, parameter_value_i: Option::from(parameter_value), parameter_value_fv: None }
    }

    pub fn new_fv(r#type: GLenum, parameter: GLenum, parameter_value: Vec<f32>) -> TextureParameter {
        TextureParameter { r#type, parameter, parameter_value_i: None, parameter_value_fv: Option::from(parameter_value) }
    }

    pub fn apply(&self) {
        match self.parameter_value_i {
            Some(val) => {
                unsafe {
                    gl::TexParameteri(self.r#type, self.parameter, val);
                }
            }
            None => {
                unsafe {
                    let fv = self.parameter_value_fv.clone().unwrap();
                    gl::TexParameterfv(self.r#type, self.parameter, fv.as_ptr());
                }
            }
        }
    }
}