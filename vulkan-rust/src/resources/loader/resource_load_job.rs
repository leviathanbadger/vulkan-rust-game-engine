use super::{ResourceLoader};

use anyhow::{anyhow, Result};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::app_data::{AppData};

pub trait ResourceLoadJob : std::fmt::Debug {
    fn needs_transient_command(&self) -> bool {
        false
    }
    fn load(&mut self, _resource_loader: &mut ResourceLoader, device: &Device, app_data: &AppData) -> Result<()>;
    fn write_to_command_buffer(&self, _resource_loader: &mut ResourceLoader, _device: &Device, _app_data: &AppData, _command_buffer: &vk::CommandBuffer) -> Result<()> {
        Err(anyhow!("ResourceLoadJob#write_to_command_buffer not implemented for this job type."))
    }
    fn after_command(&mut self, _resource_loader: &mut ResourceLoader, _device: &Device, _app_data: &AppData) -> Result<()> {
        Ok(())
    }
}
