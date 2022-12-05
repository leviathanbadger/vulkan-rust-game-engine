use nalgebra_glm as glm;
use lazy_static::{lazy_static};

use crate::{
    vertex_type,
    buffer::{CanBeVertexBufferType}
};

vertex_type!{
    pub struct Vertex {
        pos: Vec2,
        uv: Vec2
    }
}

lazy_static! {
    pub static ref NO_UV: glm::Vec2 = glm::vec2(0.0, 0.0);
}

impl CanBeVertexBufferType for Vertex {
    fn create_vertex_from_opts(pos: glm::Vec3, _normal: Option<glm::Vec3>, _color: Option<glm::Vec3>, uv: Option<glm::Vec2>, _face_normal: Option<glm::Vec3>) -> Self {
        Vertex::new(glm::vec2(pos.x, pos.y), uv.unwrap_or(*NO_UV))
    }
}
