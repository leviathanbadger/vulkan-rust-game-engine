use super::{GameComponent};

use nalgebra_glm as glm;
use anyhow::{Result};

use crate::{
    game::{
        can_be_enabled::{CanBeEnabled}
    },
    resources::{SingleFrameRenderInfo, ModelRef, MaterialRef, MaterialProperties, ModelProperties, ResourceLoader, Buffer},
    shader_input::marble::{self, MARBLE_INSTANCES}
};

#[derive(Debug)]
pub struct RenderMarbleComponent {
    enabled: bool,
    is_loaded: bool,
    path: &'static str,
    material: Option<MaterialRef>,
    model: Option<ModelRef>,
    inst_buffer: Option<Buffer<marble::MarbleInstance>>
}

impl RenderMarbleComponent {
    pub fn new(path: &'static str) -> Result<Self> {
        Ok(Self {
            enabled: true,
            is_loaded: false,
            path: path,
            material: None,
            model: None,
            inst_buffer: None
        })
    }
}

impl CanBeEnabled for RenderMarbleComponent {
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    fn set_enabled(&mut self, enabled: bool) -> () {
        self.enabled = enabled;
    }
}

impl GameComponent for RenderMarbleComponent {
    fn load_and_unload(&mut self, resource_loader: &mut ResourceLoader) -> Result<()> {
        if self.is_loaded {
            return Ok(());
        }

        let mat_props = MaterialProperties::<marble::Vertex, marble::MarbleInstance> {
            shader_name: "marble",
            ..Default::default()
        };

        self.material = Some(resource_loader.get_or_load_material(&mat_props)?);

        let model_props = ModelProperties::<marble::Vertex> {
            obj_path: Some(self.path.to_owned()),
            ..Default::default()
        };

        self.model = Some(resource_loader.get_or_load_model(&model_props)?);

        self.inst_buffer = Some(resource_loader.create_inst_buffer(&*MARBLE_INSTANCES)?);

        self.is_loaded = true;
        Ok(())
    }

    fn unload(&mut self, resource_loader: &mut ResourceLoader) -> () {
        if self.is_loaded {
            if let Some(model) = self.model.take() {
                resource_loader.unload_model(model);
                // model.destroy(device);
            }

            if let Some(material) = self.material.take() {
                resource_loader.unload_material(material);
            }

            //This should be cleaned up by the ResourceLoader later
            self.inst_buffer = None;

            self.is_loaded = false;
        }
    }

    fn create_frame_render_info(&self, frame_info: &mut SingleFrameRenderInfo, viewmodel: &glm::Mat4, previous_viewmodel: Option<&glm::Mat4>) -> Result<()> {
        if let (Some(material), Some(model), Some(inst_buffer)) = (self.material, self.model, self.inst_buffer) {
            model.create_frame_render_info_instanced(frame_info, material, false, true, viewmodel, previous_viewmodel, inst_buffer)?;
        }

        Ok(())
    }
}
