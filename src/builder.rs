use std::collections::{HashSet};
use winit::dpi::{LogicalSize};
use anyhow::{anyhow, Result};
use crate::{
    app::App,
    bootstrap::{
        BootstrapLoader,
        BootstrapCommandBufferLoader,
        BootstrapRenderImagesLoader,
        BootstrapDescriptorSetLoader,
        BootstrapFramebufferLoader,
        BootstrapPipelineLoader,
        BootstrapSwapchainLoader,
        BootstrapSyncObjectsLoader,
        BootstrapTextureSamplingLoader,
        BootstrapUniformLoader,
        BootstrapValidationLoader
    }
};

pub trait HasHeapBuilder {
    type Builder: Default + core::fmt::Debug;

    #[inline]
    fn builder() -> Self::Builder {
        Default::default()
    }
}

impl HasHeapBuilder for App {
    type Builder = AppBuilder;
}

#[derive(Debug)]
pub struct AppBuilder {
    bootstrap_loaders: Vec<Box<dyn BootstrapLoader>>,
    initial_title: &'static str,
    default_size: LogicalSize<i32>
}

impl<'a> Default for AppBuilder {
    fn default() -> Self {
        Self {
            bootstrap_loaders: vec![],
            initial_title: "",
            default_size: LogicalSize::new(300, 300)
        }
    }
}

impl AppBuilder {
    pub fn add_bootstrap_loader(mut self, loader: Box<dyn BootstrapLoader>) -> Self {
        self.bootstrap_loaders.push(loader);

        self
    }

    pub fn add_default_bootstrap_loaders(self) -> Self {
        self.add_bootstrap_loader(Box::new(BootstrapTextureSamplingLoader::new()))
            .add_bootstrap_loader(Box::new(BootstrapSwapchainLoader::new()))
            .add_bootstrap_loader(Box::new(BootstrapCommandBufferLoader::new()))
            .add_bootstrap_loader(Box::new(BootstrapUniformLoader::new()))
            .add_bootstrap_loader(Box::new(BootstrapRenderImagesLoader::new()))
            .add_bootstrap_loader(Box::new(BootstrapPipelineLoader::new()))
            .add_bootstrap_loader(Box::new(BootstrapFramebufferLoader::new()))
            .add_bootstrap_loader(Box::new(BootstrapSyncObjectsLoader::new()))
            .add_bootstrap_loader(Box::new(BootstrapDescriptorSetLoader::new()))
    }

    pub fn add_validation(self) -> Self {
        self.add_bootstrap_loader(Box::new(BootstrapValidationLoader::new()))
    }

    pub fn initial_title(mut self, title: &'static str) -> Self {
        self.initial_title = title;

        self
    }

    pub fn default_size(mut self, size: LogicalSize<i32>) -> Self {
        self.default_size = size;

        self
    }

    pub fn build(self) -> Result<App> {
        let mut bootstrap_loaders = self.bootstrap_loaders;
        let mut ordered_bootstrap_loaders = vec![];
        let mut satisfied_dependencies = HashSet::<&str>::new();

        'outer: loop {
            if bootstrap_loaders.len() == 0 {
                break;
            }

            for q in 0..(bootstrap_loaders.len()) {
                let loader = &bootstrap_loaders[q];
                let name = loader.dependency_name();
                if satisfied_dependencies.contains(name) {
                    return Err(anyhow!("Multiple bootstrap loaders with the same type ({}) detected. Fix your startup script!", name));
                }

                let dependencies = loader.depends_on();
                let mut has_unmet_dependencies = false;
                for dependency in dependencies {
                    if !satisfied_dependencies.contains(dependency) {
                        has_unmet_dependencies = true;
                        break;
                    }
                }

                if has_unmet_dependencies {
                    continue;
                }

                trace!("Adding bootstrap loader {}...", name);
                ordered_bootstrap_loaders.push(bootstrap_loaders.remove(q));
                satisfied_dependencies.insert(name);
                continue 'outer;
            }

            return Err(anyhow!("Could not resolve dependencies for the following bootstrap loaders: {:?}", bootstrap_loaders));
        }

        App::create(self.initial_title, self.default_size, ordered_bootstrap_loaders)
    }
}
