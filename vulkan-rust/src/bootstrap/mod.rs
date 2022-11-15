pub mod bootstrap_validation_loader;
pub mod queue_family_indices;
use anyhow::{Result};
use std::fmt::{Debug};
use vulkanalia::{
    prelude::v1_0::*,
    vk::{InstanceCreateInfoBuilder, PhysicalDeviceFeaturesBuilder}
};

use crate::app::AppData;

pub trait BootstrapLoader : Debug {
    fn add_required_instance_layers(&self, _required_layers: &mut Vec<*const i8>) -> Result<()> {
        Ok(())
    }
    fn add_required_instance_extensions(&self, _required_extensions: &mut Vec<*const i8>) -> Result<()> {
        Ok(())
    }
    fn instance_create(&self, inst_info: InstanceCreateInfoBuilder, app_data: &mut AppData, next: &dyn Fn(InstanceCreateInfoBuilder, &mut AppData) -> Result<Instance>) -> Result<Instance> {
        trace!("Default BootstrapLoader::instance_create");
        next(inst_info, app_data)
    }
    fn before_destroy_instance(&self, _inst: &Instance, _app_data: &mut AppData) -> () { }

    fn add_required_device_layers(&self, _required_layers: &mut Vec<*const i8>) -> Result<()> {
        Ok(())
    }
    fn add_required_device_extensions(&self, _required_extensions: &mut Vec<*const i8>) -> Result<()> {
        Ok(())
    }
    fn add_required_device_features(&self, _features: &mut PhysicalDeviceFeaturesBuilder) -> Result<()> {
        Ok(())
    }
}
