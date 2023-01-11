mod vertex;

pub use vertex::{Vertex};

use nalgebra_glm as glm;
use lazy_static::{lazy_static};

use crate::shader_input::create_box::{create_box, create_box_indices};

lazy_static! {
    pub static ref CUBE_VERTICES: Vec<Vertex> = {
        let pos = glm::vec3(-1.0, -1.0, -1.0);
        let size = glm::vec3(2.0, 2.0, 2.0);
        create_box(pos, size)
    };

    pub static ref CUBE_INDICES: Vec<u32> = create_box_indices();
}
