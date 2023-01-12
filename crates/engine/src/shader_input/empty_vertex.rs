use crate::{
    vertex_type
};

vertex_type! {
    use crate as engine;

    pub struct EmptyVertex {
        binding 1;
        location 0;
        instance true;
    }
}
