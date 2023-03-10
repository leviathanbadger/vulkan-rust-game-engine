use nalgebra_glm as glm;

use crate::resources::{CanBeVertexBufferType};

pub fn create_box<TVert: CanBeVertexBufferType>(pos: glm::Vec3, size: glm::Vec3) -> Vec<TVert> {
    let aaa = glm::vec3(pos.x, pos.y, pos.z);
    let aab = glm::vec3(pos.x, pos.y, pos.z + size.z);
    let aba = glm::vec3(pos.x, pos.y + size.y, pos.z);
    let abb = glm::vec3(pos.x, pos.y + size.y, pos.z + size.z);
    let baa = glm::vec3(pos.x + size.x, pos.y, pos.z);
    let bab = glm::vec3(pos.x + size.x, pos.y, pos.z + size.z);
    let bba = glm::vec3(pos.x + size.x, pos.y + size.y, pos.z);
    let bbb = glm::vec3(pos.x + size.x, pos.y + size.y, pos.z + size.z);

    let aa = Some(glm::vec2(0.0, 0.0));
    let ab = Some(glm::vec2(0.0, 1.0));
    let ba = Some(glm::vec2(1.0, 0.0));
    let bb = Some(glm::vec2(1.0, 1.0));

    let red = Some(glm::vec3(1.0, 0.2, 0.2));
    let green = Some(glm::vec3(0.2, 1.0, 0.2));
    let blue = Some(glm::vec3(0.2, 0.2, 1.0));

    let nxn = Some(glm::vec3(-1.0, 0.0, 0.0));
    let nxp = Some(glm::vec3(1.0, 0.0, 0.0));
    let nyn = Some(glm::vec3(0.0, -1.0, 0.0));
    let nyp = Some(glm::vec3(0.0, 1.0, 0.0));
    let nzn = Some(glm::vec3(0.0, 0.0, -1.0));
    let nzp = Some(glm::vec3(0.0, 0.0, 1.0));

    vec![
        //-x
        TVert::create_vertex_from_opts(aaa, nxn, red, aa, None, None),
        TVert::create_vertex_from_opts(aba, nxn, red, ba, None, None),
        TVert::create_vertex_from_opts(aab, nxn, red, ab, None, None),
        TVert::create_vertex_from_opts(abb, nxn, red, bb, None, None),

        //+x
        TVert::create_vertex_from_opts(baa, nxp, red, ab, None, None),
        TVert::create_vertex_from_opts(bab, nxp, red, aa, None, None),
        TVert::create_vertex_from_opts(bba, nxp, red, bb, None, None),
        TVert::create_vertex_from_opts(bbb, nxp, red, ba, None, None),

        //-y
        TVert::create_vertex_from_opts(aaa, nyn, green, aa, None, None),
        TVert::create_vertex_from_opts(aab, nyn, green, ab, None, None),
        TVert::create_vertex_from_opts(baa, nyn, green, ba, None, None),
        TVert::create_vertex_from_opts(bab, nyn, green, bb, None, None),

        //+y
        TVert::create_vertex_from_opts(aba, nyp, green, aa, None, None),
        TVert::create_vertex_from_opts(bba, nyp, green, ba, None, None),
        TVert::create_vertex_from_opts(abb, nyp, green, ab, None, None),
        TVert::create_vertex_from_opts(bbb, nyp, green, bb, None, None),

        //-z
        TVert::create_vertex_from_opts(aaa, nzn, blue, ab, None, None),
        TVert::create_vertex_from_opts(baa, nzn, blue, aa, None, None),
        TVert::create_vertex_from_opts(aba, nzn, blue, bb, None, None),
        TVert::create_vertex_from_opts(bba, nzn, blue, ba, None, None),

        //+z
        TVert::create_vertex_from_opts(aab, nzp, blue, aa, None, None),
        TVert::create_vertex_from_opts(abb, nzp, blue, ba, None, None),
        TVert::create_vertex_from_opts(bab, nzp, blue, ab, None, None),
        TVert::create_vertex_from_opts(bbb, nzp, blue, bb, None, None),
    ]
}

pub fn create_box_indices() -> Vec<u32> {
    vec![
        //-x
        0, 2, 1, 1, 2, 3,

        //+x
        4, 6, 5, 6, 7, 5,

        //-y
        8, 10, 9, 10, 11, 9,

        //+y
        12, 14, 13, 13, 14, 15,

        //-z
        16, 18, 17, 17, 18, 19,

        //+z
        20, 22, 21, 22, 23, 21
    ]
}
