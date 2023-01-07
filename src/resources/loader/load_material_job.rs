use super::{
    resource_load_job::{ResourceLoadJob},
    MaterialProperties,
    MaterialRef,
    ResourceLoader
};

use anyhow::{Result};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    resources::{
        CanBeVertexBufferType,
        CanBeInstVertexBufferType,
        shader_source::{create_shader_sources, ShaderSources, DepthBufferUsageMode, BlendStateDescriptor, create_pipeline},
        material::{Material}
    },
    app_data::{AppData}
};

#[derive(Debug)]
pub struct LoadMaterialJob {
    mat_ref: MaterialRef,
    binding_descriptions: Vec<vk::VertexInputBindingDescription>,
    attribute_descriptions: Vec<vk::VertexInputAttributeDescription>,

    depth_and_motion: Option<ShaderSources>,
    base_render: Option<ShaderSources>
}

impl LoadMaterialJob {
    pub(super) fn create_for<TVert, TInstVert>(mat_ref: MaterialRef, mat_props: &MaterialProperties<TVert, TInstVert>) -> Self where TVert : CanBeVertexBufferType, TInstVert : CanBeInstVertexBufferType {
        let binding_descriptions = [TVert::binding_descriptions(), TInstVert::binding_descriptions()].concat();
        let attribute_descriptions = [TVert::attribute_descriptions(), TInstVert::attribute_descriptions()].concat();

        let is_instanced = TInstVert::binding_descriptions().len() > 0;
        let suffix = if is_instanced { "_instanced" } else { "" };

        let depth_and_motion: Option<ShaderSources> = if !mat_props.is_opaque { None } else { Some(create_shader_sources("depth_and_motion_", mat_props.shader_name, suffix, mat_props.shader_entry)) };
        let base_render: Option<ShaderSources> = if !mat_props.is_opaque { None } else { Some(create_shader_sources("", mat_props.shader_name, suffix, mat_props.shader_entry)) };

        Self {
            mat_ref,
            binding_descriptions,
            attribute_descriptions,
            depth_and_motion,
            base_render
        }
    }
}

impl ResourceLoadJob for LoadMaterialJob {
    fn load(&mut self, resource_loader: &mut ResourceLoader, device: &Device, app_data: &AppData) -> Result<()> {
        let pipeline_info = app_data.pipeline.as_ref().unwrap();
        let layout = pipeline_info.base_render_layout;
        let render_pass = pipeline_info.base_render_pass;

        let mut depth_and_motion_pipeline = None;
        if let Some(depth_and_motion_sources) = self.depth_and_motion.take() {
            let blend_state = &[
                BlendStateDescriptor {
                    components: vk::ColorComponentFlags::R | vk::ColorComponentFlags::G,
                    ..Default::default()
                }
            ][..];
            depth_and_motion_pipeline = Some(create_pipeline(depth_and_motion_sources.vertex, depth_and_motion_sources.fragment, device, None, layout, render_pass, 0, blend_state, DepthBufferUsageMode::WriteIfLess, &self.binding_descriptions[..], &self.attribute_descriptions[..])?);
        }

        let mut base_render_pipeline = None;
        if let Some(base_render_sources) = self.base_render.take() {
            let blend_state = &[
                BlendStateDescriptor::default()
            ][..];
            base_render_pipeline = Some(create_pipeline(base_render_sources.vertex, base_render_sources.fragment, device, None, layout, render_pass, 1, blend_state, DepthBufferUsageMode::WriteIfEqual, &self.binding_descriptions[..], &self.attribute_descriptions[..])?);
        }

        let material = Material {
            is_loaded: true,
            depth_motion: depth_and_motion_pipeline,
            base_render: base_render_pipeline
        };

        resource_loader.finish_loading_material(self.mat_ref, material)
    }
}
