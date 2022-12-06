#![deprecated]
#![allow(unused)]

use nalgebra_glm as glm;
use lazy_static::{lazy_static};

use crate::{
    vertex_type,
    resources::{CanBeVertexBufferType}
};

vertex_type!{
    pub struct Vertex {
        pos: Vec2,
        color: Vec3
    }
}

lazy_static! {
    pub static ref WHITE: glm::Vec3 = glm::vec3(1.0, 1.0, 1.0);
}

impl CanBeVertexBufferType for Vertex {
    fn create_vertex_from_opts(pos: glm::Vec3, _normal: Option<glm::Vec3>, color: Option<glm::Vec3>, _uv: Option<glm::Vec2>, _face_normal: Option<glm::Vec3>) -> Self {
        Vertex::new(glm::vec2(pos.x, pos.y), color.unwrap_or(*WHITE))
    }
}
