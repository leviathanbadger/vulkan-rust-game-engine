use super::{BootstrapLoader};

use anyhow::{anyhow, Result};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    app_data::{AppData},
    app::{GraphicsCardSuitabilityError},
    bootstrap_loader
};

bootstrap_loader! {
    pub struct BootstrapTextureSamplingLoader {
        depends_on();
    }
}

impl BootstrapLoader for BootstrapTextureSamplingLoader {
    fn add_required_device_features(&self, features: &mut vk::PhysicalDeviceFeaturesBuilder) -> Result<()> {
        *features = features.sampler_anisotropy(true);

        Ok(())
    }

    fn check_physical_device_compatibility(&self, _inst: &Instance, _app_data: &AppData, _physical_device: vk::PhysicalDevice, _properties: vk::PhysicalDeviceProperties, features: vk::PhysicalDeviceFeatures) -> Result<()> {
        if features.sampler_anisotropy != vk::TRUE {
            return Err(anyhow!(GraphicsCardSuitabilityError("Does not support sampler anisotropy.")));
        }

        Ok(())
    }
}
