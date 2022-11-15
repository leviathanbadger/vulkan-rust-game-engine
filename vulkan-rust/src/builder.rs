use winit::dpi::{LogicalSize};
use anyhow::{Result};
use crate::{
    app::App,
    bootstrap::BootstrapLoader
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
    boostrap_loaders: Vec<Box<dyn BootstrapLoader>>,
    initial_title: &'static str,
    default_size: LogicalSize<i32>
}

impl<'a> Default for AppBuilder {
    fn default() -> Self {
        Self {
            boostrap_loaders: vec![],
            initial_title: "",
            default_size: LogicalSize::new(300, 300)
        }
    }
}

impl AppBuilder {
    pub fn add_bootstrap_loader(mut self, loader: Box<dyn BootstrapLoader>) -> Self {
        self.boostrap_loaders.push(loader);

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
        App::create(self.initial_title, self.default_size, self.boostrap_loaders)
    }
}
