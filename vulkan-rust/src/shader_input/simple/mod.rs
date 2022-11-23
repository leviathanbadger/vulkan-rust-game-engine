mod vertex;

pub use vertex::{Vertex};

use nalgebra_glm as glm;
use lazy_static::{lazy_static};

fn create_box(pos: glm::Vec3, size: glm::Vec3) -> Vec<Vertex> {
    let aaa = glm::vec3(pos.x, pos.y, pos.z);
    let aab = glm::vec3(pos.x, pos.y, pos.z + size.z);
    let aba = glm::vec3(pos.x, pos.y + size.y, pos.z);
    let abb = glm::vec3(pos.x, pos.y + size.y, pos.z + size.z);
    let baa = glm::vec3(pos.x + size.x, pos.y, pos.z);
    let bab = glm::vec3(pos.x + size.x, pos.y, pos.z + size.z);
    let bba = glm::vec3(pos.x + size.x, pos.y + size.y, pos.z);
    let bbb = glm::vec3(pos.x + size.x, pos.y + size.y, pos.z + size.z);

    let aa = glm::vec2(0.0, 0.0);
    let ab = glm::vec2(0.0, 1.0);
    let ba = glm::vec2(1.0, 0.0);
    let bb = glm::vec2(1.0, 1.0);

    let red = glm::vec3(1.0, 0.2, 0.2);
    let green = glm::vec3(0.2, 1.0, 0.2);
    let blue = glm::vec3(0.2, 0.2, 1.0);

    vec![
        //-x
        Vertex::new(aaa, aa, red),
        Vertex::new(aba, ba, red),
        Vertex::new(aab, ab, red),
        Vertex::new(abb, bb, red),

        //+x
        Vertex::new(baa, ab, red),
        Vertex::new(bab, aa, red),
        Vertex::new(bba, bb, red),
        Vertex::new(bbb, ba, red),

        //-y
        Vertex::new(aaa, aa, green),
        Vertex::new(aab, ab, green),
        Vertex::new(baa, ba, green),
        Vertex::new(bab, bb, green),

        //+y
        Vertex::new(aba, aa, green),
        Vertex::new(bba, ba, green),
        Vertex::new(abb, ab, green),
        Vertex::new(bbb, bb, green),

        //-z
        Vertex::new(aaa, ab, blue),
        Vertex::new(baa, aa, blue),
        Vertex::new(aba, bb, blue),
        Vertex::new(bba, ba, blue),

        //+z
        Vertex::new(aab, aa, blue),
        Vertex::new(abb, ba, blue),
        Vertex::new(bab, ab, blue),
        Vertex::new(bbb, bb, blue),
    ]
}

fn create_box_indices() -> Vec<u16> {
    vec![
        //-x
        0, 1, 2, 1, 3, 2,

        //+x
        4, 5, 6, 6, 5, 7,

        //-y
        8, 9, 10, 10, 9, 11,

        //+y
        12, 13, 14, 13, 15, 14,

        //-z
        16, 17, 18, 17, 19, 18,

        //+z
        20, 21, 22, 22, 21, 23
    ]
}

lazy_static! {
    pub static ref CUBE_VERTICES: Vec<Vertex> = {
        let pos = glm::vec3(-1.0, -1.0, -1.0);
        let size = glm::vec3(2.0, 2.0, 2.0);
        create_box(pos, size)
    };

    pub static ref CUBE_INDICES: Vec<u16> = create_box_indices();
}
