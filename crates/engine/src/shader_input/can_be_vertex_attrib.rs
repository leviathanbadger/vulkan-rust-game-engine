use std::mem::size_of;
use nalgebra_glm as glm;
use vulkanalia::{
    prelude::v1_0::*
};



pub trait CanBeVertexAttrib : Sized {
    fn vertex_struct_size() -> usize {
        size_of::<Self>()
    }
    fn vertex_format() -> vk::Format;
    fn vertex_format_repeat() -> usize {
        (Self::vertex_struct_size() + 15) / 16
    }
    fn vertex_format_offset() -> Option<usize> {
        None
    }
}



impl CanBeVertexAttrib for f32 {
    fn vertex_format() -> vk::Format {
        vk::Format::R32_SFLOAT
    }
}

impl CanBeVertexAttrib for i32 {
    fn vertex_format() -> vk::Format {
        vk::Format::R32_SINT
    }
}

impl CanBeVertexAttrib for u32 {
    fn vertex_format() -> vk::Format {
        vk::Format::R32_UINT
    }
}



impl CanBeVertexAttrib for glm::Vec1 {
    fn vertex_format() -> vk::Format {
        vk::Format::R32_SFLOAT
    }
}

impl CanBeVertexAttrib for glm::IVec1 {
    fn vertex_format() -> vk::Format {
        vk::Format::R32_SINT
    }
}

impl CanBeVertexAttrib for glm::UVec1 {
    fn vertex_format() -> vk::Format {
        vk::Format::R32_UINT
    }
}



impl CanBeVertexAttrib for glm::Vec2 {
    fn vertex_format() -> vk::Format {
        vk::Format::R32G32_SFLOAT
    }
}

impl CanBeVertexAttrib for glm::UVec2 {
    fn vertex_format() -> vk::Format {
        vk::Format::R32G32_UINT
    }
}

impl CanBeVertexAttrib for glm::IVec2 {
    fn vertex_format() -> vk::Format {
        vk::Format::R32G32_SINT
    }
}



impl CanBeVertexAttrib for glm::Vec3 {
    fn vertex_format() -> vk::Format {
        vk::Format::R32G32B32_SFLOAT
    }
}

impl CanBeVertexAttrib for glm::IVec3 {
    fn vertex_format() -> vk::Format {
        vk::Format::R32G32B32_SINT
    }
}

impl CanBeVertexAttrib for glm::UVec3 {
    fn vertex_format() -> vk::Format {
        vk::Format::R32G32B32_UINT
    }
}



impl CanBeVertexAttrib for glm::Vec4 {
    fn vertex_format() -> vk::Format {
        vk::Format::R32G32B32A32_SFLOAT
    }
}

impl CanBeVertexAttrib for glm::IVec4 {
    fn vertex_format() -> vk::Format {
        vk::Format::R32G32B32A32_SINT
    }
}

impl CanBeVertexAttrib for glm::UVec4 {
    fn vertex_format() -> vk::Format {
        vk::Format::R32G32B32A32_UINT
    }
}



impl CanBeVertexAttrib for glm::Mat4 {
    fn vertex_format() -> vk::Format {
        vk::Format::R32G32B32A32_SFLOAT
    }
    fn vertex_format_offset() -> Option<usize> {
        Some(size_of::<glm::Vec4>())
    }
}
