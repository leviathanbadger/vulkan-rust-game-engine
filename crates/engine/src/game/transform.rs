use nalgebra_glm as glm;
use anyhow::{Result};
use lazy_static::{lazy_static};

lazy_static! {
    pub static ref ORIGIN: glm::DVec3 = glm::zero::<glm::DVec3>();
    pub static ref DEFAULT_UP: glm::Vec3 = glm::vec3(0.0, 0.0, 1.0);
}

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub pos: glm::DVec3,
    pub orient: glm::Quat
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            pos: Default::default(),
            orient: glm::Quat::identity()
        }
    }
}

impl Transform {
    pub fn look_at(&mut self, target: glm::DVec3) -> () {
        self.look_at_up(target, *DEFAULT_UP);
    }
    pub fn look_at_up(&mut self, target: glm::DVec3, up: glm::Vec3) -> () {
        let dir = glm::convert::<glm::DVec3, glm::Vec3>(self.pos - target);
        self.orient = glm::quat_look_at(&dir, &up);
    }

    pub fn as_matrix(&self) -> Result<glm::DMat4> {
        let view_mat4 = glm::quat_to_mat4(&self.orient);
        let orient = glm::convert::<glm::Mat4, glm::DMat4>(view_mat4);
        let translate = glm::translate(&glm::DMat4::identity(), &self.pos);

        Ok(translate * orient)
    }

    pub fn as_matrix_inverse(&self) -> Result<glm::DMat4> {
        let view_mat4 = glm::quat_to_mat4(&self.orient);
        let mut mat = glm::convert::<glm::Mat4, glm::DMat4>(view_mat4);
        mat = glm::translate(&mat, &-self.pos);

        Ok(mat)
    }
}
