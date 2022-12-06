use nalgebra_glm as glm;
use lazy_static::{lazy_static};

use crate::{
    vertex_type,
    resources::{CanBeVertexBufferType}
};

vertex_type!{
    pub struct Vertex {
        pos: Vec3,
        normal: Vec3,
        color: Vec3,
        uv: Vec2
    }
}

lazy_static! {
    pub static ref NO_NORMAL: glm::Vec3 = glm::vec3(1.0, 0.0, 0.0);
    pub static ref NO_UV: glm::Vec2 = glm::vec2(0.0, 0.0);
    pub static ref WHITE: glm::Vec3 = glm::vec3(1.0, 1.0, 1.0);
}

impl CanBeVertexBufferType for Vertex {
    fn create_vertex_from_opts(pos: glm::Vec3, normal: Option<glm::Vec3>, color: Option<glm::Vec3>, uv: Option<glm::Vec2>, face_normal: Option<glm::Vec3>) -> Self {
        Vertex::new(pos, normal.unwrap_or_else(|| face_normal.unwrap_or(*NO_NORMAL)), color.unwrap_or(*WHITE), uv.unwrap_or(*NO_UV))
    }
}
