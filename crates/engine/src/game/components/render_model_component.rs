use super::{GameComponent};

use std::marker::{PhantomData};
use nalgebra_glm as glm;
use anyhow::{Result};

use crate::{
    game::{
        can_be_enabled::{CanBeEnabled}
    },
    resources::{CanBeVertexBufferType, CanBeInstVertexBufferType, SingleFrameRenderInfo, MaterialRef, ModelRef, MaterialProperties, ModelProperties, ResourceLoader},
    shader_input::empty_vertex::{EmptyVertex}
};

#[derive(Debug)]
pub struct RenderModelComponent<TVert, TInstVert = EmptyVertex> where TVert : CanBeVertexBufferType + 'static, TInstVert : CanBeInstVertexBufferType {
    enabled: bool,
    is_loaded: bool,
    path: &'static str,

    phantom_vert: PhantomData<TVert>,
    phantom_inst_vert: PhantomData<TInstVert>,

    material: Option<MaterialRef>,
    model: Option<ModelRef>
}

impl<TVert, TInstVert> RenderModelComponent<TVert, TInstVert> where TVert : CanBeVertexBufferType + 'static, TInstVert : CanBeInstVertexBufferType {
    pub fn new(path: &'static str) -> Result<Self> {
        Ok(Self {
            enabled: true,
            is_loaded: false,
            path: path,

            phantom_vert: Default::default(),
            phantom_inst_vert: Default::default(),

            model: None,
            material: None
        })
    }
}

impl<TVert, TInstVert> CanBeEnabled for RenderModelComponent<TVert, TInstVert> where TVert : CanBeVertexBufferType + 'static, TInstVert : CanBeInstVertexBufferType {
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    fn set_enabled(&mut self, enabled: bool) -> () {
        self.enabled = enabled;
    }
}

impl<TVert, TInstVert> GameComponent for RenderModelComponent<TVert, TInstVert> where TVert : CanBeVertexBufferType + 'static, TInstVert : CanBeInstVertexBufferType {
    fn load_and_unload(&mut self, resource_loader: &mut ResourceLoader) -> Result<()> {
        if self.is_loaded {
            return Ok(());
        }

        let mat_props = MaterialProperties::<TVert, TInstVert> {
            ..Default::default()
        };

        self.material = Some(resource_loader.get_or_load_material(&mat_props)?);

        let model_props = ModelProperties::<TVert> {
            obj_path: Some(self.path.to_owned()),
            ..Default::default()
        };

        self.model = Some(resource_loader.get_or_load_model(&model_props)?);

        // let command_pools_info = &app_data.command_pools.as_ref().unwrap();
        // let model = Model::<TVert>::new_and_create_from_obj_file(self.path, device, &app_data.memory_properties, command_pools_info)?;
        // self.model = Some(model);

        self.is_loaded = true;
        Ok(())
    }

    fn unload(&mut self, resource_loader: &mut ResourceLoader) -> () {
        if self.is_loaded {
            if let Some(model) = self.model.take() {
                resource_loader.unload_model(model);
            }

            if let Some(material) = self.material.take() {
                resource_loader.unload_material(material);
            }

            self.is_loaded = false;
        }
    }

    fn create_frame_render_info(&self, frame_info: &mut SingleFrameRenderInfo, viewmodel: &glm::Mat4, previous_viewmodel: Option<&glm::Mat4>) -> Result<()> {
        if let (Some(material), Some(model)) = (self.material, self.model) {
            model.create_frame_render_info(frame_info, material, false, true, viewmodel, previous_viewmodel)?;
        }

        Ok(())
    }
}
