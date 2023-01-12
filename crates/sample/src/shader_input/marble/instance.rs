use lazy_static::lazy_static;
use nalgebra_glm as glm;
use engine::{
    vertex_type,
    game::transform::{Transform},
    resources::{CanBeInstVertexBufferType}
};

vertex_type! {
    use engine as engine;

    pub struct MarbleInstance {
        binding 1;
        location 2;
        instance true;

        color: glm::Vec4,
        transform: glm::Mat4
    }
}

impl MarbleInstance {
    pub fn new_from_houdini_data(pos: glm::Vec3, color: glm::Vec4, orient: glm::Quat) -> Self {
        let pos = glm::vec3(pos.x, pos.z, pos.y);

        let transform = Transform {
            pos: glm::convert::<glm::Vec3, glm::DVec3>(pos),
            orient: orient
        };
        let inst_matrix = glm::convert::<glm::DMat4, glm::Mat4>(transform.as_matrix().unwrap());

        Self::new(color, inst_matrix)
    }
}

impl CanBeInstVertexBufferType for MarbleInstance { }

lazy_static! {
    pub static ref MARBLE_INSTANCES: Vec<MarbleInstance> = vec![
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.896203, -1.05015, -0.952156),
            glm::vec4(0.0, 0.0286306, 1.0, 0.6),
            glm::quat(0.678204, -0.326961, -0.592421, -0.28666)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.547684, -1.23697, -0.992617),
            glm::vec4(0.967858, 1.0, 0.0, 1.0),
            glm::quat(0.0952551, 0.51788, 0.651896, -0.545671)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.174733, -1.36451, -0.963649),
            glm::vec4(0.888727, 0.0, 1.0, 1.0),
            glm::quat(0.657596, 0.162949, -0.645922, -0.351852)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.220262, -1.3476, -0.976173),
            glm::vec4(0.699777, 1.0, 0.0, 1.0),
            glm::quat(0.841649, 0.498707, 0.12396, 0.165986)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.604879, -1.26836, -0.92124),
            glm::vec4(1.0, 0.42231, 0.0, 1.0),
            glm::quat(-0.525488, -0.837516, -0.0933594, -0.117103)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.946273, -1.07369, -0.874461),
            glm::vec4(1.0, 0.0, 0.428113, 1.0),
            glm::quat(-0.114727, -0.0320369, 0.281736, -0.95207)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.938106, -1.24998, -0.613378),
            glm::vec4(0.0, 1.0, 0.752218, 1.0),
            glm::quat(-0.698808, -0.531368, 0.224055, -0.42322)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.588197, -1.43133, -0.648847),
            glm::vec4(1.0, 0.0, 0.0718826, 1.0),
            glm::quat(0.238073, -0.178149, 0.924611, -0.238074)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.213065, -1.55325, -0.601768),
            glm::vec4(0.633812, 1.0, 0.0, 0.6),
            glm::quat(-0.253215, 0.82958, -0.125023, 0.481713)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.182389, -1.55817, -0.596237),
            glm::vec4(0.510304, 1.0, 0.0, 1.0),
            glm::quat(-0.238537, -0.108259, 0.0267582, 0.964709)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.634821, -1.44675, -0.568655),
            glm::vec4(0.0, 0.461437, 1.0, 1.0),
            glm::quat(0.0838321, 0.0289763, 0.0124925, 0.99598)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.991857, -1.25, -0.521386),
            glm::vec4(1.0, 0.0, 0.00951385, 1.0),
            glm::quat(-0.559699, -0.129486, 0.457794, -0.678525)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.947561, -1.36766, -0.228535),
            glm::vec4(1.0, 0.0519755, 0.0, 1.0),
            glm::quat(0.0166907, -0.237973, 0.866627, -0.438232)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.596826, -1.55224, -0.239821),
            glm::vec4(0.971216, 0.0, 1.0, 0.6),
            glm::quat(0.975421, 0.155748, -0.0814704, 0.132884)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.214459, -1.65191, -0.217462),
            glm::vec4(0.0, 1.0, 0.691959, 1.0),
            glm::quat(-0.0344266, 0.717238, 0.604229, -0.345387)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.182116, -1.65491, -0.212351),
            glm::vec4(0.373946, 0.0, 1.0, 1.0),
            glm::quat(-0.444017, -0.428687, 0.728404, 0.297497)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.565861, -1.5672, -0.19638),
            glm::vec4(0.0, 1.0, 0.882801, 1.0),
            glm::quat(-0.709891, 0.622259, -0.321652, -0.0734044)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.920859, -1.39678, -0.161253),
            glm::vec4(0.0, 0.70525, 1.0, 1.0),
            glm::quat(0.0314298, 0.904175, 0.125718, 0.407032)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.957832, -1.36976, 0.168273),
            glm::vec4(1.0, 0.0, 0.379512, 1.0),
            glm::quat(0.192023, -0.651308, 0.687793, -0.256644)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.600474, -1.56114, 0.155492),
            glm::vec4(1.0, 0.235825, 0.0, 1.0),
            glm::quat(0.671655, -0.526645, 0.521048, 0.00578798)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.218309, -1.65199, 0.202565),
            glm::vec4(0.0, 0.460783, 1.0, 0.6),
            glm::quat(0.269195, -0.695224, 0.654759, 0.124454)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.176585, -1.65831, 0.187771),
            glm::vec4(0.0, 0.761689, 1.0, 1.0),
            glm::quat(-0.188233, 0.794076, -0.0696045, -0.573731)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.563007, -1.56845, 0.201042),
            glm::vec4(1.0, 0.712074, 0.0, 1.0),
            glm::quat(-0.221393, 0.820895, 0.482812, 0.209782)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.914859, -1.38888, 0.236111),
            glm::vec4(0.197023, 1.0, 0.0, 1.0),
            glm::quat(-0.215432, -0.404616, 0.0370664, -0.887976)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.00041, -1.23563, 0.537898),
            glm::vec4(0.0, 0.02404, 1.0, 0.6),
            glm::quat(-0.690385, -0.0425751, -0.714124, -0.107628)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.665841, -1.44818, 0.528726),
            glm::vec4(0.649348, 1.0, 0.0, 1.0),
            glm::quat(0.42434, -0.740895, 0.211837, -0.475537)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.268145, -1.55265, 0.584631),
            glm::vec4(0.871803, 1.0, 0.0, 1.0),
            glm::quat(-0.4747, -0.840379, 0.244986, -0.0916769)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.134721, -1.5719, 0.574121),
            glm::vec4(1.0, 0.0, 0.828298, 0.6),
            glm::quat(-0.0552229, -0.851196, 0.222639, -0.472068)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.51965, -1.44434, 0.682256),
            glm::vec4(0.657739, 0.0, 1.0, 1.0),
            glm::quat(0.893761, -0.211575, 0.386495, -0.0839543)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.880069, -1.28924, 0.618229),
            glm::vec4(0.0, 1.0, 0.164057, 1.0),
            glm::quat(-0.224047, 0.605792, 0.358057, 0.67425)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.883044, -1.08306, 0.927315),
            glm::vec4(0.0, 0.214914, 1.0, 0.6),
            glm::quat(-0.317005, 0.948366, -0.00623387, 0.00845734)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.561605, -1.30982, 0.886564),
            glm::vec4(1.0, 0.0, 0.0490285, 1.0),
            glm::quat(-0.820929, 0.1233, 0.475919, 0.290472)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.175193, -1.38569, 0.930675),
            glm::vec4(0.0, 0.513636, 1.0, 0.6),
            glm::quat(0.474071, 0.359157, -0.707678, 0.381386)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.219794, -1.38014, 0.933161),
            glm::vec4(0.345942, 1.0, 0.0, 1.0),
            glm::quat(0.527436, 0.331541, -0.778946, 0.0716588)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.57243, -1.21689, 1.00402),
            glm::vec4(1.0, 0.0, 0.268579, 1.0),
            glm::quat(0.0704952, -0.563047, -0.784595, 0.249838)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.922821, -1.04732, 0.934762),
            glm::vec4(0.0, 0.703785, 1.0, 1.0),
            glm::quat(-0.155865, -0.342823, 0.892814, 0.247105)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.22857, -1.06024, -0.419528),
            glm::vec4(1.0, 0.434421, 0.0, 1.0),
            glm::quat(-0.392727, 0.866369, 0.267854, 0.153048)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.934179, -0.84931, -0.585749),
            glm::vec4(1.0, 0.228111, 0.0, 0.6),
            glm::quat(0.269831, 0.481291, -0.752662, 0.359235)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.592247, -1.0347, -0.654812),
            glm::vec4(0.0, 0.366334, 1.0, 1.0),
            glm::quat(0.526556, 0.487166, -0.368058, -0.591558)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.249981, -0.975307, -0.979334),
            glm::vec4(0.503484, 0.0, 1.0, 1.0),
            glm::quat(-0.615456, 0.754134, -0.213897, -0.0821143)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.00809172, -1.13672, -1.23535),
            glm::vec4(0.0531611, 1.0, 0.0, 1.0),
            glm::quat(0.519793, 0.0445458, 0.84609, -0.109377)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.273313, -0.810648, -1.44003),
            glm::vec4(0.0653254, 0.0, 1.0, 0.6),
            glm::quat(0.152005, -0.541807, -0.0426708, 0.825541)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.21992, -1.15305, -0.0346038),
            glm::vec4(0.856872, 1.0, 0.0, 1.0),
            glm::quat(0.248222, -0.119921, 0.921309, 0.274215)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.74371, -1.03266, -0.290491),
            glm::vec4(0.973824, 1.0, 0.0, 1.0),
            glm::quat(0.703188, -0.40417, 0.581636, -0.0622385)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.391372, -1.22377, -0.370961),
            glm::vec4(1.0, 0.227049, 0.0, 1.0),
            glm::quat(-0.404405, -0.407001, -0.77347, 0.269354)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.11682, -1.14035, -0.644683),
            glm::vec4(0.0, 1.0, 0.867414, 0.6),
            glm::quat(0.188717, 0.380974, -0.871493, -0.244428)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.267633, -1.03909, -0.733813),
            glm::vec4(0.0, 0.0715716, 1.0, 1.0),
            glm::quat(0.15938, 0.854295, 0.487844, -0.0823818)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.374601, -1.0106, -1.11418),
            glm::vec4(0.0, 0.982295, 1.0, 0.6),
            glm::quat(0.360341, 0.870438, 0.0633495, -0.32936)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.922848, -0.984702, 0.242086),
            glm::vec4(1.0, 0.359808, 0.0, 0.6),
            glm::quat(0.143414, -0.0524094, -0.841688, -0.517925)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.627363, -1.1754, 0.0584954),
            glm::vec4(1.0, 0.0, 0.789647, 1.0),
            glm::quat(0.48552, -0.574925, 0.414728, -0.511597)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.266612, -1.31886, -0.006327),
            glm::vec4(0.0, 1.0, 0.709412, 1.0),
            glm::quat(0.049062, 0.715699, 0.660549, -0.221458)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.0133785, -1.32261, -0.309696),
            glm::vec4(0.505711, 1.0, 0.0, 1.0),
            glm::quat(-0.0479856, -0.913944, 0.27928, -0.290528)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.358333, -1.22095, -0.393936),
            glm::vec4(1.0, 0.288059, 0.0, 0.6),
            glm::quat(0.187314, 0.557804, -0.144663, 0.795513)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.657735, -1.052, -0.593428),
            glm::vec4(1.0, 0.0, 0.983393, 0.6),
            glm::quat(0.732603, -0.163095, 0.433174, -0.499053)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.655525, -1.05102, 0.60296),
            glm::vec4(0.39006, 0.0, 1.0, 0.6),
            glm::quat(0.423992, -0.616457, -0.370886, 0.55014)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.384808, -1.21896, 0.369041),
            glm::vec4(0.214207, 1.0, 0.0, 1.0),
            glm::quat(0.154037, 0.546381, 0.702696, -0.428904)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.0125364, -1.32618, 0.299024),
            glm::vec4(0.0, 1.0, 0.167638, 0.6),
            glm::quat(-0.45631, -0.84068, 0.170981, 0.236228)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.240404, -1.31842, -0.00598219),
            glm::vec4(1.0, 0.0, 0.352115, 1.0),
            glm::quat(0.383205, -0.285994, 0.77776, -0.407985)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.60004, -1.1757, -0.0844652),
            glm::vec4(0.0, 0.32986, 1.0, 1.0),
            glm::quat(0.0743388, 0.844092, 0.21006, -0.487706)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.876405, -0.962185, -0.274565),
            glm::vec4(0.640039, 0.0, 1.0, 0.6),
            glm::quat(0.251706, -0.415296, -0.635969, -0.599764)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.443714, -1.00616, 1.11351),
            glm::vec4(1.0, 0.0, 0.688386, 0.6),
            glm::quat(0.880844, 0.110294, 0.442166, 0.128215)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.276656, -1.06831, 0.718901),
            glm::vec4(0.228367, 0.0, 1.0, 1.0),
            glm::quat(-0.00259466, 0.885552, -0.28667, 0.36553)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.102806, -1.15066, 0.634339),
            glm::vec4(1.0, 0.878275, 0.0, 1.0),
            glm::quat(-0.374733, 0.0851787, -0.119066, 0.915502)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.372476, -1.26136, 0.363568),
            glm::vec4(0.512609, 0.0, 1.0, 0.6),
            glm::quat(-0.761838, 0.620517, 0.185895, -0.00193548)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.700137, -1.05736, 0.279789),
            glm::vec4(1.0, 0.0, 0.990178, 0.6),
            glm::quat(-0.466705, -0.824646, -0.249067, -0.200278)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.21485, -1.15917, -0.0417204),
            glm::vec4(0.678759, 0.0, 1.0, 1.0),
            glm::quat(0.0365697, 0.848391, 0.47807, 0.224376)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.374271, -0.786577, 1.43599),
            glm::vec4(0.0, 0.176758, 1.0, 1.0),
            glm::quat(0.576061, -0.234788, 0.482916, -0.616296)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.00984973, -1.16034, 1.21371),
            glm::vec4(1.0, 0.831849, 0.0, 1.0),
            glm::quat(-0.102819, -0.429695, -0.37023, 0.817141)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.249879, -0.988132, 0.967625),
            glm::vec4(1.0, 0.0, 0.626568, 1.0),
            glm::quat(0.863685, 0.259932, -0.398822, 0.165605)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.552524, -1.04883, 0.645638),
            glm::vec4(0.0, 0.980704, 1.0, 1.0),
            glm::quat(-0.0234271, 0.450055, -0.889836, -0.0713698)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.920362, -0.898084, 0.567825),
            glm::vec4(0.484852, 0.0, 1.0, 1.0),
            glm::quat(-0.835487, 0.234459, 0.496028, 0.0307687)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.18095, -1.1305, 0.381894),
            glm::vec4(1.0, 0.736552, 0.0, 1.0),
            glm::quat(0.544827, 0.68521, 0.144954, 0.461129)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.48051, -0.623268, 0.482706),
            glm::vec4(0.820726, 0.0, 1.0, 1.0),
            glm::quat(0.278174, 0.506926, -0.815794, 0.0111969)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.40596, -0.8964, 0.203235),
            glm::vec4(0.2656, 0.0, 1.0, 1.0),
            glm::quat(0.0886604, 0.228849, -0.261144, 0.93358)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.05008, -0.817334, -0.161887),
            glm::vec4(0.0, 0.608164, 1.0, 0.6),
            glm::quat(0.390325, -0.531277, -0.602389, -0.45002)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.579186, -0.727197, -0.905049),
            glm::vec4(0.0, 1.0, 0.335831, 1.0),
            glm::quat(-0.227358, -0.221404, 0.477596, 0.819262)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.642379, -0.936723, -1.2343),
            glm::vec4(0.848996, 1.0, 0.0, 1.0),
            glm::quat(0.486859, 0.540771, -0.641245, 0.243596)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.582436, -0.608875, -1.45335),
            glm::vec4(0.980798, 0.0, 1.0, 0.6),
            glm::quat(0.0918192, 0.335772, -0.349107, -0.870029)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.26176, -0.939602, 0.580805),
            glm::vec4(1.0, 0.0, 0.94591, 0.6),
            glm::quat(0.0391444, 0.96118, -0.217304, 0.16547)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.892739, -0.60499, 0.132359),
            glm::vec4(0.0, 0.0203657, 1.0, 1.0),
            glm::quat(-0.822522, -0.455563, -0.29269, 0.173935)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.568994, -0.788422, -0.00417444),
            glm::vec4(0.558151, 0.0, 1.0, 0.6),
            glm::quat(0.730124, 0.658779, 0.149084, 0.103455)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.461264, -0.77133, -0.388973),
            glm::vec4(0.591837, 1.0, 0.0, 0.6),
            glm::quat(-0.27803, 0.548985, -0.78584, -0.061404)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.204602, -0.599826, -0.859263),
            glm::vec4(0.0, 1.0, 0.382802, 1.0),
            glm::quat(0.608495, -0.700766, -0.256666, 0.269784)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.277667, -0.904129, -1.38431),
            glm::vec4(0.0, 0.821997, 1.0, 0.6),
            glm::quat(-0.863512, -0.396605, 0.28571, -0.12418)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.912159, -0.764107, 0.693756),
            glm::vec4(0.433994, 1.0, 0.0, 1.0),
            glm::quat(-0.127977, -0.509356, 0.52747, -0.667798)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.390021, -0.796489, 0.454901),
            glm::vec4(0.115142, 1.0, 0.0, 0.6),
            glm::quat(-0.0479812, -0.536134, -0.0694686, -0.8399)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.230767, -0.945561, 0.124798),
            glm::vec4(1.0, 0.0, 0.209751, 1.0),
            glm::quat(0.646306, -0.649181, -0.359898, 0.176991)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.132642, -0.947414, -0.258007),
            glm::vec4(0.731616, 1.0, 0.0, 0.6),
            glm::quat(0.909569, 0.0534505, 0.158364, 0.380458)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.0178739, -0.772529, -0.580479),
            glm::vec4(0.0, 1.0, 0.783064, 1.0),
            glm::quat(0.228527, -0.0406249, -0.451313, -0.861651)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.0619926, -0.766446, -1.10194),
            glm::vec4(0.989643, 1.0, 0.0, 1.0),
            glm::quat(-0.601276, 0.182558, -0.738778, 0.243612)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.111741, -0.828527, 0.988193),
            glm::vec4(0.945282, 0.0, 1.0, 1.0),
            glm::quat(0.483876, 0.68246, 0.159485, -0.524097)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.0328726, -0.764621, 0.585949),
            glm::vec4(0.0, 0.495724, 1.0, 1.0),
            glm::quat(-0.680612, 0.179605, -0.6609, 0.260232)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.140624, -0.959218, 0.256789),
            glm::vec4(0.885631, 0.0, 1.0, 1.0),
            glm::quat(0.198392, 0.516607, -0.809827, -0.194775)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.238676, -0.940248, -0.126218),
            glm::vec4(0.0, 1.0, 0.49251, 1.0),
            glm::quat(0.491793, 0.221331, -0.116394, -0.834029)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.396511, -0.788149, -0.455869),
            glm::vec4(0.0, 0.409052, 1.0, 0.6),
            glm::quat(-0.300154, 0.458653, 0.521633, 0.653791)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.893864, -0.741068, -0.668071),
            glm::vec4(0.0772517, 0.0, 1.0, 1.0),
            glm::quat(0.180143, 0.760681, -0.608877, -0.134842)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.124784, -0.858786, 1.43448),
            glm::vec4(1.0, 0.0193634, 0.0, 1.0),
            glm::quat(0.192797, -0.968473, 0.109682, -0.113399)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.220303, -0.60063, 0.891965),
            glm::vec4(1.0, 0.67392, 0.0, 1.0),
            glm::quat(-0.0579436, -0.167013, 0.287941, 0.94119)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.457365, -0.76223, 0.390382),
            glm::vec4(0.0192629, 0.0, 1.0, 0.6),
            glm::quat(-0.344964, -0.796353, 0.494898, -0.0435563)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.578219, -0.790376, 0.0133127),
            glm::vec4(0.597112, 0.0, 1.0, 1.0),
            glm::quat(0.469184, -0.362, -0.769094, -0.23941)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.884654, -0.590738, -0.136377),
            glm::vec4(0.407756, 0.0, 1.0, 0.6),
            glm::quat(-0.381033, -0.809796, -0.374988, 0.241719)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.23018, -0.950603, -0.62732),
            glm::vec4(0.0, 1.0, 0.603258, 0.6),
            glm::quat(-0.23649, 0.56474, -0.284402, -0.737738)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.497959, -0.609854, 1.48005),
            glm::vec4(0.048394, 0.0, 1.0, 0.6),
            glm::quat(-0.0318637, -0.0274983, 0.603342, 0.796371)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.483664, -0.956261, 1.29005),
            glm::vec4(1.0, 0.0, 0.0070694, 1.0),
            glm::quat(-0.281245, 0.0605467, 0.349025, 0.891862)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.58821, -0.784357, 0.936354),
            glm::vec4(0.94472, 1.0, 0.0, 1.0),
            glm::quat(-0.220044, -0.588227, 0.340498, -0.699736)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.01549, -0.867683, 0.137315),
            glm::vec4(0.0, 1.0, 0.630902, 1.0),
            glm::quat(0.0608072, -0.082508, 0.711302, -0.695374)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.25373, -0.824995, -0.251868),
            glm::vec4(0.826337, 0.0, 1.0, 1.0),
            glm::quat(0.0111396, -0.739348, -0.441264, -0.508454)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.45229, -0.628171, -0.557885),
            glm::vec4(1.0, 0.0, 0.269142, 0.6),
            glm::quat(-0.271251, -0.866363, -0.191023, -0.373293)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.00372, -0.505184, 1.24297),
            glm::vec4(0.209952, 1.0, 0.0, 1.0),
            glm::quat(0.476081, -0.0905982, -0.250106, 0.838204)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.20513, -0.682507, 0.946056),
            glm::vec4(0.0, 1.0, 0.17813, 0.6),
            glm::quat(0.396181, -0.509471, -0.483927, 0.591012)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.2615, -0.477734, 0.186557),
            glm::vec4(1.0, 0.227424, 0.0, 1.0),
            glm::quat(0.761983, 0.289685, -0.0810525, -0.573494)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.45604, -0.752863, -0.323175),
            glm::vec4(1.0, 0.599891, 0.0, 0.6),
            glm::quat(-0.691887, -0.291549, -0.647848, 0.128777)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.30665, -0.784363, -0.701138),
            glm::vec4(0.0, 0.929643, 1.0, 1.0),
            glm::quat(0.0564794, -0.862993, -0.311303, -0.393883)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.23984, -0.528926, -0.998283),
            glm::vec4(1.0, 0.0, 0.947617, 1.0),
            glm::quat(0.379722, -0.623662, -0.565168, 0.383982)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.784825, -0.835797, 1.22331),
            glm::vec4(1.0, 0.273782, 0.0, 1.0),
            glm::quat(-0.607654, 0.724241, -0.219552, 0.240892)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.996643, -0.443305, 0.47947),
            glm::vec4(1.0, 0.47879, 0.0, 0.6),
            glm::quat(-0.389953, 0.305907, 0.787308, -0.366748)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.532558, -0.485917, 0.251249),
            glm::vec4(0.998302, 0.0, 1.0, 1.0),
            glm::quat(-0.797966, 0.531854, -0.255266, -0.123377)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.766828, -0.554649, -0.252812),
            glm::vec4(1.0, 0.0, 0.293441, 1.0),
            glm::quat(-0.0140688, -0.493013, 0.818781, -0.293831)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.485949, -0.341142, -0.960886),
            glm::vec4(1.0, 0.4069, 0.0, 1.0),
            glm::quat(-0.0373742, -0.558504, -0.566894, 0.604407)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.941726, -0.676667, -1.21476),
            glm::vec4(0.0, 0.179853, 1.0, 1.0),
            glm::quat(-0.23866, -0.688996, 0.65736, 0.190274)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.0868011, -0.561614, 1.2799),
            glm::vec4(0.515658, 1.0, 0.0, 1.0),
            glm::quat(-0.178843, 0.460568, 0.866749, 0.0680993)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.262993, -0.556368, 0.74232),
            glm::vec4(1.0, 0.0, 0.715016, 1.0),
            glm::quat(-0.623558, -0.588761, -0.1664, 0.486669)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.113825, -0.583428, 0.266012),
            glm::vec4(0.0, 0.437126, 1.0, 1.0),
            glm::quat(-0.876563, -0.143729, -0.439478, 0.13356)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.241084, -0.595104, -0.109022),
            glm::vec4(0.00750554, 0.0, 1.0, 0.6),
            glm::quat(0.817024, 0.447824, 0.314695, 0.181362)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.22113, -0.471572, -0.484964),
            glm::vec4(1.0, 0.568821, 0.0, 1.0),
            glm::quat(0.398719, 0.279736, -0.560587, -0.669711)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.263281, -0.511571, -1.24082),
            glm::vec4(1.0, 0.47712, 0.0, 1.0),
            glm::quat(0.00731341, 0.631051, 0.656132, 0.413778)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.273207, -0.401316, 1.22915),
            glm::vec4(0.0, 1.0, 0.743273, 1.0),
            glm::quat(0.235199, -0.836275, 0.443829, -0.219867)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.261858, -0.446781, 0.527408),
            glm::vec4(0.0, 0.714059, 1.0, 0.6),
            glm::quat(-0.671881, 0.425948, 0.542143, 0.270603)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.253364, -0.558267, 0.120123),
            glm::vec4(1.0, 0.668935, 0.0, 0.6),
            glm::quat(0.942918, 0.117403, -0.0419856, -0.308802)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.12721, -0.579072, -0.253835),
            glm::vec4(1.0, 0.0355139, 0.0, 1.0),
            glm::quat(0.197876, 0.160728, -0.962489, 0.0928758)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.530804, -0.759361, -0.830708),
            glm::vec4(1.0, 0.088038, 0.0, 0.6),
            glm::quat(-0.353368, -0.291695, -0.642954, 0.613723)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.117475, -0.460906, -1.34654),
            glm::vec4(1.0, 0.358899, 0.0, 0.6),
            glm::quat(0.128172, -0.0483114, 0.371572, -0.918244)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.820263, -0.748216, 1.25722),
            glm::vec4(1.0, 0.330635, 0.0, 1.0),
            glm::quat(0.677231, -0.463345, 0.314542, -0.477214)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.561678, -0.389517, 0.956795),
            glm::vec4(0.938693, 0.0, 1.0, 1.0),
            glm::quat(0.692219, -0.126788, -0.596449, 0.386014)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.786637, -0.573183, 0.272006),
            glm::vec4(0.0, 1.0, 0.194115, 1.0),
            glm::quat(-0.0483671, 0.160263, 0.276933, 0.946195)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.516871, -0.48345, -0.231975),
            glm::vec4(0.0, 1.0, 0.988874, 1.0),
            glm::quat(0.520029, -0.45183, 0.461186, -0.559221)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.837296, -0.398011, -0.47831),
            glm::vec4(0.727283, 0.0, 1.0, 1.0),
            glm::quat(0.936653, 0.0239973, -0.185007, -0.296441)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.748117, -0.902278, -1.2019),
            glm::vec4(0.0, 1.0, 0.453543, 1.0),
            glm::quat(-0.243684, -0.510373, 0.627875, 0.534706)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.13521, -0.562485, 1.10215),
            glm::vec4(1.0, 0.945889, 0.0, 1.0),
            glm::quat(0.00888196, 0.441801, -0.649436, -0.618842)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.22315, -0.812596, 0.807963),
            glm::vec4(1.0, 0.0, 0.336422, 1.0),
            glm::quat(0.209521, -0.793525, 0.511911, -0.253704)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.41363, -0.88903, 0.170692),
            glm::vec4(0.0, 0.65327, 1.0, 0.6),
            glm::quat(-0.540723, -0.482294, 0.423827, -0.54349)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.20034, -0.438022, -0.321062),
            glm::vec4(0.423088, 1.0, 0.0, 1.0),
            glm::quat(-0.562325, -0.272014, 0.524282, 0.578729)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.22257, -0.644324, -0.880423),
            glm::vec4(0.0, 1.0, 0.0381841, 0.6),
            glm::quat(0.620397, -0.386985, 0.409276, -0.545749)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.00194, -0.597999, -1.20554),
            glm::vec4(1.0, 0.0, 0.5474, 1.0),
            glm::quat(-0.627046, 0.649785, -0.423431, -0.072795)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.293074, -0.432154, 1.59366),
            glm::vec4(1.0, 0.0, 0.0958735, 1.0),
            glm::quat(-0.65588, -0.174599, -0.374936, -0.631474)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.664028, -0.517303, 1.44869),
            glm::vec4(0.869834, 0.0, 1.0, 0.6),
            glm::quat(-0.168964, -0.539547, -0.58308, 0.583402)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.95772, -0.381776, 0.869561),
            glm::vec4(1.0, 0.0, 0.012941, 0.6),
            glm::quat(-0.75066, 0.455846, -0.242913, -0.411956)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.42492, -0.388593, 0.795368),
            glm::vec4(1.0, 0.0, 0.0345713, 1.0),
            glm::quat(0.385068, 0.906231, -0.0174346, 0.173678)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.59151, -0.531098, -0.0237726),
            glm::vec4(0.715831, 1.0, 0.0, 1.0),
            glm::quat(-0.0307657, -0.921137, -0.332846, -0.199435)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.63151, -0.193439, -0.322654),
            glm::vec4(0.0425082, 0.0, 1.0, 1.0),
            glm::quat(0.105494, 0.739772, -0.312471, 0.58649)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.116263, -0.469306, 1.60591),
            glm::vec4(1.0, 0.0, 0.980512, 1.0),
            glm::quat(-0.0732288, 0.530588, 0.809891, 0.239146)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.454816, -0.611607, 1.08586),
            glm::vec4(0.345772, 0.0, 1.0, 1.0),
            glm::quat(0.820103, 0.390626, -0.328907, -0.258189)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.640287, -0.476197, 0.652514),
            glm::vec4(0.895152, 1.0, 0.0, 1.0),
            glm::quat(-0.433197, -0.677798, 0.593286, -0.0307087)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.762589, -0.247612, 0.0238065),
            glm::vec4(0.0, 0.00216751, 1.0, 0.6),
            glm::quat(-0.669498, 0.604891, -0.392107, 0.179253)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.13678, -0.543853, -0.434946),
            glm::vec4(0.0, 1.0, 0.00281194, 1.0),
            glm::quat(0.581793, -0.0624401, 0.748129, 0.312923)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.50363, -0.460145, -0.587641),
            glm::vec4(1.0, 0.209458, 0.0, 1.0),
            glm::quat(0.36562, 0.546606, 0.129811, 0.742087)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.901786, -0.374209, 1.36198),
            glm::vec4(1.0, 0.0, 0.95758, 1.0),
            glm::quat(0.679945, -0.699459, 0.0293311, -0.218108)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.045324, -0.317884, 0.971166),
            glm::vec4(0.0, 0.169229, 1.0, 1.0),
            glm::quat(0.275549, -0.877308, 0.0697923, -0.386695)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.0946706, -0.276437, 0.517814),
            glm::vec4(1.0, 0.0, 0.720176, 1.0),
            glm::quat(-0.664842, -0.326544, 0.594228, 0.313445)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.251856, -0.247965, 0.102574),
            glm::vec4(0.0, 0.638294, 1.0, 0.6),
            glm::quat(-0.32757, -0.843458, 0.162772, -0.393423)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.603639, -0.451331, -0.598939),
            glm::vec4(0.0, 0.985252, 1.0, 1.0),
            glm::quat(-0.709219, 0.674681, 0.179933, -0.097152)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.864568, -0.450531, -0.896922),
            glm::vec4(0.0, 1.0, 0.172968, 0.6),
            glm::quat(0.705069, 0.45075, -0.470335, 0.280155)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.891644, -0.574716, 0.790665),
            glm::vec4(0.218249, 0.0, 1.0, 1.0),
            glm::quat(-0.269652, 0.889782, 0.0245729, 0.367384)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.645697, -0.35447, 0.568901),
            glm::vec4(1.0, 0.0, 0.216258, 0.6),
            glm::quat(0.522652, 0.454987, 0.327888, 0.642114)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.407021, -0.204645, 0.0295556),
            glm::vec4(1.0, 0.974234, 0.0, 1.0),
            glm::quat(-0.465721, -0.338085, 0.739484, 0.349236)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.0512103, -0.203657, -0.147401),
            glm::vec4(0.0, 1.0, 0.145001, 0.6),
            glm::quat(0.586431, 0.0733874, -0.165359, 0.789537)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.0417657, -0.31729, -0.986411),
            glm::vec4(0.0, 1.0, 0.502771, 0.6),
            glm::quat(0.719468, 0.389868, 0.366095, 0.443107)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.863478, -0.333456, -1.39731),
            glm::vec4(0.0, 0.557962, 1.0, 1.0),
            glm::quat(0.762512, 0.589276, -0.155658, -0.217026)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.21698, -0.671218, 0.436924),
            glm::vec4(0.0, 1.0, 0.265849, 1.0),
            glm::quat(0.491849, -0.678159, 0.109777, -0.534915)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.22308, -0.539234, 0.0614173),
            glm::vec4(0.087987, 0.0, 1.0, 1.0),
            glm::quat(0.254488, -0.198598, 0.218156, 0.920979)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.00793, -0.22524, -0.0492598),
            glm::vec4(0.852354, 0.0, 1.0, 1.0),
            glm::quat(0.173937, -0.252842, 0.934881, 0.178366)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.275385, -0.492904, -0.689526),
            glm::vec4(0.0, 0.670292, 1.0, 0.6),
            glm::quat(0.734276, 0.623342, -0.264142, -0.050116)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.431329, -0.497501, -1.10665),
            glm::vec4(0.0, 0.0825551, 1.0, 1.0),
            glm::quat(-0.151563, -0.358209, -0.769213, -0.506977)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.170405, -0.424508, -1.61525),
            glm::vec4(0.0, 0.760829, 1.0, 1.0),
            glm::quat(0.675567, 0.111695, -0.294815, -0.666497)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.56869, -0.53369, 0.255647),
            glm::vec4(0.542505, 0.0, 1.0, 0.6),
            glm::quat(0.396476, -0.223107, -0.849649, 0.266698)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.5608, -0.601516, -0.136756),
            glm::vec4(1.0, 0.49544, 0.0, 0.6),
            glm::quat(0.26977, -0.37916, -0.741526, 0.483322)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.43731, -0.318179, -0.805389),
            glm::vec4(1.0, 0.354469, 0.0, 0.6),
            glm::quat(0.721465, -0.55752, 0.410395, 0.0153453)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.07825, -0.286024, -0.973872),
            glm::vec4(0.0, 1.0, 0.906979, 0.6),
            glm::quat(0.902122, -0.283896, 0.280018, -0.164828)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.608085, -0.597146, -1.447),
            glm::vec4(0.249573, 0.0, 1.0, 1.0),
            glm::quat(0.735446, -0.447211, 0.32983, 0.387727)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.30999, -0.254934, -1.62741),
            glm::vec4(0.899542, 0.0, 1.0, 1.0),
            glm::quat(0.488968, -0.491907, 0.180131, 0.697489)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.594943, -0.212985, 1.55274),
            glm::vec4(1.0, 0.681524, 0.0, 0.6),
            glm::quat(-0.0155867, -0.158303, -0.240136, 0.957618)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.218615, -0.103786, 1.48802),
            glm::vec4(1.0, 0.521102, 0.0, 0.6),
            glm::quat(0.0760844, 0.874867, 0.434311, -0.200479)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.141162, -0.173586, 1.3379),
            glm::vec4(0.5196, 1.0, 0.0, 1.0),
            glm::quat(0.441388, -0.0796967, 0.803048, -0.392351)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.870714, -0.17914, 1.42061),
            glm::vec4(0.0, 0.772298, 1.0, 1.0),
            glm::quat(0.110645, 0.309201, 0.626219, -0.707108)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.24147, -0.220018, 1.1023),
            glm::vec4(0.0, 1.0, 0.0561671, 1.0),
            glm::quat(0.342879, -0.624102, -0.542179, -0.446064)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.61856, -0.285871, 0.329135),
            glm::vec4(0.0, 0.957733, 1.0, 0.6),
            glm::quat(-0.302724, -0.0803454, 0.0408518, 0.948807)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.15803, -0.0778627, 1.2505),
            glm::vec4(0.0627062, 1.0, 0.0, 1.0),
            glm::quat(0.478289, 0.0594995, -0.40906, 0.774835)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.276617, -0.161941, 0.802621),
            glm::vec4(1.0, 0.0, 0.220096, 1.0),
            glm::quat(0.617395, 0.0938506, 0.767903, 0.142623)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.447686, -0.251707, 0.91664),
            glm::vec4(0.733545, 0.0, 1.0, 1.0),
            glm::quat(0.313821, 0.266413, 0.300751, -0.860285)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.838283, -0.106259, 0.612281),
            glm::vec4(0.0, 0.566653, 1.0, 1.0),
            glm::quat(0.14705, 0.158294, -0.289147, 0.932584)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.07433, -0.133945, 0.242639),
            glm::vec4(0.0257075, 0.0, 1.0, 1.0),
            glm::quat(0.63824, 0.686724, 0.332848, 0.101352)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.41794, -0.159695, 0.0103212),
            glm::vec4(1.0, 0.542174, 0.0, 1.0),
            glm::quat(0.739756, -0.587782, 0.25454, -0.206113)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.35149, -0.262967, 0.957075),
            glm::vec4(1.0, 0.0, 0.98525, 1.0),
            glm::quat(-0.181447, -0.974151, 0.0901875, -0.0998663)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.920209, -0.223486, 0.971157),
            glm::vec4(0.999754, 1.0, 0.0, 1.0),
            glm::quat(0.859271, -0.436943, -0.259854, 0.0566618)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.392868, -0.0921243, 0.409552),
            glm::vec4(0.0, 0.766823, 1.0, 1.0),
            glm::quat(0.429984, 0.133953, -0.0264862, -0.892451)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.472213, -0.161542, 0.46969),
            glm::vec4(0.259053, 0.0, 1.0, 1.0),
            glm::quat(0.42993, 0.149256, 0.734298, -0.503676)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.848358, -0.201212, -0.412236),
            glm::vec4(1.0, 0.0, 0.424729, 1.0),
            glm::quat(-0.190775, 0.850914, -0.440819, -0.21267)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.23455, -0.171179, -0.337942),
            glm::vec4(0.856013, 0.0, 1.0, 1.0),
            glm::quat(0.137343, -0.821775, -0.458, 0.309935)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.48648, -0.453045, 0.635533),
            glm::vec4(1.0, 0.963147, 0.0, 1.0),
            glm::quat(0.192595, -0.0700101, 0.966795, 0.152687)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.742016, -0.163334, 0.236744),
            glm::vec4(1.0, 0.0, 0.656743, 0.6),
            glm::quat(-0.595287, 0.519966, -0.207595, 0.576344)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.485952, -0.196387, -0.501868),
            glm::vec4(0.37112, 0.0, 1.0, 1.0),
            glm::quat(0.462423, 0.251733, 0.849882, -0.0222846)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.462961, -0.28988, -0.232114),
            glm::vec4(1.0, 0.0, 0.0336512, 1.0),
            glm::quat(0.57439, 0.0979559, 0.154209, -0.797935)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.0314845, -0.190584, -0.60924),
            glm::vec4(0.603563, 1.0, 0.0, 0.6),
            glm::quat(-0.641659, -0.0754543, -0.686195, -0.334239)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.16116, -0.264501, -0.714752),
            glm::vec4(1.0, 0.0917087, 0.0, 1.0),
            glm::quat(0.339697, -0.81088, -0.30478, 0.366317)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.47918, -0.239205, 0.00682545),
            glm::vec4(1.0, 0.387219, 0.0, 1.0),
            glm::quat(-0.0697679, -0.783481, 0.0357793, 0.616449)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.13325, -0.165649, -0.600385),
            glm::vec4(0.556053, 0.0, 1.0, 1.0),
            glm::quat(0.08525, -0.931375, 0.116279, 0.334294)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.771735, -0.13956, -0.77019),
            glm::vec4(1.0, 0.185926, 0.0, 0.6),
            glm::quat(0.347629, -0.601867, 0.663598, -0.276672)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.395134, -0.169742, -0.887094),
            glm::vec4(0.0, 1.0, 0.104799, 1.0),
            glm::quat(0.515655, 0.788602, 0.157394, -0.295692)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.614493, -0.0782687, -1.2274),
            glm::vec4(0.0, 0.0935028, 1.0, 1.0),
            glm::quat(0.0238722, -0.657509, 0.745183, -0.108692)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-1.16871, -0.187594, -1.19029),
            glm::vec4(0.956935, 1.0, 0.0, 1.0),
            glm::quat(-0.0185914, -0.637235, -0.109707, 0.762595)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(1.60257, -0.300832, -0.391776),
            glm::vec4(0.999892, 1.0, 0.0, 1.0),
            glm::quat(0.0854723, -0.995259, 0.0462091, 0.00424197)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.859313, -0.289579, -1.4106),
            glm::vec4(0.0, 0.538714, 1.0, 1.0),
            glm::quat(0.582991, -0.0975758, -0.792903, 0.148002)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(0.504083, -0.156985, -1.2963),
            glm::vec4(1.0, 0.900847, 0.0, 0.6),
            glm::quat(-0.123163, 0.641072, -0.400994, 0.642698)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.126877, -0.151188, -1.3301),
            glm::vec4(0.331614, 1.0, 0.0, 1.0),
            glm::quat(0.635328, 0.591435, 0.491082, 0.0734989)
        ),
        MarbleInstance::new_from_houdini_data(
            glm::vec3(-0.518474, -0.236542, -1.57651),
            glm::vec4(1.0, 0.191132, 0.0, 1.0),
            glm::quat(0.0827668, 0.296897, 0.696771, -0.647697)
        ),
    ];
}
