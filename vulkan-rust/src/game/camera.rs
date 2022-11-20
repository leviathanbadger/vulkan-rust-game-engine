use nalgebra_glm as glm;
use anyhow::{Result};
use lazy_static::{lazy_static};
use vulkanalia::{
    prelude::v1_0::*
};

pub use crate::game::has_camera_matrix::{HasCameraMatrix};

lazy_static! {
    pub static ref ORIGIN: glm::Vec3 = glm::vec3(0.0, 0.0, 0.0);
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
    pos: glm::Vec3,
    orient: glm::Quat,
    near: f32,
    far: f32,
    kind: CameraKind,
    fovy: f32,
    is_projection_dirty: bool
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: Default::default(),
            orient: Default::default(),
            near: 0.001,
            far: 1000.0,
            kind: Default::default(),
            fovy: 45.0,
            is_projection_dirty: true
        }
    }
}

#[allow(unused)]
impl Camera {
    pub fn pos(&self) -> glm::Vec3 {
        self.pos
    }
    pub fn set_pos(&mut self, pos: glm::Vec3) -> () {
        self.pos = pos;
    }

    pub fn orient(&self) -> glm::Quat {
        self.orient
    }
    pub fn set_orient(&mut self, orient: glm::Quat) -> () {
        self.orient = orient;
    }

    pub fn look_at(&mut self, target: glm::Vec3) -> () {
        self.look_at_up(target, *DEFAULT_UP);
    }
    pub fn look_at_up(&mut self, target: glm::Vec3, up: glm::Vec3) -> () {
        let dir = target - self.pos;
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
        if self.kind != kind {
            self.kind = kind;
            self.is_projection_dirty = true;
        }
    }

    pub fn fovy(&self) -> f32 {
        self.fovy
    }
    pub fn set_fovy(&mut self, fovy: f32) -> () {
        if self.fovy != fovy {
            self.fovy = fovy;
            if self.kind == CameraKind::Perspective {
                self.is_projection_dirty = true;
            }
        }
    }

    pub fn is_projection_dirty(&self) -> bool {
        self.is_projection_dirty
    }
    pub fn clear_projection_dirty(&mut self) -> () {
        self.is_projection_dirty = false;
    }
}

impl HasCameraMatrix for Camera {
    fn get_view_matrix(&self) -> Result<glm::Mat4> {
        let mut view = glm::quat_to_mat4(&self.orient);
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
