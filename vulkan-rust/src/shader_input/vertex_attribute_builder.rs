use vulkanalia::{
    prelude::v1_0::*
};

pub use crate::shader_input::can_be_vertex_attrib::{CanBeVertexAttrib};

pub struct VertexAttributeBuilder {
    attrs: Vec<vk::VertexInputAttributeDescription>,
    offset: usize,
    location: usize
}

impl VertexAttributeBuilder {
    pub fn new() -> Self {
        Self {
            attrs: vec![],
            offset: 0,
            location: 0
        }
    }

    pub fn add_attr<T : CanBeVertexAttrib>(&mut self) -> &mut Self {
        let offset_add = T::vertex_struct_size();
        let location_add = T::vertex_location_size();

        let attr = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(self.location as u32)
            .format(T::vertex_format())
            .offset(self.offset as u32)
            .build();
        self.attrs.push(attr);

        self.offset += offset_add;
        self.location += location_add;

        self
    }

    pub fn build_binding(&self) -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(self.offset as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()
    }

    pub fn build_attributes(&self) -> &[vk::VertexInputAttributeDescription] {
        &self.attrs[..]
    }
}

#[macro_export]
macro_rules! vertex_type {
    ( pub struct $struct_name:ident { $( $name:ident : $type:ty ),* } ) => {
        mod __vertex_type {
            #![allow(unused_imports)]
            use lazy_static::{lazy_static};
            use crate::{shader_input::vertex_attribute_builder::{VertexAttributeBuilder}};
            use nalgebra_glm as glm;
            use nalgebra_glm::*;
            use vulkanalia::{
                prelude::v1_0::*
            };

            #[repr(C)]
            #[derive(Copy, Clone, Debug, Default)]
            pub struct $struct_name {
                $( $name : $type ),*
            }

            lazy_static! {
                static ref VERTEX_DESCRIPTIONS_BUILDER: VertexAttributeBuilder = {
                    let mut builder = VertexAttributeBuilder::new();
                    builder
                        $( .add_attr::< $type >() )*
                        ;
                    builder
                };
            }

            impl $struct_name {
                pub fn new( $( $name : $type ),* ) -> Self {
                    Self { $( $name ),* }
                }

                pub fn binding_description() -> vk::VertexInputBindingDescription {
                    VERTEX_DESCRIPTIONS_BUILDER.build_binding()
                }

                pub fn attribute_descriptions() -> &'static [vk::VertexInputAttributeDescription] {
                    VERTEX_DESCRIPTIONS_BUILDER.build_attributes()
                }
            }
        }

        pub type $struct_name = __vertex_type::$struct_name;
    };
}