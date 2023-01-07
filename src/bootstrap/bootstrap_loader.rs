use anyhow::{Result};
use winit::window::Window;
use std::fmt::{Debug};
use vulkanalia::{
    prelude::v1_0::*,
    vk::{InstanceCreateInfoBuilder, PhysicalDeviceFeaturesBuilder, PhysicalDeviceProperties, PhysicalDeviceFeatures}
};

use crate::{
    app_data::{AppData}
};

pub trait IndirectDependency {
    fn dependency_name(&self) -> &'static str;

    fn depends_on(&self) -> &'static [&'static str];
}

pub trait BootstrapLoader : Debug + IndirectDependency {
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

    fn check_physical_device_compatibility(&self, _inst: &Instance, _app_data: &AppData, _physical_device: vk::PhysicalDevice, _properties: PhysicalDeviceProperties, _features: PhysicalDeviceFeatures) -> Result<()> {
        Ok(())
    }

    fn after_create_logical_device(&self, _inst: &Instance, _device: &Device, _window: &Window, _app_data: &mut AppData) -> Result<()> {
        Ok(())
    }
    fn before_destroy_logical_device(&self, _inst: &Instance, _device: &Device, _app_data: &mut AppData) -> () { }

    fn recreate_swapchain(&self, inst: &Instance, device: &Device, window: &Window, app_data: &mut AppData, next: &dyn Fn(&Instance, &Device, &Window, &mut AppData) -> Result<()>) -> Result<()> {
        trace!("Default BootstrapLoader::recreate_swapchain");

        self.before_destroy_logical_device(inst, device, app_data);
        next(inst, device, window, app_data)?;
        self.after_create_logical_device(inst, device, window, app_data)?;

        Ok(())
    }
}

#[macro_export]
macro_rules! replace_expr {
    ( $replace:tt , $replacewith:expr ) => {
        $replacewith
    };
}

#[macro_export]
macro_rules! bootstrap_loader {
    (
        pub struct $name:ident {
            depends_on ( $( $dependty:ty ),* ) ;
        }
    ) => {
        #[derive(Debug, Default)]
        pub struct $name { }

        #[allow(non_upper_case_globals)]
        const __bootstrap_loader_depends_on: [&str; 0 $( + crate::replace_expr!($dependty, 1) )* ] = [ $( <$dependty>::dependency_name() ),* ];

        impl $name {
            pub fn new() -> Self {
                Self::default()
            }

            #[allow(unused)]
            pub(crate) const fn dependency_name() -> &'static str {
                stringify!($name)
            }

            #[allow(unused)]
            pub(crate) fn depends_on() -> &'static [&'static str] {
                &__bootstrap_loader_depends_on[..]
            }
        }

        impl crate::bootstrap::bootstrap_loader::IndirectDependency for $name {
            fn dependency_name(&self) -> &'static str {
                Self::dependency_name()
            }

            fn depends_on(&self) -> &'static [&'static str] {
                Self::depends_on()
            }
        }
    };
}
