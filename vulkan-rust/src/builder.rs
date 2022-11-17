use winit::dpi::{LogicalSize};
use anyhow::{Result};
use crate::{
    app::App,
    bootstrap::{
        BootstrapLoader,
        bootstrap_framebuffer_loader::BootstrapFramebufferLoader,
        bootstrap_pipeline_loader::BootstrapPipelineLoader,
        bootstrap_validation_loader::BootstrapValidationLoader,
        bootstrap_swapchain_loader::BootstrapSwapchainLoader
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
    default_size: LogicalSize<i32>,
    add_validation: bool
}

impl<'a> Default for AppBuilder {
    fn default() -> Self {
        Self {
            bootstrap_loaders: vec![],
            initial_title: "",
            default_size: LogicalSize::new(300, 300),
            add_validation: false
        }
    }
}

impl AppBuilder {
    pub fn add_bootstrap_loader(mut self, loader: Box<dyn BootstrapLoader>) -> Self {
        self.bootstrap_loaders.push(loader);

        self
    }

    pub fn add_default_bootstrap_loaders(self) -> Self {
        self.add_bootstrap_loader(Box::new(BootstrapSwapchainLoader::new()))
            .add_bootstrap_loader(Box::new(BootstrapPipelineLoader::new()))
            .add_bootstrap_loader(Box::new(BootstrapFramebufferLoader::new()))
    }

    pub fn add_validation(mut self) -> Self {
        self.add_validation = true;

        self
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
        if self.add_validation {
            let validation_loader = Box::new(BootstrapValidationLoader::new());
            bootstrap_loaders.insert(0, validation_loader);
        }
        App::create(self.initial_title, self.default_size, bootstrap_loaders)
    }
}
