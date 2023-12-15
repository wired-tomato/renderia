use std::collections::HashMap;
use std::ffi::{c_void, CString};
use std::fs::File;
use std::io::Read;
use std::{mem, ptr};
use gl::types::*;
use glam::{DMat2, DMat3, DMat4, DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, Mat2, Mat3, Mat4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4};

pub struct Vao {
    id: GLuint,
}

impl Vao {
    pub fn new() -> Vao {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        Vao { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

pub struct BufferObject {
    id: GLuint,
    r#type: GLenum,
    usage: GLenum,
}

impl BufferObject {
    pub fn new(r#type: GLenum, usage: GLenum) -> BufferObject {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        BufferObject { id, r#type, usage }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, 0);
        }
    }

    pub fn store_f32(&self, data: &[f32]) {
        unsafe {
            gl::BufferData(
              self.r#type,
              (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &data[0] as *const f32 as *const c_void,
                self.usage,
            );
        }
    }

    pub fn store_i32(&self, data: &[i32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &data[0] as *const i32 as *const c_void,
                self.usage,
            );
        }
    }
}

#[derive(Clone)]
pub struct VertexAttribute {
    index: GLuint,
}

impl VertexAttribute {
    pub fn new(
        index: u32,
        size: i32,
        r#type: GLenum,
        normalized: bool,
        stride: GLsizei,
        offset: i32,
    ) -> VertexAttribute {
        unsafe {
            gl::VertexAttribPointer(index, size, r#type as GLenum, normalized.to_gl_boolean(), stride, offset as *const c_void);
        }

        VertexAttribute { index }
    }

    pub fn enable(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.index);
        }
    }

    pub fn disable(&self) {
        unsafe {
            gl::DisableVertexAttribArray(self.index);
        }
    }
}

pub trait ToGLboolean {
    fn to_gl_boolean(&self) -> GLboolean;
}

impl ToGLboolean for bool {
    fn to_gl_boolean(&self) -> GLboolean {
        self.clone() as i32 as GLboolean
    }
}

pub struct ShaderProgram {
    program_handle: u32,
    uniform_ids: HashMap<String, GLint>,
}

#[allow(temporary_cstring_as_ptr)]
impl ShaderProgram {
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> ShaderProgram {
        let mut vertex_shader_file = File::open(vertex_shader_path)
            .unwrap_or_else(|_| panic!("Failed to open {}", vertex_shader_path));
        let mut fragment_shader_file = File::open(fragment_shader_path)
            .unwrap_or_else(|_| panic!("Failed to open {}", fragment_shader_path));

        let mut vertex_shader_source = String::new();
        let mut fragment_shader_source = String::new();

        vertex_shader_file
            .read_to_string(&mut vertex_shader_source)
            .expect("Failed to read vertex shader");

        fragment_shader_file
            .read_to_string(&mut fragment_shader_source)
            .expect("Failed to read fragment shader");

        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(vertex_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);

            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(2048);
            info_log.set_len(2048 - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(vertex_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", std::str::from_utf8(&info_log).unwrap());
            }


            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(fragment_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(fragment_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", std::str::from_utf8(&info_log).unwrap());
            }

            let program_handle = gl::CreateProgram();
            gl::AttachShader(program_handle, vertex_shader);
            gl::AttachShader(program_handle, fragment_shader);
            gl::LinkProgram(program_handle);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            ShaderProgram {
                program_handle,
                uniform_ids: HashMap::new()
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_handle);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn create_uniform(&mut self, uniform_name: &str) {
        let uniform_location = unsafe {
           gl::GetUniformLocation(
               self.program_handle,
               CString::new(uniform_name).unwrap().as_ptr(),
           )
        };

        if uniform_location < 0 {
            panic!("Cannot locate uniform: {}", uniform_name);
        } else {
            self.uniform_ids.insert(uniform_name.to_string(), uniform_location);
        }
    }

    pub fn set_uniform(&self, uniform_name: &str, mut value: UniformValue) {
        let uniform_location = self.uniform_ids[uniform_name];
        value.call_gl(uniform_location);
    }

    pub fn has_uniform(&self, uniform_name: &str) -> bool {
        self.uniform_ids.contains_key(uniform_name)
    }
}

pub enum UniformValue {
    Uniform1D { value: [f64; 1] },
    Uniform1F { value: [f32; 1] },
    Uniform1I { value: [i32; 1] },
    Uniform1Ui { value: [u32; 1] },
    Uniform2D { value: DVec2 },
    Uniform2F { value: Vec2 },
    Uniform2I { value: IVec2 },
    Uniform2Ui { value: UVec2 },
    Uniform3D { value: DVec3 },
    Uniform3F { value: Vec3 },
    Uniform3I { value: IVec3 },
    Uniform3Ui { value: UVec3 },
    Uniform4D { value: DVec4 },
    Uniform4F { value: Vec4 },
    Uniform4I { value: IVec4 },
    Uniform4Ui { value: UVec4 },
    UniformMatrix2D { value: DMat2 },
    UniformMatrix2F { value: Mat2 },
    UniformMatrix3D { value: DMat3 },
    UniformMatrix3F { value: Mat3 },
    UniformMatrix4D { value: DMat4 },
    UniformMatrix4F { value: Mat4 },
}

impl UniformValue {
    pub fn call_gl(&mut self, location: i32) {
        match self {
            UniformValue::Uniform1D { value } => {
                unsafe { gl::Uniform1dv(location, 1, value.as_mut().as_ptr()); }
            }
            UniformValue::Uniform1F { value } => {
                unsafe { gl::Uniform1fv(location, 1, value.as_mut().as_ptr()); }
            }
            UniformValue::Uniform1I { value } => {
                unsafe { gl::Uniform1iv(location, 1, value.as_mut().as_ptr()); }
            }
            UniformValue::Uniform1Ui { value } => {
                unsafe { gl::Uniform1uiv(location, 1, value.as_mut().as_ptr()); }
            }
            UniformValue::Uniform2D { value } => {
                unsafe { gl::Uniform2dv(location, 1, value.as_mut().as_ptr()); }
            }
            UniformValue::Uniform2F { value } => {
                unsafe { gl::Uniform2fv(location, 1, value.as_mut().as_ptr()); }
            }
            UniformValue::Uniform2I { value } => {
                unsafe { gl::Uniform2iv(location, 1, value.as_mut().as_ptr()); }
            }
            UniformValue::Uniform2Ui { value } => {
                unsafe { gl::Uniform2uiv(location, 1, value.as_mut().as_ptr()); }
            }
            UniformValue::Uniform3D { value } => {
                unsafe { gl::Uniform3dv(location, 1, value.as_mut().as_ptr()); }
            }
            UniformValue::Uniform3F { value } => {
                unsafe { gl::Uniform3fv(location, 1, value.as_mut().as_ptr()); }
            }
            UniformValue::Uniform3I { value } => {
                unsafe { gl::Uniform3iv(location, 1, value.as_mut().as_ptr()); }
            }
            UniformValue::Uniform3Ui { value } => {
                unsafe { gl::Uniform3uiv(location, 1, value.as_mut().as_ptr()); }
            }
            UniformValue::Uniform4D { value } => {
                unsafe { gl::Uniform4dv(location, 1, value.as_mut().as_ptr()); }
            }
            UniformValue::Uniform4F { value } => {
                unsafe { gl::Uniform4fv(location, 1, value.as_mut().as_ptr()) }
            }
            UniformValue::Uniform4I { value } => {
                unsafe { gl::Uniform4iv(location, 1, value.as_mut().as_ptr()) }
            }
            UniformValue::Uniform4Ui { value } => {
                unsafe { gl::Uniform4uiv(location, 1, value.as_mut().as_ptr()) }
            }
            UniformValue::UniformMatrix2D { value } => {
                unsafe { gl::UniformMatrix2dv(location, 1, gl::FALSE, value.as_mut().as_ptr()); }
            }
            UniformValue::UniformMatrix2F { value } => {
                unsafe { gl::UniformMatrix2fv(location, 1, gl::FALSE, value.as_mut().as_ptr()); }
            }
            UniformValue::UniformMatrix3D { value } => {
                unsafe { gl::UniformMatrix3dv(location, 1, gl::FALSE, value.as_mut().as_ptr()); }
            }
            UniformValue::UniformMatrix3F { value } => {
                unsafe { gl::UniformMatrix3fv(location, 1, gl::FALSE, value.as_mut().as_ptr()); }
            }
            UniformValue::UniformMatrix4D { value } => {
                unsafe { gl::UniformMatrix4dv(location, 1, gl::FALSE, value.as_mut().as_ptr()); }
            }
            UniformValue::UniformMatrix4F { value } => {
                unsafe { gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_mut().as_ptr()); }
            }
        }
    }
}