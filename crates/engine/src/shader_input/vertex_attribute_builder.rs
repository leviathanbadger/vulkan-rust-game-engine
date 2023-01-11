use vulkanalia::{
    prelude::v1_0::*
};

pub use crate::shader_input::can_be_vertex_attrib::{CanBeVertexAttrib};

pub struct VertexAttributeBuilder {
    attrs: Vec<vk::VertexInputAttributeDescription>,
    bindings: Vec<vk::VertexInputBindingDescription>,
    offset: usize,
    location: usize,
    binding: u32,
    instance: bool
}

impl VertexAttributeBuilder {
    pub fn new() -> Self {
        Self {
            attrs: vec![],
            bindings: vec![],
            offset: 0,
            location: 0,
            binding: 0,
            instance: false
        }
    }

    pub fn set_binding(&mut self, binding: u32) -> &mut Self {
        self.binding = binding;

        self
    }

    pub fn set_location(&mut self, location: usize) -> &mut Self {
        if self.attrs.len() > 0 {
            panic!("Can't set location after the first attribute has been added.");
        }
        self.location = location;

        self
    }

    pub fn set_instance(&mut self, instance: bool) -> &mut Self {
        self.instance = instance;

        self
    }

    pub fn add_attr<T : CanBeVertexAttrib>(&mut self) -> &mut Self {
        if self.bindings.len() > 0 {
            panic!("Bindings were already finalized for this VertexAttributeBuilder. Can't add more attributes!");
        }

        let offset_add = T::vertex_format_offset().unwrap_or(T::vertex_struct_size());

        let repeat = T::vertex_format_repeat();

        for _q in 0..repeat {
            let attr = vk::VertexInputAttributeDescription::builder()
                .binding(self.binding)
                .location(self.location as u32)
                .format(T::vertex_format())
                .offset(self.offset as u32)
                .build();
            self.attrs.push(attr);

            self.offset += offset_add;
            self.location += 1;
        }

        self
    }

    pub fn finalize_bindings(&mut self) -> &mut Self {
        if self.bindings.len() == 0 && self.attrs.len() > 0 {
            let normal_binding = vk::VertexInputBindingDescription::builder()
                .binding(self.binding)
                .stride(self.offset as u32)
                .input_rate(if self.instance { vk::VertexInputRate::INSTANCE } else { vk::VertexInputRate::VERTEX })
                .build();
            self.bindings.push(normal_binding);
        }

        self
    }

    pub fn build_bindings(&self) -> &[vk::VertexInputBindingDescription] {
        &self.bindings[..]
    }

    pub fn build_attributes(&self) -> &[vk::VertexInputAttributeDescription] {
        &self.attrs[..]
    }
}

pub trait HasVertexAttributeBindings {
    fn binding_descriptions() -> &'static [vk::VertexInputBindingDescription];

    fn attribute_descriptions() -> &'static [vk::VertexInputAttributeDescription];
}

impl HasVertexAttributeBindings for () {
    fn binding_descriptions() -> &'static [vk::VertexInputBindingDescription] {
        &[]
    }

    fn attribute_descriptions() -> &'static [vk::VertexInputAttributeDescription] {
        &[]
    }
}

#[macro_export]
macro_rules! vertex_type {
    ( pub struct $struct_name:ident { binding $binding:expr ; location $location:expr ; instance $instance:expr ;  $( $name:ident : $type:ty ),* } ) => {
        mod __vertex_type {
            #![allow(unused_imports, unused)]
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
                $( pub(super) $name : $type ),*
            }

            lazy_static! {
                static ref VERTEX_DESCRIPTIONS_BUILDER: VertexAttributeBuilder = {
                    let mut builder = VertexAttributeBuilder::new();
                    builder
                        .set_binding( $binding )
                        .set_location( $location )
                        .set_instance( $instance )
                        $( .add_attr::< $type >() )*
                        .finalize_bindings()
                        ;
                    builder
                };
            }

            impl $struct_name {
                pub fn new( $( $name : $type ),* ) -> Self {
                    Self { $( $name ),* }
                }
            }

            impl crate::shader_input::vertex_attribute_builder::HasVertexAttributeBindings for $struct_name {
                fn binding_descriptions() -> &'static [vk::VertexInputBindingDescription] {
                    VERTEX_DESCRIPTIONS_BUILDER.build_bindings()
                }

                fn attribute_descriptions() -> &'static [vk::VertexInputAttributeDescription] {
                    VERTEX_DESCRIPTIONS_BUILDER.build_attributes()
                }
            }

            impl ::std::cmp::PartialEq for $struct_name {
                fn eq(&self, other: &Self) -> bool {
                    true $( && self.$name == other.$name )*
                }
            }

            impl ::std::cmp::Eq for $struct_name { }

            impl ::core::hash::Hash for $struct_name {
                fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                    let (_, bytes, _) = unsafe { crate::util::any_as_u8_slice(self).align_to::<u8>() };
                    bytes.hash(state);
                }
            }
        }

        pub type $struct_name = __vertex_type::$struct_name;
    };
    ( pub struct $struct_name:ident { $( $name:ident : $type:ty ),* } ) => {
        crate::vertex_type! {
            pub struct $struct_name {
                binding 0;
                location 0;
                instance false;

                $( $name : $type ),*
            }
        }
    }
}

vertex_type! {
    pub struct EmptyVertex {
        binding 1;
        location 0;
        instance true;
    }
}
