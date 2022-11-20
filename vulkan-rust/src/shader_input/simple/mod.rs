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

    let red = glm::vec3(pos.x + size.x, 0.0, 0.0);
    let green = glm::vec3(0.0, pos.x + size.x, 0.0);
    let blue = glm::vec3(0.0, 0.0, pos.x + size.x);

    fn dark(color: glm::Vec3) -> glm::Vec3 {
        glm::vec3(color.x * 0.6, color.y * 0.6, color.z * 0.6)
    }

    vec![
        //-x
        Vertex::new(aaa, red),
        Vertex::new(aba, red),
        Vertex::new(aab, red),

        Vertex::new(aba, dark(red)),
        Vertex::new(abb, dark(red)),
        Vertex::new(aab, dark(red)),

        //+x
        Vertex::new(baa, red),
        Vertex::new(bab, red),
        Vertex::new(bba, red),

        Vertex::new(bba, dark(red)),
        Vertex::new(bab, dark(red)),
        Vertex::new(bbb, dark(red)),

        //-y
        Vertex::new(aaa, green),
        Vertex::new(aab, green),
        Vertex::new(baa, green),

        Vertex::new(baa, dark(green)),
        Vertex::new(aab, dark(green)),
        Vertex::new(bab, dark(green)),

        //+y
        Vertex::new(aba, green),
        Vertex::new(bba, green),
        Vertex::new(abb, green),

        Vertex::new(bba, dark(green)),
        Vertex::new(bbb, dark(green)),
        Vertex::new(abb, dark(green)),

        //-z
        Vertex::new(aaa, blue),
        Vertex::new(baa, blue),
        Vertex::new(aba, blue),

        Vertex::new(baa, dark(blue)),
        Vertex::new(bba, dark(blue)),
        Vertex::new(aba, dark(blue)),

        //+z
        Vertex::new(aab, blue),
        Vertex::new(abb, blue),
        Vertex::new(bab, blue),

        Vertex::new(bab, dark(blue)),
        Vertex::new(abb, dark(blue)),
        Vertex::new(bbb, dark(blue)),
    ]
}

lazy_static! {
    pub static ref VERTICES: Vec<Vertex> = {
        let pos = glm::vec3(-1.0, -1.0, -1.0);
        let size = glm::vec3(2.0, 2.0, 2.0);
        create_box(pos, size)
    };
}
