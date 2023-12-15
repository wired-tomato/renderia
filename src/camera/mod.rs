use glam::{Mat4, Quat, Vec3, vec3};
use crate::rendering::rgl::ShaderProgram;
use crate::rendering::rgl::UniformValue::UniformMatrix4F;

pub struct PerspectiveCamera {
    pos: Vec3,
    rotation: Quat,
    fov: f32,
    perspective: Mat4,
    width: f32,
    height: f32,
    near: f32,
    far: f32,
}

impl PerspectiveCamera {
    const RIGHT: Vec3 = vec3(1.0, 0.0, 0.0);
    const UP: Vec3 = vec3(0.0, 1.0, 0.0);
    const DIRECTION: Vec3 = vec3(0.0, 0.0, -1.0);

    pub fn new(pos: Vec3, rotation: Quat, fov: f32, width: f32, height: f32, near: f32, far: f32) -> PerspectiveCamera {
        let perspective = Mat4::perspective_rh_gl(fov.to_radians(), width / height, near, far);
        PerspectiveCamera { pos, rotation, fov, perspective, width, height, near, far }
    }

    pub fn apply_vm_to_uniform(&self, uniform_name: &str, program: &mut ShaderProgram) {
        if !program.has_uniform(uniform_name) {
            program.create_uniform(uniform_name);
        }

        program.set_uniform(uniform_name, UniformMatrix4F { value: self.view_matrix() })
    }

    pub fn apply_pm_to_uniform(&self, uniform_name: &str, program: &mut ShaderProgram) {
        if !program.has_uniform(uniform_name) {
            program.create_uniform(uniform_name);
        }

        program.set_uniform(uniform_name, UniformMatrix4F { value: self.perspective.clone() })
    }

    pub fn projection_matrix(&self) -> Mat4 {
        self.perspective.clone()
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.pos, self.cam_direction(), self.cam_up())
    }

    pub fn cam_direction(&self) -> Vec3 {
        (self.rotation * Self::DIRECTION).normalize()
    }

    pub fn cam_right(&self) -> Vec3 {
        (self.rotation * Self::RIGHT).normalize()
    }

    pub fn cam_up(&self) -> Vec3 {
        (self.rotation * Self::UP).normalize()
    }

    pub fn rotate(&mut self, rotation: Quat) {
        self.rotation *= rotation;
    }

    pub fn set_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
    }

    pub fn translate(&mut self, translation: Vec3) {
        self.pos += translation;
    }

    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }
}