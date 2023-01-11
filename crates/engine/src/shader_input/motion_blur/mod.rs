mod vertex;

use nalgebra_glm as glm;
use lazy_static::{lazy_static};

pub use vertex::{Vertex};

lazy_static! {
    pub static ref VERTICES: Vec<Vertex> = vec![
        Vertex::new(glm::vec2(-1.0, -1.0), glm::vec2(0.0, 0.0)),
        Vertex::new(glm::vec2(1.0, 1.0), glm::vec2(1.0, 1.0)),
        Vertex::new(glm::vec2(1.0, -1.0), glm::vec2(1.0, 0.0)),

        Vertex::new(glm::vec2(1.0, 1.0), glm::vec2(1.0, 1.0)),
        Vertex::new(glm::vec2(-1.0, -1.0), glm::vec2(0.0, 0.0)),
        Vertex::new(glm::vec2(-1.0, 1.0), glm::vec2(0.0, 1.0)),
    ];
}
