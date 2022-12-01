use nalgebra_glm as glm;
use lazy_static::{lazy_static};

use crate::{
    vertex_type,
    buffer::{CanBeVertexBufferType}
};

vertex_type!{
    pub struct Vertex {
        pos: Vec3,
        uv: Vec2,
        color: Vec3
    }
}

lazy_static! {
    pub static ref NO_UV: glm::Vec2 = glm::vec2(0.0, 0.0);
    pub static ref WHITE: glm::Vec3 = glm::vec3(1.0, 1.0, 1.0);
}

impl CanBeVertexBufferType for Vertex {
    fn create_vertex_from_opts(pos: glm::Vec3, _normal: Option<glm::Vec3>, color: Option<glm::Vec3>, uv: Option<glm::Vec2>) -> Self {
        Vertex::new(pos, uv.unwrap_or(*NO_UV), color.unwrap_or(*WHITE))
    }
}
