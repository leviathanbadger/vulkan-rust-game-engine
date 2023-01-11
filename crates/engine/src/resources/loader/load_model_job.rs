use super::{
    resource_load_job::{ResourceLoadJob},
    ResourceLoader,
    ModelProperties,
    ModelRef
};

use anyhow::{Result, anyhow};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    resources::{
        CanBeVertexBufferType,
        Model
    },
    app_data::{AppData}
};

#[derive(Debug)]
pub struct LoadModelJob<TVert> where TVert : CanBeVertexBufferType {
    model_ref: ModelRef,
    obj_path: Option<String>,
    model: Option<Model<TVert>>
}

impl<TVert> LoadModelJob<TVert> where TVert : CanBeVertexBufferType {
    pub(super) fn create_for(model_ref: ModelRef, model_props: &ModelProperties<TVert>) -> Self {
        Self {
            model_ref,
            obj_path: model_props.obj_path.clone(),
            model: None
        }
    }
}

impl<TVert> ResourceLoadJob for LoadModelJob<TVert> where TVert : CanBeVertexBufferType {
    fn needs_transient_command(&self) -> bool {
        true
    }

    fn load(&mut self, _resource_loader: &mut ResourceLoader, device: &Device, app_data: &AppData) -> Result<()> {
        if let Some(path) = self.obj_path.clone() {
            self.model = Some(Model::<TVert>::new_and_create_from_obj_file(path, device, &app_data.memory_properties)?);
        } else {
            return Err(anyhow!("Can not load model without OBJ path"));
        }

        Ok(())
    }

    fn write_to_command_buffer(&self, _resource_loader: &mut ResourceLoader, device: &vulkanalia::Device, _app_data: &AppData, command_buffer: &vulkanalia::vk::CommandBuffer) -> anyhow::Result<()> {
        let model = self.model.as_ref().unwrap();
        model.write_submit_to_command_buffer(device, command_buffer)?;

        Ok(())
    }

    fn after_command(&mut self, resource_loader: &mut ResourceLoader, _device: &Device, _app_data: &AppData) -> Result<()> {
        let model = self.model.unwrap();
        resource_loader.finish_loading_model(self.model_ref, model.reinterpret_readonly())
    }
}
