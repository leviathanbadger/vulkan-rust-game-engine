use nalgebra_glm as glm;
use anyhow::{Result};
use lazy_static::{lazy_static};
use vulkanalia::{
    prelude::v1_0::*
};

pub use crate::game::has_camera_matrix::{HasCameraMatrix};

lazy_static! {
    pub static ref ORIGIN: glm::DVec3 = glm::zero::<glm::DVec3>();
    pub static ref DEFAULT_UP: glm::Vec3 = glm::vec3(0.0, 1.0, 0.0);
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub enum CameraKind {
    #[default]
    Perspective,
    #[allow(unused)]
    Orthographic
}

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pos: glm::DVec3,
    orient: glm::Quat,
    near: f32,
    far: f32,
    kind: CameraKind,
    fovy: f32
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: Default::default(),
            orient: Default::default(),
            near: 0.001,
            far: 1000.0,
            kind: Default::default(),
            fovy: 45.0
        }
    }
}

#[allow(unused)]
impl Camera {
    pub fn pos(&self) -> glm::DVec3 {
        self.pos
    }
    pub fn set_pos(&mut self, pos: glm::DVec3) -> () {
        self.pos = pos;
    }

    pub fn orient(&self) -> glm::Quat {
        self.orient
    }
    pub fn set_orient(&mut self, orient: glm::Quat) -> () {
        self.orient = orient;
    }

    pub fn look_at(&mut self, target: glm::DVec3) -> () {
        self.look_at_up(target, *DEFAULT_UP);
    }
    pub fn look_at_up(&mut self, target: glm::DVec3, up: glm::Vec3) -> () {
        let dir = glm::convert::<glm::DVec3, glm::Vec3>(target - self.pos);
        self.orient = glm::quat_look_at(&dir, &up);
    }

    pub fn near(&self) -> f32 {
        self.near
    }
    pub fn set_near(&mut self, near: f32) -> () {
        self.near = near;
    }

    pub fn far(&self) -> f32 {
        self.far
    }
    pub fn set_far(&mut self, far: f32) -> () {
        self.far = far;
    }

    pub fn kind(&self) -> CameraKind {
        self.kind
    }
    pub fn set_kind(&mut self, kind: CameraKind) -> () {
        self.kind = kind;
    }

    pub fn fovy(&self) -> f32 {
        self.fovy
    }
    pub fn set_fovy(&mut self, fovy: f32) -> () {
        self.fovy = fovy;
    }
}

impl HasCameraMatrix for Camera {
    fn get_view_matrix(&self) -> Result<glm::DMat4> {
        let view_mat4 = glm::quat_to_mat4(&self.orient);
        let mut view = glm::convert::<glm::Mat4, glm::DMat4>(view_mat4);
        view = glm::translate(&view, &-self.pos);

        Ok(view)
    }

    fn get_projection_matrix(&self, bounds: vk::Extent2D) -> Result<glm::Mat4> {
        let mut projection = match self.kind {
            CameraKind::Perspective => {
                let aspect_ratio = bounds.width as f32 / bounds.height as f32;
                let fovy = glm::radians(&glm::vec1(self.fovy))[0];
                glm::perspective(aspect_ratio, fovy, self.near, self.far)
            }
            CameraKind::Orthographic => {
                glm::ortho(-(bounds.width as f32) / 2.0, (bounds.width as f32) / 2.0, -(bounds.height as f32) / 2.0, (bounds.height as f32) / 2.0, self.near, self.far)
            }
        };

        projection[(1, 1)] *= -1.0;

        Ok(projection)
    }
}
